use thiserror::Error;

/// Errors returned by check-rke2-version
#[derive(Error, Debug)]
pub enum Error {
    #[error("Channel not found: {0}")]
    NoChannelFound(String),

    /// Configuration is required to send notifications
    #[error("No configuration present: {0}")]
    MissingConfiguration(String),

    // ### Converting from other error types ###
    /// Pass-thru [`std::io::Error`].
    #[error("std::io Error: {0}")]
    IO(#[from] std::io::Error),

    /// Pass-thru `serde_json::Error`.
    #[error("Serde_json Error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    /// Pass-thru `serde_json::Error`.
    #[error("reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Pass-thru `pass-it-on::Error`.
    #[error("Pass-it-on Error: {0}")]
    PassItOn(#[from] pass_it_on::Error),

    /// Pass-thru `toml::de::Error`.
    #[error("Serde Toml Error: {0}")]
    SerdeToml(#[from] toml::de::Error),
}
