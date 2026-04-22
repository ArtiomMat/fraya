#[derive(Debug)]
pub enum Error {
    Gltf(gltf::Error),
    NoScene,
}

impl From<gltf::Error> for Error {
    fn from(value: gltf::Error) -> Self {
        Self::Gltf(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Gltf(gltf) => write!(f, "{}", gltf),
            Error::NoScene => write!(f, "No scenes"),
        }
    }
}

impl std::error::Error for Error {}
