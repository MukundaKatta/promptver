use promptver::{changed, hash, short_hash};

#[test]
fn whitespace_normalization() {
    let v1 = "System: be helpful.\nUser: {input}";
    let v2 = "System: be helpful.   \r\nUser: {input}\n\n";
    assert_eq!(hash(v1), hash(v2));
    assert!(!changed(v1, v2));
}

#[test]
fn substantive_change_changes_hash() {
    let v1 = "be brief";
    let v2 = "be terse";
    assert_ne!(hash(v1), hash(v2));
    assert!(changed(v1, v2));
}

#[test]
fn short_hash_is_12_chars() {
    let h = short_hash("anything");
    assert_eq!(h.len(), 12);
    assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn hash_is_64_hex_chars() {
    let h = hash("anything");
    assert_eq!(h.len(), 64);
    assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn known_sha256_after_normalize() {
    // "abc" normalizes to "abc" (no trailing ws, no BOM, no CRLF).
    // SHA-256("abc") = ba7816bf...
    assert!(hash("abc").starts_with("ba7816bf"));
}

#[test]
fn bom_stripped() {
    let v1 = "abc";
    let v2 = "\u{FEFF}abc";
    assert_eq!(hash(v1), hash(v2));
}

#[test]
fn collapses_blank_line_runs() {
    let v1 = "line\n\nline";
    let v2 = "line\n\n\n\n\nline";
    assert_eq!(hash(v1), hash(v2));
}
