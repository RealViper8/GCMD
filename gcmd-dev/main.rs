use crossterm::{execute, style::{SetBackgroundColor, SetForegroundColor}};
use terminal::LOGO;
use std::{io::{self, Write}, path::Path};

use tokio;

#[tokio::main(flavor="current_thread")]
async fn main() {
    execute!(io::stdout(), SetForegroundColor(crossterm::style::Color::Cyan)).unwrap();
    clear().await;
    println!("\t\t\x1b[1;35m--- \x1b[0;32mGCMD \x1b[0;36mDEV \x1b[1;35m---");

    let mut input: String = String::new();
    let mut cd = std::env::current_dir().unwrap().display().to_string();

    loop {
        print!("\x1b[0;36m{}\x1b[1;32m$\x1b[1;35m:\x1b[0;32m ", std::env::current_dir().unwrap().display());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            continue;
        }

        if input.trim().split_whitespace().count() >= 2 {
            match (input.trim().split_whitespace().nth(0).unwrap(), input.trim().split_whitespace().nth(1).unwrap()) {
                ("cd", path) if Path::new(path).exists() => {
                    if input.trim().split_whitespace().nth(1).unwrap() == ".." {
                        cd = std::env::current_dir().unwrap().parent().unwrap().display().to_string();
                    } else {
                        cd = std::env::current_dir().unwrap().join(input.trim().split_whitespace().nth(1).unwrap()).display().to_string();
                    }
                    std::env::set_current_dir(&cd).unwrap();
                }
                _ => {}
            }
        } else if input.trim().split_whitespace().count() == 1 {
            match input.trim().split_whitespace().nth(0).unwrap() {
                "help" | "?" => {
                    println!("\x1b[1;34mAvailable commands:\x1b[0m");
                    println!("\x1b[0;32mhelp, ?       \x1b[0m- Displays this help menu.");
                    println!("\x1b[0;32mneofetch, sysinfo\x1b[0m - Shows system information.");
                    println!("\x1b[0;32mcls, clear\x1b[0m - Clears terminal.");
                    println!("\x1b[0;32mls, dir\x1b[0m - Lists all files in current directory.");
                    println!("\x1b[0;32mexit, quit    \x1b[0m- Exits the application.");
                    println!();
                }
                "neofetch" | "sysinfo" => {
                    println!("{}", LOGO);
                }
                "exit" | "quit" => {
                    break;
                }
                "cls" | "clear" => {
                    clear().await;
                }
                "ls" | "dir" => {
                    // println!("{}", std::env::current_dir().unwrap().display());
                    println!();
                    for entry in std::fs::read_dir(std::env::current_dir().unwrap()).unwrap() {
                        println!("{}", entry.unwrap().file_name().to_str().unwrap());
                    }
                    println!();
                }
                _ => {
                    println!("\x1b[0;31mCommand not found: '{}'\x1b[0m\n", input.trim());
                }
            }
        }

        input = String::new();
    }

    print!("\x1b[0m");
    io::stdout().flush().unwrap();  
    execute!(io::stdout(), SetForegroundColor(crossterm::style::Color::Reset), SetBackgroundColor(crossterm::style::Color::Reset)).unwrap();
}

#[cfg(target_os="windows")]
async fn clear() {
    std::process::Command::new("cmd").args(["/c","cls"]).status().unwrap();
}

#[cfg(target_os="linux")]
async fn clear() {
    std::process::Command::new("bash").args(["-c","clear"]).status().unwrap();
}