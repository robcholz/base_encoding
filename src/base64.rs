const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64_encode(bytes: &[u8]) -> String {
    let mut ans = String::new();
    let mut chunks = bytes.chunks_exact(3);
    for chunk in &mut chunks {
        let mut bin = u32::from(chunk[0]) << 16 | u32::from(chunk[1]) << 8 | u32::from(chunk[2]);
        for _ in 0..4 {
            let bits = ((bin >> 18) & 0b111111) as u8;
            ans.push(char::from(TABLE[usize::from(bits)]));

            bin <<= 6;
        }
    }
    let rem = chunks.remainder();
    if !rem.is_empty() {
        let (cnt, mut bin) = match rem.len() {
            1 => (2, u32::from(rem[0]) << 16),
            2 => (3, u32::from(rem[0]) << 16 | u32::from(rem[1]) << 8),
            _ => unsafe {
                core::hint::unreachable_unchecked();
            },
        };
        for _ in 0..cnt {
            let bits = ((bin >> 18) & 0b111111) as u8;
            ans.push(char::from(TABLE[usize::from(bits)]));

            bin <<= 6;
        }
        for _ in cnt..4 {
            ans.push('=');
        }
    }

    debug_assert!(ans.len() % 4 == 0);
    ans
}

pub fn base64_decode(str: &str) -> Result<Vec<u8>, String> {
    if str.is_empty() {
        return Ok(Vec::new());
    }

    if str.len() % 4 != 0 {
        return Err("Invalid base64 string length".to_string());
    }

    let mut result = Vec::with_capacity(str.len() * 3 / 4);
    let mut chars = str.chars();

    let mut lookup = [0u8; 256];
    for (i, &c) in TABLE.iter().enumerate() {
        lookup[c as usize] = i as u8;
    }

    while let Some(a) = chars.next() {
        if a == '=' {
            return Err("Unexpected padding character".to_string());
        }

        let b = chars
            .next()
            .ok_or_else(|| "Unexpected end of string".to_string())?;
        if b == '=' {
            return Err("Unexpected padding character".to_string());
        }

        let c = chars
            .next()
            .ok_or_else(|| "Unexpected end of string".to_string())?;
        let d = chars
            .next()
            .ok_or_else(|| "Unexpected end of string".to_string())?;

        let a_val = lookup[a as usize];
        let b_val = lookup[b as usize];

        if a_val >= 64 || b_val >= 64 {
            return Err(format!(
                "Invalid base64 character found: '{}' or '{}'",
                a, b
            ));
        }

        result.push((a_val << 2) | (b_val >> 4));

        if c != '=' {
            let c_val = lookup[c as usize];
            if c_val >= 64 {
                return Err(format!("Invalid base64 character found: '{}'", c));
            }

            result.push((b_val << 4) | (c_val >> 2));

            if d != '=' {
                let d_val = lookup[d as usize];
                if d_val >= 64 {
                    return Err(format!("Invalid base64 character found: '{}'", d));
                }

                result.push((c_val << 6) | d_val);
            }
        } else if d != '=' {
            return Err("Invalid padding".to_string());
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode(&[0xde, 0xad, 0xbe, 0xef]), "3q2+7w==");
        assert_eq!(base64_encode(&[]), "");
        assert_eq!(
            base64_encode("Many hands make light work.".as_bytes()),
            "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu"
        );
    }

    #[test]
    fn test_base64_decode() {
        assert_eq!(base64_decode("3q2+7w=="), Ok(vec![0xde, 0xad, 0xbe, 0xef]));
        assert_eq!(base64_decode(""), Ok(vec![]));
        assert_eq!(
            base64_decode("TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu"),
            Ok("Many hands make light work.".as_bytes().to_vec())
        );
        assert!(base64_decode("3q2+7w").is_err());
        assert!(base64_decode("3q2+7w==").is_ok());
    }
}
