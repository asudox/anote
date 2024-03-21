use crate::structs::TOMLConfig;
use std::fs::File;
use std::io::Read;

/// This function is used to get the entire config.toml as a TOMLConfig struct.
pub fn get_config() -> TOMLConfig {
    let mut file = File::open("anote.toml").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let config: TOMLConfig = toml::from_str(&buf).unwrap();

    if config.anote.MINIMUM_ANIMELIST_SIZE < 10 {
        panic!("MINIMUM_ANIMELIST_SIZE must be at least 10");
    } else if config.anote.CACHED_GENRE_COMBO_EXPIRATION_TIME == 0 {
        panic!("CACHED_GENRE_COMBO_EXPIRATION_TIME must be at least 1");
    }

    config
}
