New Relic SDK
=============

An idiomatic Rust wrapper around the New Relic C SDK.

Usage
-----

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
ackorelic = "*"
```

You can then instrument your code as follows:

```rust
use std::{env, thread, time::Duration};

use ackorelic::{App, NewRelicConfig};

fn main() {
    let license_key =
        env::var("NEW_RELIC_LICENSE_KEY").unwrap_or_else(|_| "example-license-key".to_string());
    let app = App::new("my app", &license_key).expect("Could not create app");

    // Start a web transaction and a segment
    let _transaction = app
        .web_transaction("Transaction name")
        .expect("Could not start transaction");
    thread::sleep(Duration::from_secs(1));

    // Transaction ends automatically.

    // App is destroyed automatically.
}
```

See the examples directory of the repository for more complex examples, including segments (custom, datastore and external) and further configuration.

This crate still requires the New Relic daemon to be running as per the [documentation for the New Relic C SDK][c-sdk]; be sure to read this first.

Functionality
-------------

The core functionality from the C SDK is currently implemented. A few extra things are still TODO!

* [ ] Transactions
    * [x] Adding attributes
    * [x] Noticing errors
    * [x] Ignoring transactions
    * [ ] Overriding timings
* [x] Segments
    * [x] Custom
    * [x] Datastore
    * [x] External
    * [x] Nesting segments
    * [ ] Overriding timings
* [x] Custom events
* [x] Custom metrics
* [ ] Transaction tracing configuration
* [ ] Datastore segment tracing configuration
* [x] Logging/daemon configuration

Failures
--------

Currently, creating a transaction using this library returns a `Result<newrelic::Transaction, newrelic::Error>`, making it up to the user to either fail hard, or ignore, when transactions fail to be created.

However, when working with Segments in the library, the failure of the segment is hidden from the user for ergonomic purposes. That is, in the following example, creation of the segment might fail (which will be logged using the `log` crate), but the argument passed to the custom segment closure is still of type `newrelic::Segment`. This makes it much simpler to work with nested segments.

This behaviour may be changed in future, if it proves to be unpopular.

```rust
use ackorelic::{App, NewRelicConfig};

fn main() {
    let app =
        App::new("my app", "my license key").expect("Could not create app");
    let transaction = app
        .web_transaction("Transaction name")
        .expect("Could not start transaction");
    let expensive_value = transaction.custom_segment("Segment name", "Segment category", |seg| {
        do_something_expensive()
    });
}
```

[c-sdk]: https://docs.newrelic.com/docs/agents/c-sdk/get-started/introduction-c-sdk#architecture
[newrelic-sys]: https://crates.io/crates/newrelic-sys
