use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
};

pub trait Enviroment {
    fn get<K: AsRef<OsStr>>(&self, key: K) -> Option<OsString>;
    fn set<K: AsRef<OsStr>, V: AsRef<OsStr>>(&mut self, key: K, value: V);
}

pub struct ProcessEnv;

impl Enviroment for ProcessEnv {
    fn get<K: AsRef<OsStr>>(&self, key: K) -> Option<OsString> {
        std::env::var_os(key)
    }

    fn set<K: AsRef<OsStr>, V: AsRef<OsStr>>(&mut self, key: K, value: V) {
        std::env::set_var(key, value)
    }
}

pub struct FakeEnv(HashMap<OsString, OsString>);

impl Default for FakeEnv {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'a> Enviroment for FakeEnv {
    fn get<K: AsRef<OsStr>>(&self, key: K) -> Option<OsString> {
        self.0.get(key.as_ref()).cloned()
    }

    fn set<K: AsRef<OsStr>, V: AsRef<OsStr>>(&mut self, key: K, value: V) {
        self.0
            .insert(key.as_ref().to_os_string(), value.as_ref().to_os_string());
    }
}
