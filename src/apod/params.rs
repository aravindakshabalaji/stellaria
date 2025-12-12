// SPDX-License-Identifier: MIT OR Apache-2.0

use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::StellariaResult;
use crate::apod::{ApodError::ApodParamsError, date_serde};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApodParams {
    #[serde(default, skip_serializing_if = "Option::is_none", with = "date_serde")]
    pub date: Option<NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "date_serde")]
    pub start_date: Option<NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "date_serde")]
    pub end_date: Option<NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
    pub thumbs: bool,
}

#[derive(Default, Debug)]
pub struct ApodParamsBuilder {
    thumbs: bool,
    range: Option<ApodRange>,
}

#[derive(Debug)]
enum ApodRange {
    Count(u8),
    DateRange {
        start_date: NaiveDate,
        end_date: NaiveDate,
    },
    Date(NaiveDate),
}

impl ApodParamsBuilder {
    pub fn thumbs(mut self, thumbs: bool) -> Self {
        self.thumbs = thumbs;
        self
    }

    pub fn count(mut self, count: u8) -> Self {
        self.range = Some(ApodRange::Count(count));
        self
    }

    pub fn date(mut self, date: NaiveDate) -> Self {
        self.range = Some(ApodRange::Date(date));
        self
    }

    pub fn date_range(mut self, start_date: NaiveDate, end_date: NaiveDate) -> Self {
        self.range = Some(ApodRange::DateRange {
            start_date,
            end_date,
        });
        self
    }

    pub fn build(self) -> StellariaResult<ApodParams> {
        let mut params = ApodParams {
            thumbs: self.thumbs,
            ..Default::default()
        };

        if let Some(range) = self.range {
            match range {
                ApodRange::Count(count) => params.count = Some(count),
                ApodRange::Date(date) => {
                    if (date > Utc::now().date_naive())
                        || (date < NaiveDate::from_ymd_opt(1995, 6, 16).unwrap())
                    {
                        return Err(ApodParamsError(
                            "Date must be between Jun 16, 1995 and Dec 12, 2025.".to_string(),
                        )
                        .into());
                    }
                    params.date = Some(date)
                }
                ApodRange::DateRange {
                    start_date,
                    end_date,
                } => {
                    if start_date > end_date {
                        return Err(ApodParamsError(
                            "Start date cannot be greater than end date".to_string(),
                        )
                        .into());
                    }
                    params.start_date = Some(start_date);
                    params.end_date = Some(end_date);
                }
            }
        } else {
            params.date = Some(Utc::now().date_naive());
        }

        Ok(params)
    }
}

impl ApodParams {
    pub fn builder() -> ApodParamsBuilder {
        ApodParamsBuilder::default()
    }
}
