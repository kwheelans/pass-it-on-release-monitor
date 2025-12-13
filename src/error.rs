use thiserror::Error;

/// Errors returned by check-rke2-version
#[derive(Error, Debug)]
pub enum Error {
    /// Provided rancher channel not found
    #[error("Rancher channel not found: {0}")]
    RancherChannelNotFound(String),

    /// Configuration is required to monitor and send notifications
    #[error("No configuration present: {0}")]
    MissingConfiguration(String),

    /// Cannot create a Monitor from the entity Model data
    #[error("Unable to create known Monitor type from Model")]
    ModelConversionFailed,

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

    /// Pass-thru `octocrab::Error`.
    #[error("Octocrab Error: {0}")]
    Octocrab(#[from] octocrab::Error),

    /// Pass-thru `sea_orm::error::DbErr`.
    #[error("Database Error: {0}")]
    Database(#[from] sea_orm::error::DbErr),

    #[error("Zip Archive Error: {0}")]
    ZipArchive(#[from] zip::result::ZipError)
}
