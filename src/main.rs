use std::io::{self, Write};
use std::fs;
use std::env;
use colored::*;

mod install;
mod uninstall;

fn wait() {
    print!("ENTER >");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}

fn main() -> io::Result<()> {
    colored::control::set_override(true);
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 { //BarLang
        let file_path: &String = &args[1];

        println!("{} {}", "Compiling".green(), file_path.green());

        if file_path.ends_with(".lll") {
            if let Err(e) = print_file_contents(file_path) {
                eprintln!("{}", e);
            }
        } else {
            eprintln!("Not BarLang File");
        }
        wait();
        Ok(())
    } else { //Install BarLang
        println!("BarLang Installer {}", env!("CARGO_PKG_VERSION"));
        println!("1) Install BarLang");
        println!("2) UnInstall BarLang");
        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let choice = input.trim();

            match choice {
                "1" => {
                    if let Err(e) = install::install("C:\\BarLang\\BarLang.exe") {
                        eprintln!("{}", e);
                    }
                    break;
                },
                "2" => {
                    if let Err(e) = uninstall::uninstall("C:\\BarLang") {
                        eprintln!("{}", e);
                    }
                    break;
                },
                _ => {}
            }
        }

        wait();
        Ok(())
    }
}
fn print_file_contents(file_path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(file_path)?;
    println!("📄 파일 내용 ({}):\n", file_path);
    println!("{}", contents);
    wait();
    Ok(())
}