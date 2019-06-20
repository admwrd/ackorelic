use std::env;
use crate::App;

lazy_static! {
    pub static ref NR_APP: App = {
        let license_key = env::var("NEW_RELIC_LICENSE_KEY").unwrap_or_else(|_| "example-license-key".to_string());
        let app_name = env::var("NEW_RELIC_APP_NAME").unwrap_or_else(|_| "acko_test".to_string());
        let app = App::new(&app_name, &license_key).expect("Could not create app");
        app
    };
}