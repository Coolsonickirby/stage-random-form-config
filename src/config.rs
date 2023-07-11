use std::collections::HashMap;
use crate::StageKind;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Entry {
    #[serde(default)]
    pub valid_types: Vec<StageKind>,
    #[serde(default = "HazardState::default")]
    pub hazard: HazardState
}

#[derive(Deserialize, Clone, Debug)]
pub enum HazardState {
    Off,
    On,
    Random,
    Default
}
impl HazardState {
    fn default() -> Self { HazardState::Default }
}

#[derive(Deserialize)]
struct UserConfig(pub HashMap<String, Entry>);

impl UserConfig {
    pub fn load() -> Option<UserConfig> {
        match std::fs::read_to_string("sd:/ultimate/stage-config.toml") {
            Ok(data) => {
                match toml::from_str(&data) {
                    Ok(res) => Some(res),
                    Err(err) => {
                        println!("[stage-random-form-config::UserConfig::load] Error! {:?}", err);
                        None
                    },
                }
            },
            Err(err) => {
                println!("[stage-random-form-config::UserConfig::load] Error! {:?}", err);
                None
            },
        }
    }
}

#[derive(Debug)]
pub struct DataConfig(pub HashMap<u64, Entry>);

impl DataConfig {
    pub fn load() -> Option<DataConfig> {
        match UserConfig::load() {
            Some(mut data) => {
                let mut new_map: HashMap<u64, Entry> = HashMap::new();
                for (key, value) in &data.0 {
                    let new_key;
                    if key.starts_with("0x") {
                        new_key = u64::from_str_radix(key.trim_start_matches("0x"), 16).unwrap();
                    } else if key.chars().all(char::is_numeric) {
                        new_key = key.parse::<u64>().unwrap();
                    } else {
                        new_key = arcropolis_api::hash40(key).as_u64();
                    }
                    new_map.insert(new_key, value.clone());
                }
                data.0.clear();
                Some(DataConfig(new_map))
            },
            None => None
        }
    }
}
