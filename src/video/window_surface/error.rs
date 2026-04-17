
#[derive(Debug)]
pub enum Error { Sdl3(String) }

impl From<sdl3::Error> for Error {
    fn from(value: sdl3::Error) -> Self {
        Self::Sdl3(value.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Sdl3(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {}
