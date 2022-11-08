use std::fs;
use std::env;
use std::error::Error;
use std::process::exit;

struct Settings {
    filename: String,
}

struct Mode {
    heading_level: usize,
    active_star_symbols: usize,
    last_char: char,
    bold: bool,
    italic: bool,
    quote: bool,
}

impl Mode {
    pub fn incr_heading_level(&mut self) {
        if self.heading_level < 6 {
            self.heading_level += 1;
        }
    }

    pub fn set_quote(&mut self) {
        self.quote = true;
    }

    pub fn handle_newline(&mut self) {
        self.heading_level = 0;

        if self.last_char == '\n' {
            self.quote = false;
        }
    }

    pub fn change_bold_italic(&mut self) {
        self.active_star_symbols += 1;

        if self.active_star_symbols == 2
        && self.last_char != '*' {
            self.italic = false;
            self.bold   = false;
            self.active_star_symbols = 0;
        }

        match self.active_star_symbols {
            1 => { self.italic = true;
                   self.bold   = false; },
            2 => { self.italic = false;
                   self.bold   = true;  },
            4 => { self.bold   = false; },
            _ => {}
        }
    }
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

fn update_modes(mut mode: Mode, c: char) -> Mode {
    if c == '#' {
        mode.incr_heading_level();

    } else if c == '*' {
        mode.change_bold_italic();

    } else if c == '>' {
        mode.set_quote();

    } else if c == '\n' {
        mode.handle_newline();
    }

    mode.last_char = c;
    mode
}

fn main() -> Result<(), Box<dyn Error>>{
    let settings: Settings;
    let mut write_mode: Mode;
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

    write_mode = Mode {
        heading_level: 0,
        active_star_symbols: 0,
        last_char: '\0',
        bold: false,
        italic: false,
        quote: false,
    };

    loop {
        if file_content_reverse.len() < 1 { break; }

        current_char = match file_content_reverse.pop() {
            Some(c) => c,
            None => ' ',
        };

        write_mode = update_modes(write_mode, current_char);
        rendered_text.push(current_char);
    }

    println!("{}", rendered_text);
    Ok(())
}
