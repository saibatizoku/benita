//! Command-line interpreter items.
use utilities::atof;

use url::Url;

/// Validator function for URLs.
pub fn is_url(v: String) -> ::std::result::Result<(), String> {
    match Url::parse(&v) {
        Ok(_) => Ok(()),
        _ => Err("Invalid URL".to_string()),
    }
}

/// Validator function for floating point numbers.
pub fn is_float(v: String) -> ::std::result::Result<(), String> {
    match atof(&v) {
        Ok(_) => Ok(()),
        _ => Err("The value is not numeric.".to_string()),
    }
}
