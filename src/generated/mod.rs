#![allow(unknown_lints)]
#![allow(clippy)]
#![allow(missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]
// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

use dbus;
use dbus::arg;

pub trait OrgFreedesktopDBusProperties {
    type Err;
    fn get(
        &self,
        interface_name: &str,
        property_name: &str,
    ) -> Result<arg::Variant<Box<arg::RefArg>>, Self::Err>;
    fn get_all(
        &self,
        interface_name: &str,
    ) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Self::Err>;
    fn set(
        &self,
        interface_name: &str,
        property_name: &str,
        value: arg::Variant<Box<arg::RefArg>>,
    ) -> Result<(), Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target = dbus::Connection>> OrgFreedesktopDBusProperties
    for dbus::ConnPath<'a, C> {
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
            },
        ));
        try!(m.as_result());
        let mut i = m.iter_init();
        let value: arg::Variant<Box<arg::RefArg>> = try!(i.read());
        Ok(value)
    }

    fn get_all(
        &self,
        interface_name: &str,
    ) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"GetAll".into(),
            |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append(interface_name);
            },
        ));
        try!(m.as_result());
        let mut i = m.iter_init();
        let properties: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>> =
            try!(i.read());
        Ok(properties)
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
            },
        ));
        try!(m.as_result());
        Ok(())
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

pub trait OrgFreedesktopDBusIntrospectable {
    type Err;
    fn introspect(&self) -> Result<String, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target = dbus::Connection>> OrgFreedesktopDBusIntrospectable
    for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn introspect(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.freedesktop.DBus.Introspectable".into(), &"Introspect".into(), |_| {
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let xml_data: String = try!(i.read());
        Ok(xml_data)
    }
}

pub trait OrgFreedesktopDBusPeer {
    type Err;
    fn ping(&self) -> Result<(), Self::Err>;
    fn get_machine_id(&self) -> Result<String, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target = dbus::Connection>> OrgFreedesktopDBusPeer
    for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn ping(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Peer".into(),
            &"Ping".into(),
            |_| {},
        ));
        try!(m.as_result());
        Ok(())
    }

    fn get_machine_id(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Peer".into(),
            &"GetMachineId".into(),
            |_| {},
        ));
        try!(m.as_result());
        let mut i = m.iter_init();
        let machine_uuid: String = try!(i.read());
        Ok(machine_uuid)
    }
}

pub trait OrgMprisMediaPlayer2 {
    type Err;
    fn raise(&self) -> Result<(), Self::Err>;
    fn quit(&self) -> Result<(), Self::Err>;
    fn get_can_quit(&self) -> Result<bool, Self::Err>;
    fn get_can_raise(&self) -> Result<bool, Self::Err>;
    fn get_has_track_list(&self) -> Result<bool, Self::Err>;
    fn get_identity(&self) -> Result<String, Self::Err>;
    fn get_desktop_entry(&self) -> Result<String, Self::Err>;
    fn get_supported_uri_schemes(&self) -> Result<Vec<String>, Self::Err>;
    fn get_supported_mime_types(&self) -> Result<Vec<String>, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target = dbus::Connection>> OrgMprisMediaPlayer2
    for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn raise(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.mpris.MediaPlayer2".into(),
            &"Raise".into(),
            |_| {},
        ));
        try!(m.as_result());
        Ok(())
    }

    fn quit(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.mpris.MediaPlayer2".into(),
            &"Quit".into(),
            |_| {},
        ));
        try!(m.as_result());
        Ok(())
    }

    fn get_can_quit(&self) -> Result<bool, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2");
                i.append("CanQuit");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_can_raise(&self) -> Result<bool, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2");
                i.append("CanRaise");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_has_track_list(&self) -> Result<bool, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2");
                i.append("HasTrackList");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_identity(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2");
                i.append("Identity");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_desktop_entry(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2");
                i.append("DesktopEntry");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_supported_uri_schemes(&self) -> Result<Vec<String>, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2");
                i.append("SupportedUriSchemes");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_supported_mime_types(&self) -> Result<Vec<String>, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2");
                i.append("SupportedMimeTypes");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }
}

