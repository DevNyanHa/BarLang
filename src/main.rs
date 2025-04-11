use std::io::{self, Write};
use std::fs;
use std::env;
use colored::*;

mod registry;
mod install;

fn wait() {
    print!("ENTER >");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
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
        return;
    } else {
        let path: String = String::from("C:\\BarLang\\BarLang.exe");

        if let Err(e) = install::install(&path) {
            eprintln!("{}", e);
            return;
        }

        if let Err(e) = registry::register(&path) {
            eprintln!("{}", e);
            return;
        }

        wait();
    }
}
fn print_file_contents(file_path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(file_path)?;
    println!("📄 파일 내용 ({}):\n", file_path);
    println!("{}", contents);
    wait();
    Ok(())
}