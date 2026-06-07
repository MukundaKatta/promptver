//! # promptver
//!
//! Hash and version prompt templates so eval results, cache keys, and
//! audit logs stay stable when templates change.
//!
//! - [`hash`] returns a 64-char lowercase SHA-256 hex digest of the
//!   normalized template.
//! - [`short_hash`] returns the first 12 hex chars (enough for human
//!   eyeballing in CI output).
//! - [`changed`] tells you whether two templates would produce different
//!   hashes once normalized.
//!
//! Normalization: trim trailing whitespace per line, collapse runs of
//! blank lines, strip BOM, normalize CRLF→LF. Substantive changes
//! still produce different hashes; pure whitespace edits do not.
//!
//! ## Example
//!
//! ```
//! use promptver::{hash, short_hash, changed};
//! let v1 = "System: be helpful.\nUser: {input}";
//! let v2 = "System: be helpful.   \r\nUser: {input}\n\n";
//! // Trailing space and CRLF do not change the hash.
//! assert_eq!(hash(v1), hash(v2));
//! assert!(!changed(v1, v2));
//! assert_eq!(short_hash(v1).len(), 12);
//! ```

#![deny(missing_docs)]

mod sha256;

/// 64-char hex SHA-256 of the normalized template.
pub fn hash(template: &str) -> String {
    sha256::hex(normalize(template).as_bytes())
}

/// First 12 hex chars of `hash`.
pub fn short_hash(template: &str) -> String {
    let h = hash(template);
    h[..12].to_string()
}

/// True when the two templates produce different hashes after
/// normalization.
pub fn changed(a: &str, b: &str) -> bool {
    hash(a) != hash(b)
}

fn normalize(s: &str) -> String {
    let s = s.strip_prefix('\u{FEFF}').unwrap_or(s);
    let s = s.replace("\r\n", "\n").replace('\r', "\n");
    // Trim trailing whitespace on each line.
    let mut out = String::with_capacity(s.len());
    for (i, line) in s.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }
        let stripped = line.trim_end_matches([' ', '\t']);
        out.push_str(stripped);
    }
    // Collapse runs of blank lines to a single blank line.
    let mut collapsed = String::with_capacity(out.len());
    let mut prev_blank = false;
    for line in out.split('\n') {
        let is_blank = line.is_empty();
        if is_blank && prev_blank {
            continue;
        }
        if !collapsed.is_empty() {
            collapsed.push('\n');
        }
        collapsed.push_str(line);
        prev_blank = is_blank;
    }
    // Trim leading/trailing blank lines for stability.
    collapsed.trim_matches('\n').to_string()
}
