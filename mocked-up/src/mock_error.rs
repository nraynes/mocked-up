use rust_alert::alert;

/// A custom error type used to convert error types from various crates.
#[alert(errors = [
    std::io::Error,
    String,
    std::string::FromUtf8Error,
    serde_json::Error,
    std::num::ParseIntError,
])]
pub struct MockError {}
