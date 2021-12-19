mod rendering;
use rendering::render;

use std::env::args;
use std::env::consts::OS;

use std::fs::{read_to_string, File, create_dir_all};
use std::path::Path;
use std::io::Write;

use serde_json::{Value, from_str};

use dirs::home_dir;
// TODO implement
// As a module doesn't seem too viable.
use discord_rich_presence::{activity, new_client, DiscordIpc};

fn main() {
    // TODO take in cli args (if any)
    let argv: Vec<String> = args().collect();
    if argv.len() != 1 {

    }

    // TODO load config
    // Feel like we'll need this at some point.
    let root_dir = home_dir().unwrap().join(".notvim");

    // Basically before but universal.
    let cfg_path = &root_dir.join("config.json");

    if !Path::new(cfg_path).exists() {
        create_dir_all(root_dir);
        let mut file = File::create(cfg_path).unwrap();
        file.write_all(b"{\n}").unwrap();
    }

    let config: Value = from_str(&read_to_string(cfg_path).unwrap()).expect("Failed to parse config.json");
    // TODO register plugins

    // TODO rendering
    render(config);
}
