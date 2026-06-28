use sha2::{Digest, Sha256};

/// Compute the Xiaomi unlock code for a device.
///
/// The algorithm upper-cases both `mac` and `sn`, strips common
/// separators from the MAC address, then hashes
/// `mac + sn + "XIAOMI"` with SHA-256. The first 10 bytes of the
/// digest are taken modulo 0xA and concatenated as decimal digits.
pub fn calc_unlock_code(mac: &str, sn: &str) -> String {
    let mac = mac
        .to_uppercase()
        .replace('：', "") // Chinese full-width colon
        .replace(':', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('.', "");
    let sn = sn.to_uppercase().trim().to_string();

    let mut hasher = Sha256::default();
    hasher.update(mac);
    hasher.update(sn);
    hasher.update("XIAOMI");

    let hash = hasher.finalize();

    let mut code = String::with_capacity(10);
    for byte in hash.iter().take(10) {
        let digit = byte % 0xA;
        code.push_str(&digit.to_string());
    }

    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_case() {
        // The hash is deterministic for a given input pair.
        let code = calc_unlock_code("00:11:22:33:44:55", "SN123456789");
        assert_eq!(code.len(), 10);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
        // Calling again with differently-cased/separated input must
        // yield the same code.
        assert_eq!(
            code,
            calc_unlock_code("00-11-22-33-44-55", "sn123456789")
        );
    }
}
