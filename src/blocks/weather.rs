use chrono::NaiveDateTime;
use chrono::{TimeZone, Utc};
use crossbeam_channel::Sender;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::time::Duration;
use uuid::Uuid;

use crate::blocks::Update;
use crate::blocks::{Block, ConfigBlock};
use crate::config::Config;
use crate::de::deserialize_duration;
use crate::errors::*;
use crate::input::{I3BarEvent, MouseButton};
use crate::scheduler::Task;
use crate::util::FormatTemplate;
use crate::widget::I3BarWidget;
use crate::widgets::button::ButtonWidget;

const OPENWEATHERMAP_API_KEY_ENV: &str = "OPENWEATHERMAP_API_KEY";
const OPENWEATHERMAP_CITY_ID_ENV: &str = "OPENWEATHERMAP_CITY_ID";
const OPENWEATHERMAP_PLACE_ENV: &str = "OPENWEATHERMAP_PLACE";
const OPENWEATHERMAP_LAT_ENV: &str = "OPENWEATHERMAP_LAT";
const OPENWEATHERMAP_LON_ENV: &str = "OPENWEATHERMAP_LON";

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum WeatherService {
    OpenWeatherMap {
        #[serde(default = "WeatherService::getenv_openweathermap_api_key")]
        api_key: Option<String>,
        #[serde(default = "WeatherService::getenv_openweathermap_city_id")]
        city_id: Option<String>,
        #[serde(default = "WeatherService::getenv_openweathermap_place")]
        place: Option<String>,
        #[serde(default = "WeatherService::getenv_openweathermap_lat")]
        lat: Option<String>,
        #[serde(default = "WeatherService::getenv_openweathermap_lon")]
        lon: Option<String>,
        units: OpenWeatherMapUnits,
    },
}

