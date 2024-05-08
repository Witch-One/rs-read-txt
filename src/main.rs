use clap::Parser;
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal;
use encoding_rs::ISO_8859_10;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
#[derive(Parser, Debug)]
#[command(version, about)]
struct ReadText {
    #[arg(short, long)]
    file: String,
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let args = ReadText::parse();
    let path_str = args.file;
    println!("path_str: {path_str}");
    let path = PathBuf::from(path_str);
    println!("path: {:?}", path);
    if !bool::from(path.exists()) {
        println!("can't find text");

        return;
    }
    terminal::enable_raw_mode().unwrap();
    let mut txt_content = String::new();
    match fs::read(path) {
        Ok(_content) => {
            let (decoded, _, _) = ISO_8859_10.decode(&_content);
            txt_content = decoded.into_owned();
        }
        Err(err) => {
            println!("& color:red {}", err);
            return;
        }
    }

    let mut last_line = 0;

    let lines = txt_content.split('\n').collect::<Vec<&str>>();

    let line = lines[last_line];

    println!("line:{}", line);

    loop {
        if poll(Duration::from_millis(100)).unwrap() {
            let event = read().unwrap();
            match event {
                Event::Key(key_event) => {
                    // print!("KeyEvent:{:?}", key_event);
                    match key_event.code {
                        KeyCode::Down => {
                            if last_line < lines.len() - 1 {
                                clear_screen();
                                last_line += 1;
                                let line = lines[last_line];
                                println!("line:{}", line);
                            }
                        }
                        KeyCode::Up => {
                            if last_line > 0 {
                                clear_screen();
                                last_line -= 1;
                                let line = lines[last_line];
                                println!("line:{}", line);
                            }
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
