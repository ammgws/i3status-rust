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

pub trait NetConnmanIwdNetwork {
    type Err;
    fn connect(&self) -> Result<(), Self::Err>;
    fn get_name(&self) -> Result<String, Self::Err>;
    fn get_connected(&self) -> Result<bool, Self::Err>;
    fn get_device(&self) -> Result<dbus::Path<'static>, Self::Err>;
    fn get_type(&self) -> Result<String, Self::Err>;
    fn get_known_network(&self) -> Result<dbus::Path<'static>, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target = dbus::Connection>> NetConnmanIwdNetwork
    for dbus::ConnPath<'a, C>
{
    type Err = dbus::Error;

    fn connect(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"net.connman.iwd.Network".into(),
            &"Connect".into(),
            |_| {}
        ));
        try!(m.as_result());
        Ok(())
    }

    fn get_name(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Network",
            "Name",
        )
    }

    fn get_connected(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Network",
            "Connected",
        )
    }

    fn get_device(&self) -> Result<dbus::Path<'static>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Network",
            "Device",
        )
    }

    fn get_type(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Network",
            "Type",
        )
    }

    fn get_known_network(&self) -> Result<dbus::Path<'static>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "net.connman.iwd.Network",
            "KnownNetwork",
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
