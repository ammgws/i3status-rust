// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

use blocks::dbus;
use blocks::dbus::arg;

pub trait OrgFreedesktopDBusIntrospectable {
    type Err;
    fn introspect(&self) -> Result<String, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target = dbus::Connection>> OrgFreedesktopDBusIntrospectable
    for dbus::ConnPath<'a, C>
{
    type Err = dbus::Error;

    fn introspect(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Introspectable".into(),
            &"Introspect".into(),
            |_| {}
        ));
        try!(m.as_result());
        let mut i = m.iter_init();
        let xml: String = try!(i.read());
        Ok(xml)
    }
}

pub trait NetConnmanIwdDevice {
    type Err;
    fn scan(&self) -> Result<(), Self::Err>;
    fn disconnect(&self) -> Result<(), Self::Err>;
    fn get_ordered_networks(
        &self,
    ) -> Result<Vec<(dbus::Path<'static>, String, i16, String)>, Self::Err>;
    fn register_signal_level_agent(
        &self,
        path: dbus::Path,
        levels: Vec<i16>,
    ) -> Result<(), Self::Err>;
    fn unregister_signal_level_agent(&self, path: dbus::Path) -> Result<(), Self::Err>;
    fn connect_hidden_network(&self, name: &str) -> Result<(), Self::Err>;
    fn get_name(&self) -> Result<String, Self::Err>;
    fn get_address(&self) -> Result<String, Self::Err>;
    fn get_connected_network(&self) -> Result<dbus::Path<'static>, Self::Err>;
    fn get_wds(&self) -> Result<bool, Self::Err>;
    fn set_wds(&self, value: bool) -> Result<(), Self::Err>;
    fn get_powered(&self) -> Result<bool, Self::Err>;
    fn set_powered(&self, value: bool) -> Result<(), Self::Err>;
    fn get_scanning(&self) -> Result<bool, Self::Err>;
    fn get_state(&self) -> Result<String, Self::Err>;
    fn get_adapter(&self) -> Result<dbus::Path<'static>, Self::Err>;
    fn get_mode(&self) -> Result<String, Self::Err>;
    fn set_mode(&self, value: String) -> Result<(), Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target = dbus::Connection>> NetConnmanIwdDevice
    for dbus::ConnPath<'a, C>
{
    type Err = dbus::Error;

    fn scan(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"net.connman.iwd.Device".into(),
            &"Scan".into(),
            |_| {}
        ));
        try!(m.as_result());
        Ok(())
    }

    fn disconnect(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"net.connman.iwd.Device".into(),
            &"Disconnect".into(),
            |_| {}
        ));
        try!(m.as_result());
        Ok(())
    }

    fn get_ordered_networks(
        &self,
    ) -> Result<Vec<(dbus::Path<'static>, String, i16, String)>, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"net.connman.iwd.Device".into(),
            &"GetOrderedNetworks".into(),
            |_| {}
        ));
        try!(m.as_result());
        let mut i = m.iter_init();
        let networks: Vec<(dbus::Path<'static>, String, i16, String)> = try!(i.read());
        Ok(networks)
    }

    fn register_signal_level_agent(
        &self,
        path: dbus::Path,
        levels: Vec<i16>,
    ) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"net.connman.iwd.Device".into(),
            &"RegisterSignalLevelAgent".into(),
            |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append(path);
                i.append(levels);
            }
        ));
        try!(m.as_result());
        Ok(())
    }

    fn unregister_signal_level_agent(&self, path: dbus::Path) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"net.connman.iwd.Device".into(),
            &"UnregisterSignalLevelAgent".into(),
            |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append(path);
            }
        ));
        try!(m.as_result());
        Ok(())
    }

    fn connect_hidden_network(&self, name: &str) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"net.connman.iwd.Device".into(),
            &"ConnectHiddenNetwork".into(),
            |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append(name);
            }
        ));
        try!(m.as_result());
        Ok(())
    }

    fn get_name(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Device",
            "Name",
        )
    }

    fn get_address(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Device",
            "Address",
        )
    }

    fn get_connected_network(&self) -> Result<dbus::Path<'static>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Device",
            "ConnectedNetwork",
        )
    }

    fn get_wds(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Device",
            "WDS",
        )
    }

    fn get_powered(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Device",
            "Powered",
        )
    }

    fn get_scanning(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Device",
            "Scanning",
        )
    }

    fn get_state(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Device",
            "State",
        )
    }

    fn get_adapter(&self) -> Result<dbus::Path<'static>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Device",
            "Adapter",
        )
    }

    fn get_mode(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Device",
            "Mode",
        )
    }

    fn set_wds(&self, value: bool) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "net.connman.iwd.Device",
            "WDS",
            value,
        )
    }

    fn set_powered(&self, value: bool) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "net.connman.iwd.Device",
            "Powered",
            value,
        )
    }

    fn set_mode(&self, value: String) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "net.connman.iwd.Device",
            "Mode",
            value,
        )
    }
}

