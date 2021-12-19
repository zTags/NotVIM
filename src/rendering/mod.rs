use colored::*;

use std::io::{self, Write};

pub fn render(config: serde_json::Value) {
    let sizes = termsize::get().unwrap();
    let cols = sizes.cols as usize;
    let rows = sizes.rows as usize;

    let fg_r = *(&config["theme"]["fg"]["red"].as_u64().unwrap()) as u8;
    let fg_g = *(&config["theme"]["fg"]["green"].as_u64().unwrap()) as u8;
    let fg_b = *(&config["theme"]["fg"]["blue"].as_u64().unwrap()) as u8;

    let bg_r = *(&config["theme"]["bg"]["red"].as_u64().unwrap()) as u8;
    let bg_g = *(&config["theme"]["bg"]["green"].as_u64().unwrap()) as u8;
    let bg_b = *(&config["theme"]["bg"]["blue"].as_u64().unwrap()) as u8;

    let demo_text = "hey\nhi o/\nhru?\ngreat, u?\nim also good thanks for asking";

    let demo_text_at_newlines: Vec<&str> = demo_text.split("\n").collect();

    let mut top_row: u32 = 0;

    for row_ in 0..rows {
        let row = row_ as usize;
        if row < demo_text_at_newlines.len() {
            print!("{}", demo_text_at_newlines[row].on_truecolor(bg_r, bg_g, bg_b).truecolor(fg_r, fg_g, fg_b));
            for col in 0..(cols - demo_text_at_newlines[row].len()) {
                print!("{}", " ".on_truecolor(bg_r, bg_g, bg_b).truecolor(fg_r, fg_g, fg_b));
            }
        } else {
            if row < rows - 2 {
                print!("{}", "~".on_truecolor(bg_r, bg_g, bg_b).truecolor(fg_r, fg_g, fg_b));
                for col in 0..cols - 1 {
                    print!("{}", " ".on_truecolor(bg_r, bg_g, bg_b).truecolor(fg_r, fg_g, fg_b));
                }
            } else {
                print!("{}", "status".on_truecolor(fg_r, fg_g, fg_b).truecolor(bg_r, bg_g, bg_b));
                for col in 0..cols - "status".len() {
                    print!("{}", " ".on_truecolor(fg_r, fg_g, fg_b).truecolor(bg_r, bg_g, bg_b));
                }
                print!("{}", "command".on_truecolor(bg_r, bg_g, bg_b).truecolor(fg_r, fg_g, fg_b));
                for col in 0..cols - "command".len() {
                    print!("{}", " ".on_truecolor(bg_r, bg_g, bg_b).truecolor(fg_r, fg_g, fg_b));
                }
                io::stdout().flush();
                break;
            }
        }
    }

    loop { }
}
