use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

/// A struct representing a Docker 'volume'
#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Volume {
    /// The name of the Docker volume
    pub name: String,

    /// The volume driver
    pub driver: String,

    /// The location on the host filesystem where the volume is mounted
    pub mountpoint: PathBuf,

    /// Low-level details about the volume, provided by the volume driver
    #[serde(default)]
    pub status: HashMap<String, String>,

    /// User-defined key/value metadata
    #[serde(default)]
    pub labels: HashMap<String, String>,

    /// The scope of the volume
    pub scope: Scope,

    /// The datetime that the container was created
    pub created_at: DateTime<Utc>,
}

/// The level at which the volume exists
#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    /// machine level
    Local,
    /// cluster-wide
    Global,
}