pub trait OrgFreedesktopDBusProperties {
    type Err;
    fn get(
        &self,
        interface_name: &str,
        property_name: &str,
    ) -> Result<arg::Variant<Box<arg::RefArg>>, Self::Err>;
    fn set(
        &self,
        interface_name: &str,
        property_name: &str,
        value: arg::Variant<Box<arg::RefArg>>,
    ) -> Result<(), Self::Err>;
    fn get_all(
        &self,
        interface_name: &str,
    ) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target = dbus::Connection>> OrgFreedesktopDBusProperties
    for dbus::ConnPath<'a, C>
{
    type Err = dbus::Error;

    fn get(
        &self,
        interface_name: &str,
        property_name: &str,
    ) -> Result<arg::Variant<Box<arg::RefArg>>, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append(interface_name);
                i.append(property_name);
            }
        ));
        try!(m.as_result());
        let mut i = m.iter_init();
        let value: arg::Variant<Box<arg::RefArg>> = try!(i.read());
        Ok(value)
    }

    fn set(
        &self,
        interface_name: &str,
        property_name: &str,
        value: arg::Variant<Box<arg::RefArg>>,
    ) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Set".into(),
            |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append(interface_name);
                i.append(property_name);
                i.append(value);
            }
        ));
        try!(m.as_result());
        Ok(())
    }

    fn get_all(
        &self,
        interface_name: &str,
    ) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Self::Err>
    {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"GetAll".into(),
            |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append(interface_name);
            }
        ));
        try!(m.as_result());
        let mut i = m.iter_init();
        let props: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>> =
            try!(i.read());
        Ok(props)
    }
}

#[derive(Debug, Default)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>,
    pub invalidated_properties: Vec<String>,
}

impl dbus::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
    fn append(&self, i: &mut arg::IterAppend) {
        (&self.interface_name as &arg::RefArg).append(i);
        (&self.changed_properties as &arg::RefArg).append(i);
        (&self.invalidated_properties as &arg::RefArg).append(i);
    }
    fn get(&mut self, i: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        self.interface_name = try!(i.read());
        self.changed_properties = try!(i.read());
        self.invalidated_properties = try!(i.read());
        Ok(())
    }
}

pub trait NetConnmanIwdWiFiSimpleConfiguration {
    type Err;
    fn push_button(&self) -> Result<(), Self::Err>;
    fn generate_pin(&self) -> Result<String, Self::Err>;
    fn start_pin(&self, pin: &str) -> Result<(), Self::Err>;
    fn cancel(&self) -> Result<(), Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target = dbus::Connection>> NetConnmanIwdWiFiSimpleConfiguration
    for dbus::ConnPath<'a, C>
{
    type Err = dbus::Error;

    fn push_button(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"net.connman.iwd.WiFiSimpleConfiguration".into(),
            &"PushButton".into(),
            |_| {}
        ));
        try!(m.as_result());
        Ok(())
    }

    fn generate_pin(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"net.connman.iwd.WiFiSimpleConfiguration".into(),
            &"GeneratePin".into(),
            |_| {}
        ));
        try!(m.as_result());
        let mut i = m.iter_init();
        let pin: String = try!(i.read());
        Ok(pin)
    }

    fn start_pin(&self, pin: &str) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"net.connman.iwd.WiFiSimpleConfiguration".into(),
            &"StartPin".into(),
            |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append(pin);
            }
        ));
        try!(m.as_result());
        Ok(())
    }

    fn cancel(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"net.connman.iwd.WiFiSimpleConfiguration".into(),
            &"Cancel".into(),
            |_| {}
        ));
        try!(m.as_result());
        Ok(())
    }
}
