use alloy::primitives::{ruint, Uint, U256};

/// Converts a hexadecimal string (with "0x" prefix) to a decimal `U256`.
///
/// # Arguments
/// - `hexadecimal_str` - The hexadecimal string to convert.
///
/// # Returns
/// - `Ok(Uint<256, 4>)` on success.
/// - `Err(ruint::ParseError)` if the input is invalid.
///
/// # Example
/// ```
/// let decimal = hexadecimal_str_to_decimal_str("0x1a".to_string());
/// assert!(decimal.is_ok());
/// ```
pub fn hexadecimal_str_to_decimal_str(hexadecimal_str: String) -> Result<Uint<256, 4>, ruint::ParseError> {
    U256::from_str_radix(&hexadecimal_str.strip_prefix("0x").unwrap(), 16)
}

/// Converts a decimal string to a `0x` prefixed hexadecimal string.
///
/// # Arguments
/// - `decimal_str` - The decimal string to convert.
///
/// # Returns
/// - A `Result<String, ruint::ParseError>`, where the `Ok` variant contains the `0x` prefixed hexadecimal string.
///
/// # Example
/// ```
/// let hex_str = decimal_str_to_hexadecimal_str("26".to_string());
/// assert_eq!(hex_str.unwrap(), "0x1a");
/// ```
pub fn decimal_str_to_hexadecimal_str(decimal_str: String) -> Result<String, ruint::ParseError> {
    let value = U256::from_str_radix(&decimal_str, 10)?;
    Ok(format!("0x{:x}", value))
}
