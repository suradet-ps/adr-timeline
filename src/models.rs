//! Data models for the ADR Timeline application.
use serde::{Deserialize, Serialize};

/// Represents the type of timeline item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ItemType {
  Drug,
  Reaction,
  Lab,
}

/// A single timeline item (drug, reaction, or lab entry).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Item {
  pub(crate) id: u64,
  pub(crate) name: String,
  #[serde(rename = "rawName")]
  pub(crate) raw_name: String,
  #[serde(default)]
  pub(crate) dose: String,
  #[serde(default)]
  pub(crate) start: Option<String>,
  #[serde(default)]
  pub(crate) end: Option<String>,
  #[serde(default)]
  pub(crate) ongoing: bool,
  #[serde(rename = "startUnknown", default)]
  pub(crate) start_unknown: bool,
  #[serde(rename = "type")]
  pub(crate) item_type: ItemType,
  #[serde(rename = "firstDose", default)]
  pub(crate) first_dose: String,
  #[serde(rename = "lastDose", default)]
  pub(crate) last_dose: String,
}

/// Patient/header information for the timeline document.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct FormData {
  #[serde(rename = "hospitalName", default)]
  pub(crate) hospital_name: String,
  #[serde(rename = "patientName", default)]
  pub(crate) patient_name: String,
  #[serde(rename = "patientHN", default)]
  pub(crate) patient_hn: String,
  #[serde(rename = "patientAN", default)]
  pub(crate) patient_an: String,
  #[serde(rename = "reportDate", default)]
  pub(crate) report_date: String,
  #[serde(rename = "preparedBy", default)]
  pub(crate) prepared_by: String,
  #[serde(rename = "pharmaNote", default)]
  pub(crate) pharma_note: String,
}

/// The complete exportable data structure for JSON save/load.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AppData {
  pub(crate) header: FormData,
  pub(crate) items: Vec<Item>,
}
