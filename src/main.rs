use ansi_term::Color;
use linefeed::{Interface, ReadResult};

use crate::lexing::lexer::Lexer;

mod lexing;
mod parsing;

#[cfg(not(tarpaulin_include))]
fn main() {
    let interface = Interface::new("tinysh").unwrap();
    let style = Color::Cyan;
    let prompt_text = "tinysh> ";
    let mut verbose = true;

    println!(
        "{}",
        Color::Blue.paint("Welcome to tinysh 0.0.3 by Charlotte Thomas")
    );

    interface
        .set_prompt(&format!(
            "\x01{prefix}\x02{text}\x01{suffix}\x02",
            prefix = style.prefix(),
            text = prompt_text,
            suffix = style.suffix()
        ))
        .unwrap();
    
    while let ReadResult::Input(line) = interface.read_line().unwrap() {
        match line.as_str().trim() {
            "exit" => break,
            "verbose" => {
                verbose = !verbose;
                println!(
                    "{} {}",
                    Color::Purple.paint(" You toggled verbose to"),
                    Color::Yellow.paint(if verbose { "on" } else { "off" })
                )
            }
            "info" => {
                println!("{}",Color::Purple.paint(" Tinysh v0.0.3\n By Charlotte Thomas\n Repository: https://github.com/tinysh"))
            }
            _ => {
                let lexer = Lexer { str: line.clone() };
                println!("{:?}", lexer.lex());
            }
        }
        interface.add_history_unique(line);
    }
    println!("{}", Color::Blue.paint("Exiting tinysh, goodbye :)"));
}
