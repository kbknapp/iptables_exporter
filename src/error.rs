use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum IptablesError {
    #[error("Invalid policy {0}. Choices are ACCEPT or DROP")]
    InvalidPolicy(String),
}
