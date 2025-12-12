// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(test)]
mod tests {
    use crate::apod::{ApodApi, ApodApiError, params::ApodParams};
    use crate::{Api, StellariaError};

    use chrono::NaiveDate;
    use reqwest::Client;

    struct Setup {
        apod: ApodApi,
    }

    impl Setup {
        fn new() -> Self {
            let _ = dotenv::dotenv();
            let client = Client::new();
            let token: String =
                std::env::var("API_TOKEN").expect("API_TOKEN must be set in .env or environment");

            Self {
                apod: ApodApi::new(token.to_string(), client.clone()),
            }
        }
    }

    // ==================== API Tests ====================

    #[tokio::test]
    async fn test_apod_default() {
        let setup = Setup::new();
        let params = ApodParams::builder();
        let resp = setup.apod.get(params.build().unwrap()).await;
        assert!(resp.is_ok());
    }

    #[test]
    fn test_apod_error_conversion() {
        let error = ApodApiError {
            code: 500,
            msg: "Internal Server Error".to_string(),
            service_version: "v1".to_string(),
        };

        let stellaria_error: StellariaError = error.into();
        assert!(
            stellaria_error
                .to_string()
                .contains("Internal Server Error")
        );
    }

    // ==================== Builder Pattern Tests ====================

    #[test]
    fn test_builder_default_uses_today() {
        let params = ApodParams::builder().build().unwrap();
        let today = chrono::Utc::now().date_naive();
        assert_eq!(params.date, Some(today));
        assert_eq!(params.count, None);
        assert!(!params.thumbs);
    }

    #[test]
    fn test_builder_with_single_date() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 12).unwrap();
        let params = ApodParams::builder().date(date).build().unwrap();

        assert_eq!(params.date, Some(date));
        assert_eq!(params.start_date, None);
        assert_eq!(params.end_date, None);
    }

    #[test]
    fn test_builder_with_count() {
        let params = ApodParams::builder().count(5).build().unwrap();

        assert_eq!(params.count, Some(5));
        assert_eq!(params.date, None);
    }

    #[test]
    fn test_builder_with_date_range() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();

        let params = ApodParams::builder()
            .date_range(start, end)
            .build()
            .unwrap();

        assert_eq!(params.start_date, Some(start));
        assert_eq!(params.end_date, Some(end));
        assert_eq!(params.date, None);
        assert_eq!(params.count, None);
    }

    #[test]
    fn test_builder_with_thumbs() {
        let params = ApodParams::builder().thumbs(true).build().unwrap();

        assert!(params.thumbs);
    }

    // ==================== Date Validation Tests ====================

    #[test]
    fn test_date_too_early_fails() {
        let too_early = NaiveDate::from_ymd_opt(1995, 6, 15).unwrap();
        let result = ApodParams::builder().date(too_early).build();

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Date must be between"));
    }

    #[test]
    fn test_too_late_fails() {
        let future = NaiveDate::from_ymd_opt(2099, 12, 31).unwrap();
        let result = ApodParams::builder().date(future).build();

        assert!(result.is_err());
    }

    #[test]
    fn test_recent_valid_date_succeeds() {
        let valid = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
        let result = ApodParams::builder().date(valid).build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_date_range_reversed_fails() {
        let start = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        let result = ApodParams::builder().date_range(start, end).build();

        assert!(result.is_err());
    }

    #[test]
    fn test_date_range_same_date_succeeds() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let result = ApodParams::builder().date_range(date, date).build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_date_range_valid_span_succeeds() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

        let result = ApodParams::builder().date_range(start, end).build();

        assert!(result.is_ok());
    }

    // ==================== Serialization Tests ====================

    #[test]
    fn test_serialize_with_date() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 12).unwrap();
        let params = ApodParams::builder().date(date).build().unwrap();

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["date"], "2024-12-12");
    }

    #[test]
    fn test_serialize_date_range() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();

        let params = ApodParams::builder()
            .date_range(start, end)
            .build()
            .unwrap();

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["start_date"], "2024-01-01");
        assert_eq!(json["end_date"], "2024-01-31");
    }

    #[test]
    fn test_serialize_count() {
        let params = ApodParams::builder().count(10).build().unwrap();

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["count"], 10);
    }

    #[test]
    fn test_serialize_thumbs() {
        let params = ApodParams::builder().thumbs(true).build().unwrap();

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["thumbs"], true);
    }

    #[test]
    fn test_deserialize_date_from_string() {
        let json = r#"{"date": "2024-12-12", "thumbs": false}"#;
        let params: ApodParams = serde_json::from_str(json).unwrap();

        assert_eq!(
            params.date,
            Some(NaiveDate::from_ymd_opt(2024, 12, 12).unwrap())
        );
    }

    #[test]
    fn test_round_trip_serialization() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let original = ApodParams::builder()
            .date(date)
            .thumbs(true)
            .build()
            .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ApodParams = serde_json::from_str(&json).unwrap();

        assert_eq!(original.date, deserialized.date);
        assert_eq!(original.thumbs, deserialized.thumbs);
    }
}
