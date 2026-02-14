use idsmith::swift::{GenOptions, Registry};
use rand::thread_rng;

#[test]
fn test_swift_generation() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();

    for _ in 0..100 {
        let result = registry.generate(&opts, &mut rng);
        assert!(
            registry.validate(&result.code),
            "Failed to validate generated code: {}",
            result.code
        );
        assert!(result.valid);
    }
}

#[test]
fn test_specific_countries() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let countries = vec!["US", "GB", "DE", "FR"];

    for country in countries {
        let opts = GenOptions {
            country: Some(country.to_string()),
        };
        let result = registry.generate(&opts, &mut rng);
        assert_eq!(result.country, country);
        assert!(
            registry.validate(&result.code),
            "Failed to validate {}: {}",
            country,
            result.code
        );
    }
}
