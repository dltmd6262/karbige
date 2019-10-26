use std::fmt;

#[derive(Debug, Clone)]
pub struct PairParsingError;

impl fmt::Display for PairParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error in converting currency string to enum, or vice versa"
        )
    }
}

#[derive(Debug)]
pub enum FetchError {
    Network(reqwest::Error),
    JSON(serde_json::Error),
    Parsing(PairParsingError),
}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> FetchError {
        FetchError::Network(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::JSON(err)
    }
}

impl From<PairParsingError> for FetchError {
    fn from(err: PairParsingError) -> FetchError {
        FetchError::Parsing(err)
    }
}
