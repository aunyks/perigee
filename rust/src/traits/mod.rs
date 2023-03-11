pub trait TryFromToml {
    fn try_from_toml(toml_str: &str) -> Result<Self, String>
    where
        Self: Sized;
}

pub trait TryToToml {
    fn try_to_toml(&self) -> Result<String, String>;
}

pub trait TryFromBytes {
    fn try_from_bytes(bytes: &[u8]) -> Result<Self, String>
    where
        Self: Sized;
}

pub trait TryToBytes {
    fn try_to_bytes(&self) -> Result<Vec<u8>, String>;
}

pub trait FromConfig {
    type Config<'a>;

    fn from_config<'a>(config: Self::Config<'a>) -> Self;

    fn set_config<'a>(&mut self, _config: Self::Config<'a>) {}
}
