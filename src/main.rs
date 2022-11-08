use std::fs;
use std::error::Error;
use std::env;
use std::process::exit;

enum Mode {
    HeadingOne,
    HeadingTwo,
    HeadingThree,
    HeadingFour,
    HeadingFive,
    HeadingSix,
    Bold,
    Italic,
    Qoute,
}

struct Settings {
    filename: String,
}

fn print_help_and_exit() {
    println!("Help currently not available");
    exit(0);
}

fn settings_from_args() -> Option<Settings> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        return None;
    }

    Some(Settings {
        filename: args.get(args.len()-1)
                      .unwrap()
                      .to_string(),
    })
}

fn main() -> Result<(), Box<dyn Error>>{
    let settings: Settings;
    let mut file_content_reverse: Vec<char>;
    let mut rendered_text: String;
    let mut current_char: char;

    settings = match settings_from_args() {
        Some(result) => result,
        None => { 
            print_help_and_exit();
            Settings { filename: "".to_string() }
        }
    };

    rendered_text = String::new();
    file_content_reverse = fs::read_to_string(settings.filename)?
                              .chars()
                              .rev()
                              .collect::<Vec<char>>();

    loop {
        if file_content_reverse.len() < 1 { break; }

        current_char = match file_content_reverse.pop() {
            Some(c) => c,
            None => ' ',
        };

        rendered_text.push(current_char);
    }

    println!("{}", rendered_text);
    Ok(())
}
