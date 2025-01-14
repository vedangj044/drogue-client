mod downstream;
mod kafka;

pub use downstream::*;
pub use kafka::*;

use crate::{
    dialect, meta::v1::NonScopedMetadata, serde::Base64Standard, translator, Dialect, Section,
    Translator,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// An application, owning devices.
#[derive(Clone, Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Application {
    pub metadata: NonScopedMetadata,
    #[serde(default)]
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub spec: Map<String, Value>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub status: Map<String, Value>,
}

translator!(Application);

/// The application's trust-anchors.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct ApplicationSpecTrustAnchors {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub anchors: Vec<ApplicationSpecTrustAnchorEntry>,
}

dialect!(ApplicationSpecTrustAnchors [Section::Spec => "trustAnchors"]);

/// A single trust-anchor entry.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct ApplicationSpecTrustAnchorEntry {
    #[serde(with = "Base64Standard")]
    pub certificate: Vec<u8>,
}

/// The status of the trust-anchors.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct ApplicationStatusTrustAnchors {
    pub anchors: Vec<ApplicationStatusTrustAnchorEntry>,
}

dialect!(ApplicationStatusTrustAnchors [Section::Status => "trustAnchors"]);

/// A single trust-anchor status.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ApplicationStatusTrustAnchorEntry {
    #[serde(rename_all = "camelCase")]
    Valid {
        subject: String,
        #[serde(with = "Base64Standard")]
        certificate: Vec<u8>,
        not_before: DateTime<Utc>,
        not_after: DateTime<Utc>,
    },
    Invalid {
        error: String,
        message: String,
    },
}
