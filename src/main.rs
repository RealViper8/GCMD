use crossterm::execute;
use crossterm::style::{SetBackgroundColor, SetForegroundColor};
use mlua::Lua;
use terminal::{get_gpu_info, init, LOGO};
use std::path::Path;
use std::{fs, io};
use std::io::prelude::Write;
use std::process::Command;

fn main() {
    let lua = Lua::new();
    let mut input = String::new();
    let gpu = get_gpu_info();
    let gpu_info = if gpu == None {String::from("N/A")} else { gpu.unwrap().to_owned() };
    let fetchlogo = LOGO.replace("{os}", &std::env::consts::OS.to_uppercase()).replace("{arch}", std::env::consts::ARCH).replace("{cpu}", num_cpus::get().to_owned().to_string().as_str()).replace("{mem}", std::mem::size_of::<u64>().to_string().as_str()).replace("{gpu}", &gpu_info as &str);
    let mut cd = std::env::current_dir().unwrap().display().to_string();
    init();

    loop {
        print!("\x1b[0;36m{}\x1b[1;32m$\x1b[1;35m:\x1b[0;32m ", cd);
        //print!("\x1b[0;36m>\x1b[0;32m ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            0
        });
        
        if input.trim().is_empty() {
            continue;
        }

        if input.trim().split_whitespace().nth(1) != None {
            match (input.trim().split_whitespace().nth(0).unwrap(), input.trim().split_whitespace().nth(1).unwrap()) {
                ("cd", path) => {
                    if Path::new(path).exists() {
                        std::env::set_current_dir(path).unwrap();
                    } else {
                        println!("\x1b[0;31mDirectory not found.\x1b[0m");
                    }
                }
                _ => ()
            }
        } else {
            match input.trim().split_whitespace().nth(0).unwrap() {
                "help" | "?" => {
                    println!("\x1b[1;34mAvailable commands:\x1b[0m");
                    println!("\x1b[0;32mhelp, ?       \x1b[0m- Displays this help menu.");
                    println!("\x1b[0;32mneofetch, sysinfo\x1b[0m - Shows system information.");
                    println!("\x1b[0;32mcls, clear\x1b[0m - Clears terminal.");
                    println!("\x1b[0;32mls, dir\x1b[0m - Lists all files in current directory.");
                    println!("\x1b[0;32msh, shell \x1b[0m - Opens the operating system shell.");
                    println!("\x1b[0;32mmods, modules \x1b[0m - Lists all the available mods for games.");
                    println!("\x1b[0;32mwifi \x1b[0m - Lists all the saved wifi networks.");
                    println!("\x1b[0;32mwifi <ssid> \x1b[0m - Lists the password of a saved wifi network.");
                    println!("\x1b[0;32mexit, quit    \x1b[0m- Exits the application.");
                    println!();
                }
                "wifi" | "wlan" => {
                    #[cfg(not(target_os="windows"))]
                    let networks = Command::new("nmcli")
                        .arg("dev")
                        .arg("wifi")
                        .arg("list")
                        .output()
                        .expect("failed to execute process");

                    #[cfg(target_os="windows")]
                    let networks = Command::new("netsh")
                        .arg("wlan")
                        .arg("show")
                        .arg("profiles")
                        .output()
                        .expect("failed to execute process");

                    #[cfg(not(target_os="windows"))]
                    let networks = String::from_utf8(networks.stdout).unwrap();
                    println!("{:#?}", networks);
                    println!();
                }
                "osh" | "shell" => {
                    #[cfg(target_os="windows")]
                    let status = Command::new("cmd")
                        .status()
                        .expect("failed to execute process");

                    #[cfg(not(target_os="windows"))]
                    let status = Command::new("sh")
                        .status()
                        .expect("failed to execute process");

                    if !status.success() {
                        eprintln!("\x1b[0;31mShell exited with non-zero status.\x1b[0m");
                    }
                    println!();
                }
                "ls" | "dir" => {
                    let paths = fs::read_dir("./").unwrap();
                    for path in paths {
                        println!("\x1b[0;32m{}", path.unwrap().path().display().to_string().replace("./", ""));
                    }
                    println!();
                }
                "neofetch" | "sysinfo" => {
                    println!("{}", fetchlogo);
                }
                "exit" | "quit" => {
                    println!("\x1b[0;32mExiting...\x1b[0m");
    
                    match lua.load("require('modules.terminal').exit()").exec() {
                        Ok(_) => (),
                        Err(err) => println!("{}", err)
                    }
                    break;
                }
                "cls" | "clear" => {
                    terminal::clear();
                }
                _ => ()
            }
        }

        cd = std::env::current_dir().unwrap().display().to_string();
        input = String::new();
    }

    print!("\x1b[0m");
    io::stdout().flush().unwrap();
    execute!(io::stdout(), SetForegroundColor(crossterm::style::Color::Reset), SetBackgroundColor(crossterm::style::Color::Reset)).unwrap();
}