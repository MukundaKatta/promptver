# promptver

[![crates.io](https://img.shields.io/crates/v/promptver.svg)](https://crates.io/crates/promptver)

Hash and version prompt templates so eval results, cache keys, and
audit logs stay stable when whitespace changes.

```rust
use promptver::{hash, short_hash, changed};
let v1 = "System: be helpful.\nUser: {input}";
let v2 = "System: be helpful.   \r\nUser: {input}\n\n";
assert_eq!(hash(v1), hash(v2));      // trailing space + CRLF ignored
assert!(!changed(v1, v2));
assert_eq!(short_hash(v1).len(), 12); // 12-char ID for CI logs
```

Inlined SHA-256 — zero deps. MIT or Apache-2.0.
