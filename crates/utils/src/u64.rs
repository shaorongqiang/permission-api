use anyhow::{Result, anyhow};

pub fn u64_to_bytes(u: u64) -> Vec<u8> {
    u.to_be_bytes().to_vec()
}

pub fn bytes_to_u64(bytes: &[u8]) -> Result<u64> {
    let bytes = bytes.try_into().map_err(|e| anyhow!("{}", e))?;
    Ok(u64::from_be_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u64_to_bytes() {
        let test_cases = vec![
            (0u64, vec![0, 0, 0, 0, 0, 0, 0, 0]),
            (1u64, vec![0, 0, 0, 0, 0, 0, 0, 1]),
            (255u64, vec![0, 0, 0, 0, 0, 0, 0, 255]),
            (256u64, vec![0, 0, 0, 0, 0, 0, 1, 0]),
            (u64::MAX, vec![255, 255, 255, 255, 255, 255, 255, 255]),
        ];

        for (input, expected) in test_cases {
            assert_eq!(u64_to_bytes(input), expected);
        }
    }

    #[test]
    fn test_bytes_to_u64() {
        let test_cases = vec![
            (vec![0, 0, 0, 0, 0, 0, 0, 0], 0u64),
            (vec![0, 0, 0, 0, 0, 0, 0, 1], 1u64),
            (vec![0, 0, 0, 0, 0, 0, 0, 255], 255u64),
            (vec![0, 0, 0, 0, 0, 0, 1, 0], 256u64),
            (vec![255, 255, 255, 255, 255, 255, 255, 255], u64::MAX),
        ];

        for (input, expected) in test_cases {
            assert_eq!(bytes_to_u64(&input).unwrap(), expected);
        }
    }

    #[test]
    fn test_bytes_to_u64_invalid_length() {
        // 测试长度不足的情况
        let short_bytes = vec![1, 2, 3, 4];
        assert!(bytes_to_u64(&short_bytes).is_err());

        // 测试长度过长的情况
        let long_bytes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert!(bytes_to_u64(&long_bytes).is_err());
    }

    #[test]
    fn test_roundtrip() {
        let test_values = vec![0, 1, 255, 256, 65535, 4294967295, u64::MAX];

        for value in test_values {
            let bytes = u64_to_bytes(value);
            let decoded = bytes_to_u64(&bytes).unwrap();
            assert_eq!(value, decoded);
        }
    }
}
