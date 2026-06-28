use sha2::{Digest, Sha256};

fn normalize_mac(mac: &str) -> String {
    mac.to_uppercase()
        .replace('：', "") // Chinese full-width colon
        .replace(':', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('.', "")
}

fn normalize_sn(sn: &str) -> String {
    sn.to_uppercase().trim().to_string()
}

/// Compute the Xiaomi unlock code for a device.
///
/// The algorithm upper-cases both `mac` and `sn`, strips common
/// separators from the MAC address, then hashes
/// `mac + sn + "XIAOMI"` with SHA-256. The first 10 bytes of the
/// digest are taken modulo 0xA and concatenated as decimal digits.
///
/// When `new_algorithm` is `true`, the insertion order is swapped to
/// `sn + mac + "XIAOMI"`, which is reported to work for S5 / 10P and
/// newer devices.
pub fn calc_unlock_code(mac: &str, sn: &str, new_algorithm: bool) -> String {
    let mac = normalize_mac(mac);
    let sn = normalize_sn(sn);

    let mut hasher = Sha256::default();
    if new_algorithm {
        hasher.update(&sn);
        hasher.update(&mac);
    } else {
        hasher.update(&mac);
        hasher.update(&sn);
    }
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
        let code = calc_unlock_code("00:11:22:33:44:55", "SN123456789", false);
        assert_eq!(code.len(), 10);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
        // Calling again with differently-cased/separated input must
        // yield the same code.
        assert_eq!(
            code,
            calc_unlock_code("00-11-22-33-44-55", "sn123456789", false)
        );
    }

    #[test]
    fn new_algorithm_differs() {
        let old = calc_unlock_code("00:11:22:33:44:55", "SN123456789", false);
        let new = calc_unlock_code("00:11:22:33:44:55", "SN123456789", true);
        assert_ne!(old, new);
        assert_eq!(new.len(), 10);
    }
}
