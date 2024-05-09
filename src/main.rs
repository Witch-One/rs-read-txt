use clap::Parser;
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal;
use encoding_rs::ISO_8859_10;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use toml::{self};
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
    let path = PathBuf::from(path_str.clone());
    println!("path: {:?}", path);
    if !bool::from(path.exists()) {
        println!("can't find text");

        return;
    }
    terminal::enable_raw_mode().unwrap();
    let mut txt_content = String::new();

    let config = match fs::read_to_string("../../src/config.toml") {
        Ok(content) => content,
        Err(_) => {
            println!("Error: Failed to read config file.");
            return;
        }
    };

    let mut config_toml: toml::Value = match toml::from_str(&config) {
        Ok(value) => value,
        Err(err) => {
            println!("Error: Failed to parse config file: {}", err);
            return;
        }
    };
    let update_config = {
        let path_str = path_str.clone();

        move |line_count: usize| {
            let mut config = toml::map::Map::new();

            config.insert(path_str.clone(), toml::Value::Integer(line_count as i64));

            // 将 Value 转换为 TOML 格式的字符串
            let toml_string = match toml::to_string(&config) {
                Ok(s) => s,
                Err(_) => {
                    println!("Error: Failed to serialize data to TOML format.");
                    return;
                }
            };

            // 将 TOML 字符串写入到配置文件
            match fs::write("../../src/config.toml", toml_string) {
                Ok(()) => {}
                Err(_) => println!("Error: Failed to write config file."),
            };
        }
    };

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

    let mut last_line: usize = match config_toml.get(path_str.clone()) {
        Some(value) => value.as_integer().unwrap() as usize,
        None => 0,
    };

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
                                update_config(last_line);
                                let line = lines[last_line];
                                println!("line{}:{}", last_line, line);
                            }
                        }
                        KeyCode::Up => {
                            if last_line > 0 {
                                clear_screen();
                                last_line -= 1;
                                update_config(last_line);
                                let line = lines[last_line];
                                println!("line{}:{}", last_line, line);
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
