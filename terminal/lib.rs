use std::io;
use std::path::Path;
use std::process::Command;
use sha256::digest;
use crossterm::execute;
use crossterm::style::SetForegroundColor;
use mlua::Lua;

use base64::Engine;
use base64::prelude::BASE64_STANDARD;

use configparser::ini::Ini;

pub const BOOTSTRAPP: &str = include_str!("bootstrap.lua");

const VERSION: &str = "0.1.5";
pub const LOGO: &str = "\x1b[1;32m
                    ..                                \x1b[0;32m                             
                  .oK0l\x1b[0;32m
                 :0KKKKd.\x1b[0;34m
               .xKO0KKKKd\x1b[0;35m\t\t\t     \x1b[1;32mSystem \x1b[0;36mInformation                         
              ,Od' .d0000l\x1b[0;36m                        -------------------------
             .c;.   .'''...           ..'.\x1b[0;32m
.,:cloddxxxkkkkOOOOkkkkkkkkxxxxxxxxxkkkx:\x1b[0;34m         OS: {os}
;kOOOOOOOkxOkc'...',;;;;,,,'',;;:cllc:,.\x1b[0;35m          ARCHITECTURE: {arch}
 .okkkkd,.lko  .......',;:cllc:;,,'''''.\x1b[0;36m          CPU CORES: {cpu}
   .cdo. :xd' cd:.  ..';'',,,'',,;;;,'.\x1b[0;32m           GPU: {gpu}
      . .ddl.;doooc'..;oc;'..';::;,'. \x1b[0;34m            RAM: {mem} GB
        coo;.oooolllllllcccc:'.  .\x1b[0;35m
       .ool''lllllccccccc:::::;.\x1b[0;36m
       ;lll. .':cccc:::::::;;;;'\x1b[0;32m
       :lcc:'',..';::::;;;;;;;,,.\x1b[0;34m
       :cccc::::;...';;;;;,,,,,,.\x1b[0;35m
       ,::::::;;;,'.  ..',,,,'''. \x1b[0;36m
    ```
    ........          ......\x1b[0m
";

pub fn txt_print(text: &str) -> io::Result<()> {
    println!("{}", text.chars().filter(|c| !c.is_control()).collect::<String>());
    Ok(())
}

pub fn get_gpu_info() -> Option<String> {
  if cfg!(target_os = "linux") {
    let output = Command::new("lspci")
        .arg("-v")
        .output()
        .expect("Failed to execute lspci command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
        if line.contains("VGA") || line.contains("3D") {
          return Some(line.to_string());
        }
    }
  } else if cfg!(target_os = "windows") {
        let output = Command::new("wmic")
          .args(&["path", "win32_VideoController", "get", "name"])
          .output()
          .expect("Failed to execute wmic command");

        let output_str = String::from_utf8_lossy(&output.stdout);
      
        return output_str.lines().nth(1).map(|line| line.trim().to_string());
  }
  None
}

fn hash(_lua: &Lua, input: String) -> Result<String, mlua::Error> {
    Ok(digest(input).to_owned())
}

fn verify(_lua: &Lua, (input, expected): (String, String)) -> Result<bool, mlua::Error> {
    let hash = digest(input.as_bytes());

    if hash == expected {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn init() {
    let mut config = Ini::new();
    let lua = Lua::new();

    if Path::new("config.ini").exists() {
        config.load("config.ini").unwrap();
    }

    lua.globals().set("hash", lua.create_function(hash).unwrap()).unwrap();
    lua.globals().set("verify", lua.create_function(verify).unwrap()).unwrap();
    lua.globals().set("encode", lua.create_function(|_, input: String| {
        Ok(BASE64_STANDARD.encode(input))
    }).unwrap()).unwrap();

    // println!("{:#?}", config.get("terminal", "savelogin"));
    if config.get("terminal", "savelogin") != Some("true".to_string()) {
        match lua.load(&*BOOTSTRAPP).exec() {
            Ok(_) => (),
            Err(err) => println!("{}", err)
        }
    }
    
    execute!(io::stdout(), SetForegroundColor(crossterm::style::Color::Cyan)).unwrap();
    println!("\tGCMD \x1b[0;32m[Version {}]", VERSION);
}

pub fn clear() {
    println!("\x1b[2J\x1b[H");
}