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
