/*!
A Rust wrapper over the New Relic C SDK.

See also the [rocket_newrelic] crate for example integration with the
Rocket web framework.

---

Note: versions 0.1.0 onwards of this crate are completely incompatible
with previous versions as they move away from the deprecated New Relic SDK
to the newer New Relic C SDK. This has additional requirements: see
https://docs.newrelic.com/docs/agents/c-sdk/get-started/introduction-c-sdk
for details.

In particular, the New Relic SDK will not link against musl - see the [newrelic-sys] crate for more details.

See https://github.com/hjr3/newrelic-rs for the <0.1.0 repository.

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
new-relic = "0.2"
```

You can then instrument your code as follows:

```rust
use std::{env, thread, time::Duration};

use newrelic::{App, NewRelicConfig};

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

There are several more detailed examples in the [examples] directory of the
crate repository, demonstrating features such as simple and nested segments
and custom events.

This crate still requires the New Relic daemon to be running as per the
[documentation for the New Relic C SDK][c-sdk]; be sure to read this first.

[c-sdk]: https://docs.newrelic.com/docs/agents/c-sdk/get-started/introduction-c-sdk#architecture
[examples]: https://github.com/sd2k/newrelic/tree/master/examples
[newrelic-sys]: https://crates.io/crates/newrelic-sys
*/
//#![deny(missing_docs)]

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

pub mod acko_segment;
mod app;
mod error;
mod event;
pub mod newrelic_fn;
pub mod nr_connection;
pub mod nr_init;
mod segment;
pub mod skill;
pub mod tables;
pub mod transaction;

pub use log::Level as LogLevel;

pub use crate::app::{App, AppConfig, LogOutput, NewRelicConfig};
pub use crate::error::{Error, Result};
pub use crate::event::CustomEvent;
pub use crate::segment::{
    Datastore, DatastoreParams, DatastoreParamsBuilder, ExternalParams, ExternalParamsBuilder,
    Segment,
};
pub use crate::transaction::{Attribute, Transaction};

//use diesel::prelude::*;

//pub use crate::nr_connection::NRConnection;
