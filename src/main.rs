mod rendering;
use rendering::render;

use std::env::args;
use std::env::consts::OS;

use std::fs::{read_to_string, File, create_dir_all};
use std::path::Path;
use std::io::Write;

use serde_json::{Value, from_str};

fn main() {
    // TODO take in cli args (if any)
    let argv: Vec<String> = args().collect();
    if argv.len() != 1 {

    }

    // TODO load config
    let cfg_path = match OS {
        "windows" => "%appdata%/notvim/config.json",
        "macos"   => unimplemented!(),
        "linux"   => "/home/tags/.config/notvim/config.json",
        _         => unimplemented!(),
    };

    if !Path::new(cfg_path).exists() {
        create_dir_all("/home/tags/.config/notvim");
        let mut file = File::create(cfg_path).unwrap();
        file.write_all(b"{\n}").unwrap();
    }

    let config: Value = from_str(&read_to_string(cfg_path).unwrap()).expect("Failed to parse config.json");

    println!("{}", config);
    // TODO register commands

    // TODO rendering
    render(config);
}
