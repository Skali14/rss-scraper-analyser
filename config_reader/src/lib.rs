use config::Config;
use std::collections::HashMap;

pub fn read_value(value: &str) -> String {
    let settings = Config::builder()
        .add_source(config::File::with_name(std::env::current_exe().unwrap().parent().unwrap().join("config.toml").to_str().unwrap()))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let config_values = settings.try_deserialize::<HashMap<String, String>>().unwrap();

    let value_to_be_returned = config_values.get(value);

    match value_to_be_returned {
        Some(value) => value.to_owned(),
        None => String::from("empty")
    }
}