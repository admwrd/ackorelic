use std::env;
use crate::App;

lazy_static! {
    pub static ref NR_APP: App = {
        let license_key = env::var("NEW_RELIC_LICENSE_KEY").unwrap_or_else(|_| "df9713ab7d366e521d6b4a29b1971ee45b8415c4".to_string());
        let app_name = env::var("NEW_RELIC_APP_NAME").unwrap_or_else(|_| "acko_api_test".to_string());
        let app = App::new(&app_name, &license_key).expect("Could not create app");
        app
    };
}