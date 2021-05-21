#[derive(Debug)]
pub enum Error {
    CargoToml(cargo_toml::Error),
    IO(std::io::Error),
    ExecutionFailure(i32),
    String(String),
}

impl From<std::io::Error> for Error {
    fn from(io: std::io::Error) -> Self {
        Self::IO(io)
    }
}

impl From<cargo_toml::Error> for Error {
    fn from(cargo_toml_error: cargo_toml::Error) -> Self {
        Self::CargoToml(cargo_toml_error)
    }
}