pub trait OrgMprisMediaPlayer2Player {
    type Err;
    fn next(&self) -> Result<(), Self::Err>;
    fn previous(&self) -> Result<(), Self::Err>;
    fn pause(&self) -> Result<(), Self::Err>;
    fn play_pause(&self) -> Result<(), Self::Err>;
    fn stop(&self) -> Result<(), Self::Err>;
    fn play(&self) -> Result<(), Self::Err>;
    fn seek(&self, offset: i64) -> Result<(), Self::Err>;
    fn set_position(&self, track_id: dbus::Path, position: i64) -> Result<(), Self::Err>;
    fn open_uri(&self, uri: &str) -> Result<(), Self::Err>;
    fn get_playback_status(&self) -> Result<String, Self::Err>;
    fn get_loop_status(&self) -> Result<String, Self::Err>;
    fn set_loop_status(&self, value: String) -> Result<(), Self::Err>;
    fn get_rate(&self) -> Result<f64, Self::Err>;
    fn set_rate(&self, value: f64) -> Result<(), Self::Err>;
    fn get_shuffle(&self) -> Result<bool, Self::Err>;
    fn set_shuffle(&self, value: bool) -> Result<(), Self::Err>;
    fn get_metadata(
        &self,
    ) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Self::Err>;
    fn get_volume(&self) -> Result<f64, Self::Err>;
    fn set_volume(&self, value: f64) -> Result<(), Self::Err>;
    fn get_position(&self) -> Result<i64, Self::Err>;
    fn get_minimum_rate(&self) -> Result<f64, Self::Err>;
    fn get_maximum_rate(&self) -> Result<f64, Self::Err>;
    fn get_can_go_next(&self) -> Result<bool, Self::Err>;
    fn get_can_go_previous(&self) -> Result<bool, Self::Err>;
    fn get_can_play(&self) -> Result<bool, Self::Err>;
    fn get_can_pause(&self) -> Result<bool, Self::Err>;
    fn get_can_seek(&self) -> Result<bool, Self::Err>;
    fn get_can_control(&self) -> Result<bool, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target = dbus::Connection>> OrgMprisMediaPlayer2Player
    for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn next(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.mpris.MediaPlayer2.Player".into(),
            &"Next".into(),
            |_| {},
        ));
        try!(m.as_result());
        Ok(())
    }

    fn previous(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.mpris.MediaPlayer2.Player".into(),
            &"Previous".into(),
            |_| {},
        ));
        try!(m.as_result());
        Ok(())
    }

    fn pause(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.mpris.MediaPlayer2.Player".into(),
            &"Pause".into(),
            |_| {},
        ));
        try!(m.as_result());
        Ok(())
    }

    fn play_pause(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.mpris.MediaPlayer2.Player".into(),
            &"PlayPause".into(),
            |_| {},
        ));
        try!(m.as_result());
        Ok(())
    }

    fn stop(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.mpris.MediaPlayer2.Player".into(),
            &"Stop".into(),
            |_| {},
        ));
        try!(m.as_result());
        Ok(())
    }

    fn play(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.mpris.MediaPlayer2.Player".into(),
            &"Play".into(),
            |_| {},
        ));
        try!(m.as_result());
        Ok(())
    }

    fn seek(&self, offset: i64) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.mpris.MediaPlayer2.Player".into(),
            &"Seek".into(),
            |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append(offset);
            },
        ));
        try!(m.as_result());
        Ok(())
    }

    fn set_position(&self, track_id: dbus::Path, position: i64) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.mpris.MediaPlayer2.Player".into(),
            &"SetPosition".into(),
            |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append(track_id);
                i.append(position);
            },
        ));
        try!(m.as_result());
        Ok(())
    }

    fn open_uri(&self, uri: &str) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.mpris.MediaPlayer2.Player".into(),
            &"OpenUri".into(),
            |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append(uri);
            },
        ));
        try!(m.as_result());
        Ok(())
    }

    fn get_playback_status(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("PlaybackStatus");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_loop_status(&self) -> Result<String, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("LoopStatus");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_rate(&self) -> Result<f64, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("Rate");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_shuffle(&self) -> Result<bool, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("Shuffle");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_metadata(
        &self,
    ) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("Metadata");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_volume(&self) -> Result<f64, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("Volume");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_position(&self) -> Result<i64, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("Position");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_minimum_rate(&self) -> Result<f64, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("MinimumRate");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_maximum_rate(&self) -> Result<f64, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("MaximumRate");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_can_go_next(&self) -> Result<bool, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("CanGoNext");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_can_go_previous(&self) -> Result<bool, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("CanGoPrevious");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_can_play(&self) -> Result<bool, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("CanPlay");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_can_pause(&self) -> Result<bool, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("CanPause");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_can_seek(&self) -> Result<bool, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("CanSeek");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn get_can_control(&self) -> Result<bool, Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Get".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("CanControl");
            },
        ));
        let v: arg::Variant<_> = try!(try!(m.as_result()).read1());
        Ok(v.0)
    }

    fn set_loop_status(&self, value: String) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Set".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("LoopStatus");
                i.append(arg::Variant(value));
            },
        ));
        m.as_result().map(|_| ())
    }

    fn set_rate(&self, value: f64) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Set".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("Rate");
                i.append(arg::Variant(value));
            },
        ));
        m.as_result().map(|_| ())
    }

    fn set_shuffle(&self, value: bool) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Set".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("Shuffle");
                i.append(arg::Variant(value));
            },
        ));
        m.as_result().map(|_| ())
    }

    fn set_volume(&self, value: f64) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(
            &"org.freedesktop.DBus.Properties".into(),
            &"Set".into(),
            move |msg| {
                let mut i = arg::IterAppend::new(msg);
                i.append("org.mpris.MediaPlayer2.Player");
                i.append("Volume");
                i.append(arg::Variant(value));
            },
        ));
        m.as_result().map(|_| ())
    }
}

#[derive(Debug, Default)]
pub struct OrgMprisMediaPlayer2PlayerSeeked {
    pub position: i64,
}

impl dbus::SignalArgs for OrgMprisMediaPlayer2PlayerSeeked {
    const NAME: &'static str = "Seeked";
    const INTERFACE: &'static str = "org.mpris.MediaPlayer2.Player";
    fn append(&self, i: &mut arg::IterAppend) {
        (&self.position as &arg::RefArg).append(i);
    }
    fn get(&mut self, i: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        self.position = try!(i.read());
        Ok(())
    }
}
