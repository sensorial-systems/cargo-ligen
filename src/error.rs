#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    ExecutionFailure(i32)
}

impl From<std::io::Error> for Error {
    fn from(io: std::io::Error) -> Self {
        Self::IO(io)
    }
}
