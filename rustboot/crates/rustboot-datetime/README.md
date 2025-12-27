# rustboot-datetime

Timestamp and duration utilities.

```rust
use dev_engineeringlabs_rustboot_datetime::*;

// Current time
let now = now();
let millis = now_millis();

// Formatting
let s = format_timestamp(&now);
let parsed = parse_timestamp(&s)?;

// Duration
format_duration(Duration::from_secs(90)); // "1m 30s"
```

See [overview](docs/overview.md).
