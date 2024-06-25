/// Library errors.
#[derive(Fail, Debug)]
pub enum PocketcastsError {
    #[fail(display = "Http Error: {}", _0)]
    ReqwestError(#[cause] reqwest::Error),
    #[fail(display = "could not authorize")]
    Unauthorized,
}

impl From<reqwest::Error> for PocketcastsError {
    fn from(error: reqwest::Error) -> Self {
        PocketcastsError::ReqwestError(error)
    }
}
