// SPDX-License-Identifier: MIT OR Apache-2.0

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serializer};

const FORMAT: &str = "%Y-%m-%d";

pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(d) => serializer.serialize_str(&d.format(FORMAT).to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    s.map(|date_str| NaiveDate::parse_from_str(&date_str, FORMAT).map_err(serde::de::Error::custom))
        .transpose()
}
