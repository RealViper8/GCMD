use crossterm::execute;
use crossterm::style::{SetBackgroundColor, SetForegroundColor};
use terminal::{get_gpu_info, init, LOGO};
use std::io;
use std::io::prelude::Write;


fn main() {
    let mut input = String::new();
    let gpu = get_gpu_info();
    let gpu_info = if gpu == None {String::from("N/A")} else { gpu.unwrap().to_owned() };
    let fetchlogo = LOGO.replace("{os}", &std::env::consts::OS.to_uppercase()).replace("{arch}", std::env::consts::ARCH).replace("{cpu}", num_cpus::get().to_owned().to_string().as_str()).replace("{mem}", std::mem::size_of::<u64>().to_string().as_str()).replace("{gpu}", &gpu_info as &str);
    init();

    loop {
        print!("\x1b[0;36m>\x1b[0;32m ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            0
        });
        
        if input.trim().is_empty() {
            continue;
        }

        match input.trim().split_whitespace().nth(0).unwrap() {
            "neofetch" | "sysinfo" => {
                println!("{}", fetchlogo);
            }
            "exit" | "quit" => {
                break;
            }
            "cls" | "clear" => {
                terminal::clear();
            }
            _ => ()
        }

        input = "".to_string();
    }

    print!("\x1b[0m");
    io::stdout().flush().unwrap();
    execute!(io::stdout(), SetForegroundColor(crossterm::style::Color::Reset), SetBackgroundColor(crossterm::style::Color::Reset)).unwrap();
}

pub async fn scan_qr() {
    
}