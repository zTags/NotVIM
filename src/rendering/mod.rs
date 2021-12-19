use colored::*;

pub fn render(config: serde_json::Value) {
    let sizes = termsize::get().unwrap();
    let cols = sizes.cols;
    let rows = sizes.rows;

    let fg_r = &config["theme"]["bg"]["red"].as_u64().unwrap();
    let fg_g = &config["theme"]["bg"]["green"].as_u64().unwrap();
    let fg_b = &config["theme"]["bg"]["blue"].as_u64().unwrap();

    let bg_r = &config["theme"]["bg"]["red"].as_u64().unwrap();
    let bg_g = &config["theme"]["bg"]["green"].as_u64().unwrap();
    let bg_b = &config["theme"]["bg"]["blue"].as_u64().unwrap();

    for row in 0..rows {
        for col in 0..cols {
            print!("{}", " ".on_truecolor(*bg_r as u8, *bg_g as u8, *bg_b as u8));
        }
        
        if row < rows-1 {
            println!();
        }
    }

    loop {
        break;
    }
}
