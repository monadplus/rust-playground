/// Converts a hex string interpreted as big-endian bytes to a 64-bits word.
/// This method overflows if the value is too big.
fn from_hex(hex_string: String) -> Result<u64, String> {
    match hex::decode(hex_string) {
        Ok(bytes) => {
            let value = bytes.iter().fold(0, |x, &i| x << 8 | i as u64);
            Ok(value)
        }
        Err(err) => Err(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex_test() {
        let value = from_hex("BEEF".to_string()).unwrap();
        assert_eq!(value, 48879);
    }
}