impl WeatherService {
    fn getenv_openweathermap_api_key() -> Option<String> {
        env::var(OPENWEATHERMAP_API_KEY_ENV).ok()
    }
    fn getenv_openweathermap_city_id() -> Option<String> {
        env::var(OPENWEATHERMAP_CITY_ID_ENV).ok()
    }
    fn getenv_openweathermap_place() -> Option<String> {
        env::var(OPENWEATHERMAP_PLACE_ENV).ok()
    }
    fn getenv_openweathermap_lat() -> Option<String> {
        env::var(OPENWEATHERMAP_LAT_ENV).ok()
    }
    fn getenv_openweathermap_lon() -> Option<String> {
        env::var(OPENWEATHERMAP_LON_ENV).ok()
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OpenWeatherMapUnits {
    Metric,
    Imperial,
}

pub struct Weather {
    id: String,
    weather: ButtonWidget,
    format: String,
    weather_keys: HashMap<String, String>,
    service: WeatherService,
    update_interval: Duration,
    autolocate: bool,
}

fn malformed_json_error(info: String) -> Error {
    BlockError(
        "weather".to_string(),
        format!("Malformed JSON for {}", info),
    )
}

fn get_weather_icon(desc: String) -> String {
    let icon = match desc.as_str() {
        "Clear" => "weather_sun",
        "Rain" | "Drizzle" => "weather_rain",
        "Clouds" | "Fog" | "Mist" => "weather_clouds",
        "Thunderstorm" => "weather_thunder",
        "Snow" => "weather_snow",
        _ => "weather_default",
    };
    icon.to_string()
}

impl Weather {
    fn update_weather(&mut self) -> Result<()> {
        match self.service {
            WeatherService::OpenWeatherMap {
                api_key: Some(ref api_key),
                ref city_id,
                ref place,
                ref lat,
                ref lon,
                ref units,
            } => {
                // TODO: might be good to allow for different geolocation services to be used, similar to how we have `service` for the weather API
                let geoip_city = if self.autolocate {
                    let geoip_output = match Command::new("sh")
                        .args(&["-c", "curl --max-time 3 --silent 'https://ipapi.co/json/'"])
                        .output()
                    {
                        Ok(raw_output) => String::from_utf8(raw_output.stdout)
                            .block_error("weather", "Failed to decode")?,
                        Err(_) => {
                            // We don't want the bar to crash if we can't reach the geoip service
                            String::from("")
                        }
                    };

                    if geoip_output.is_empty() {
                        None
                    } else {
                        let geoip_json: serde_json::value::Value =
                            serde_json::from_str(&geoip_output).block_error(
                                "weather",
                                "Failed to parse JSON response from geoip service.",
                            )?;

                        geoip_json
                            .pointer("/city")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                    }
                } else {
                    None
                };

                let api_query = if let Some(city) = geoip_city {
                    format!("weather?q={}", city)
                } else if city_id.is_some() {
                    format!("weather?id={}", city_id.as_ref().unwrap())
                } else if place.is_some() {
                    format!("weather?q={}", place.as_ref().unwrap())
                } else if lat.is_some() && lon.is_some() {
                    format!(
                        "onecall?lat={}&lon={}&exclude=current,minutely,hourly",
                        lat.as_ref().unwrap(),
                        lon.as_ref().unwrap()
                    )
                } else {
                    return Err(BlockError(
                        "weather".to_string(),
                        format!(
                            "Either 'service.city_id' or 'service.place' must be provided. Add one to your config file or set with the environment variables {} or {}",
                            OPENWEATHERMAP_CITY_ID_ENV.to_string(),
                            OPENWEATHERMAP_PLACE_ENV.to_string(),
                        ),
                    ));
                };
                let output = Command::new("sh")
                    .args(&[
                        "-c",
                        // with these options curl will print http response body to stdout, http status code to stderr
                        &format!(
                            r#"curl -m 3 --silent \
                                "https://api.openweathermap.org/data/2.5/{api_query}&appid={api_key}&units={units}" \
                                --write-out "%{{stderr}} %{{http_code}}""#,
                            api_query = api_query,
                            api_key = api_key,
                            units = match *units {
                                OpenWeatherMapUnits::Metric => "metric",
                                OpenWeatherMapUnits::Imperial => "imperial",
                            },
                        ),
                    ])
                    .output()
                    .block_error("weather", "Failed to execute curl.")
                    .and_then(|raw_output| {
                        let status_code = String::from_utf8(raw_output.stderr)
                            .block_error("weather", "Invalid curl output")
                            .and_then(|out|
                                out.trim().parse::<i32>()
                                    .block_error("weather", &format!("Unexpected curl output {}", out))
                            )?;

                        // All 300-399 and >500 http codes should be considered as temporary error,
                        // and not result in block error, i.e. leave the output empty.
                        match status_code {
                            code if (code >= 300 && code < 400) || code >= 500 => Ok("".to_string()),
                            _ => String::from_utf8(raw_output.stdout)
                                .block_error("weather", "Received non-UTF8 characters in response."),
                        }
                    })?;

                // Don't error out on empty responses e.g. for when not
                // connected to the internet.
                if output.is_empty() {
                    self.weather.set_icon("weather_default");
                    self.weather_keys = HashMap::new();
                    return Ok(());
                }

                let json: serde_json::value::Value = serde_json::from_str(&output)
                    .block_error("weather", "Failed to parse JSON response.")?;

                // Try to convert an API error into a block error.
                if let Some(val) = json.get("message") {
                    return Err(BlockError(
                        "weather".to_string(),
                        format!("API Error: {}", val.as_str().unwrap()),
                    ));
                };

                let lookup = if lat.is_some() && lon.is_some() {
                    String::from("/daily/0/weather/0/main")
                } else {
                    String::from("/weather/0/main")
                };
                let raw_weather = json
                    .pointer(&lookup)
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .ok_or_else(|| malformed_json_error(lookup))?;

                let mut week_icons = Vec::new();
                if lat.is_some() && lon.is_some() {
                    for x in 1..8 {
                        week_icons.push(get_weather_icon(
                            json.pointer(&format!("/daily/{}/weather/0/main", x))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                                .ok_or_else(|| {
                                    malformed_json_error(format!("/daily/{}/weather/0/main", x))
                                })?,
                        ));
                    }
                } else {
                    week_icons.extend(
                        [
                            String::from("x"),
                            String::from("x"),
                            String::from("x"),
                            String::from("x"),
                            String::from("x"),
                            String::from("x"),
                            String::from("x"),
                        ]
                        .iter()
                        .cloned(),
                    );
                };

                let lookup = if lat.is_some() && lon.is_some() {
                    String::from("/daily/0/temp/day")
                } else {
                    String::from("/main/temp")
                };
                let raw_temp = json
                    .pointer(&lookup)
                    .and_then(|v| v.as_f64())
                    .ok_or_else(|| malformed_json_error(lookup))?;

                let lookup = if lat.is_some() && lon.is_some() {
                    String::from("/daily/0/humidity")
                } else {
                    String::from("/main/humidity")
                };
                let raw_humidity = json
                    .pointer(&lookup)
                    .map_or(Some(0.0), |v| v.as_f64()) // provide default value 0.0
                    .ok_or_else(|| malformed_json_error(lookup))?;

                let lookup = if lat.is_some() && lon.is_some() {
                    String::from("/daily/0/wind_speed")
                } else {
                    String::from("/wind/speed")
                };
                let raw_wind_speed: f64 = json
                    .pointer(&lookup)
                    .map_or(Some(0.0), |v| v.as_f64()) // provide default value 0.0
                    .ok_or_else(|| malformed_json_error(lookup))?; // error when conversion to f64 fails

                let lookup = if lat.is_some() && lon.is_some() {
                    String::from("/daily/0/wind_deg")
                } else {
                    String::from("/wind_deg")
                };
                let raw_wind_direction: Option<f64> = json
                    .pointer(&lookup)
                    .map_or(Some(None), |v| v.as_f64().map(Some)) // provide default value None
                    .ok_or_else(|| malformed_json_error(lookup))?; // error when conversion to f64 fails

                let lookup = if lat.is_some() && lon.is_some() {
                    String::from("/timezone")
                } else {
                    String::from("/name")
                };
                let raw_location = json
                    .pointer(&lookup)
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .ok_or_else(|| malformed_json_error(lookup))?;

                let lookup = if lat.is_some() && lon.is_some() {
                    String::from("/daily/0/sunset")
                } else {
                    String::from("/sys/sunset")
                };
                let sunset = Utc.timestamp(
                    json.pointer(&lookup)
                        .and_then(|v| v.as_i64())
                        .map(|s| s)
                        .ok_or_else(|| malformed_json_error(lookup))?,
                    0,
                );

                // Compute the Australian Apparent Temperature (AT),
                // using the metric formula found on Wikipedia.
                // If using imperial units, we must first convert to metric.
                let metric = match *units {
                    OpenWeatherMapUnits::Metric => true,
                    OpenWeatherMapUnits::Imperial => false,
                };

                let temp_celsius = if metric {
                    raw_temp
                } else {
                    // convert Fahrenheit to Celsius
                    (raw_temp - 32.0) * 0.556
                };

                let exponent = 17.27 * temp_celsius / (237.7 + temp_celsius);
                let water_vapor_pressure = raw_humidity * 0.06105 * exponent.exp();

                let metric_wind_speed = if metric {
                    raw_wind_speed
                } else {
                    // convert mph to m/s
                    raw_wind_speed * 0.447
                };

                let metric_apparent_temp =
                    temp_celsius + 0.33 * water_vapor_pressure - 0.7 * metric_wind_speed - 4.0;
                let apparent_temp = if metric {
                    metric_apparent_temp
                } else {
                    1.8 * metric_apparent_temp + 32.0
                };

                // Convert wind direction in azimuth degrees to abbreviation names
                fn convert_wind_direction(direction_opt: Option<f64>) -> String {
                    match direction_opt {
                        Some(direction) => match direction.round() as i64 {
                            24..=68 => "NE".to_string(),
                            69..=113 => "E".to_string(),
                            114..=158 => "SE".to_string(),
                            159..=203 => "S".to_string(),
                            204..=248 => "SW".to_string(),
                            249..=293 => "W".to_string(),
                            294..=338 => "NW".to_string(),
                            _ => "N".to_string(),
                        },
                        None => "-".to_string(),
                    }
                }

                self.weather
                    .set_icon(&get_weather_icon(raw_weather.clone()));

                self.weather_keys = map_to_owned!("{weather}" => raw_weather,
                                  "{temp}" => format!("{:.0}", raw_temp),
                                  "{humidity}" => format!("{:.0}", raw_humidity),
                                  "{apparent}" => format!("{:.0}",apparent_temp),
                                  "{wind}" => format!("{:.1}", raw_wind_speed),
                                  "{direction}" => convert_wind_direction(raw_wind_direction),
                                  "{location}" => raw_location,
                                  "{sunset}" => sunset.to_string(),
                                  "{day1_icon}" => week_icons[0],
                                  "{day2_icon}" => week_icons[1],
                                  "{day3_icon}" => week_icons[2],
                                  "{day4_icon}" => week_icons[3],
                                  "{day5_icon}" => week_icons[4],
                                  "{day6_icon}" => week_icons[5],
                                  "{day7_icon}" => week_icons[6]);
                Ok(())
            }
            WeatherService::OpenWeatherMap { ref api_key, .. } => {
                if api_key.is_none() {
                    Err(BlockError(
                        "weather".to_string(),
                        format!(
                            "Missing member 'service.api_key'. Add the member or configure with the environment variable {}",
                            OPENWEATHERMAP_API_KEY_ENV.to_string()
                        ),
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WeatherConfig {
    #[serde(
        default = "WeatherConfig::default_interval",
        deserialize_with = "deserialize_duration"
    )]
    pub interval: Duration,
    #[serde(default = "WeatherConfig::default_format")]
    pub format: String,
    pub service: WeatherService,
    #[serde(default = "WeatherConfig::default_autolocate")]
    pub autolocate: bool,
}

impl WeatherConfig {
    fn default_interval() -> Duration {
        Duration::from_secs(600)
    }

    fn default_format() -> String {
        "{weather} {temp}\u{00b0}".to_string()
    }

    fn default_autolocate() -> bool {
        false
    }
}

impl ConfigBlock for Weather {
    type Config = WeatherConfig;

    fn new(
        block_config: Self::Config,
        config: Config,
        _tx_update_request: Sender<Task>,
    ) -> Result<Self> {
        let id = Uuid::new_v4().to_simple().to_string();
        Ok(Weather {
            id: id.clone(),
            weather: ButtonWidget::new(config, &id),
            format: block_config.format,
            weather_keys: HashMap::new(),
            service: block_config.service,
            update_interval: block_config.interval,
            autolocate: block_config.autolocate,
        })
    }
}

impl Block for Weather {
    fn update(&mut self) -> Result<Option<Update>> {
        self.update_weather()?;
        // Display an error/disabled-looking widget when we don't have any
        // weather information, which is likely due to internet connectivity.
        if self.weather_keys.keys().len() == 0 {
            self.weather.set_text("×".to_string());
        } else {
            let fmt = FormatTemplate::from_string(&self.format)?;
            self.weather.set_text(fmt.render(&self.weather_keys));
        }
        Ok(Some(self.update_interval.into()))
    }

    fn view(&self) -> Vec<&dyn I3BarWidget> {
        vec![&self.weather]
    }

    fn click(&mut self, event: &I3BarEvent) -> Result<()> {
        if event.matches_name(self.id()) {
            if let MouseButton::Left = event.button {
                self.update()?;
            }
        }
        Ok(())
    }

    fn id(&self) -> &str {
        &self.id
    }
}
