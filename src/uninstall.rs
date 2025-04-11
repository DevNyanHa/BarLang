use winreg::enums::HKEY_CLASSES_ROOT;
use winreg::RegKey;
use std::io;
use std::fs;

pub fn uninstall(path: &str) -> io::Result<()> {
    println!("Uninstalling BarLang");

    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

    match hkcr.delete_subkey_all(".lll") {
        Ok(_) => println!("Delete extension key .lll"),
        Err(e) => println!("{}", e),
    }

    match hkcr.delete_subkey_all("BarLang") {
        Ok(_) => println!("Delete BarLang key"),
        Err(e) => println!("{}", e),
    }

    if std::path::Path::new(path).exists() {
        fs::remove_dir_all(path)?;
        println!("Delete folder {}", path);
    }

    println!("Successfully removed BarLang");
    Ok(())
}