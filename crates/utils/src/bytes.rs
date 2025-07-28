use anyhow::{Result, anyhow};

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>> {
    hex::decode(hex.strip_prefix("0x").unwrap_or(hex))
        .map_err(|e| anyhow!("Failed to decode hex: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_hex() {
        let test_cases = vec![
            (vec![], "0x"),
            (vec![0], "0x00"),
            (vec![255], "0xff"),
            (vec![1, 2, 3, 4], "0x01020304"),
            (vec![255, 255, 255, 255], "0xffffffff"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(bytes_to_hex(&input), expected);
        }
    }

    #[test]
    fn test_hex_to_bytes() {
        let test_cases = vec![
            ("0x", vec![]),
            ("0x00", vec![0]),
            ("0xff", vec![255]),
            ("0x01020304", vec![1, 2, 3, 4]),
            ("0xffffffff", vec![255, 255, 255, 255]),
            // 测试不带0x前缀的情况
            ("", vec![]),
            ("00", vec![0]),
            ("ff", vec![255]),
            ("01020304", vec![1, 2, 3, 4]),
        ];

        for (input, expected) in test_cases {
            assert_eq!(hex_to_bytes(input).unwrap(), expected);
        }
    }

    #[test]
    fn test_hex_to_bytes_invalid() {
        let invalid_hexes = vec![
            "0xinvalid",
            "invalid",
            "0x12345", // 奇数长度
            "12345",   // 奇数长度
        ];

        for invalid_hex in invalid_hexes {
            assert!(hex_to_bytes(invalid_hex).is_err());
        }
    }

    #[test]
    fn test_roundtrip() {
        let test_bytes = vec![
            vec![],
            vec![0],
            vec![255],
            vec![1, 2, 3, 4],
            vec![255, 255, 255, 255],
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        ];

        for bytes in test_bytes {
            let hex = bytes_to_hex(&bytes);
            let decoded = hex_to_bytes(&hex).unwrap();
            assert_eq!(bytes, decoded);
        }
    }
}
