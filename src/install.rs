use std::fs;
use std::io;
use std::env;
use std::path::Path;
use std::io::Write;
use winreg::RegKey;
use winreg::enums::HKEY_CLASSES_ROOT;

const ICON_BYTES: &[u8] = include_bytes!("../icon.ico");

pub fn install(path: &str) -> io::Result<()> {
    println!("Installing BarLang {}", env!("CARGO_PKG_VERSION"));
    if let Some(parent) = Path::new(path).parent() {
        println!("path : {}", parent.display());
    }
    let current_exe = env::current_exe()?;
    let target_path = Path::new(path);

    if let Some(parent_dir) = target_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    //copy files
    fs::copy(&current_exe, &target_path)?;
    let mut icon = fs::File::create(target_path.with_file_name("icon.ico"))?;
    icon.write_all(ICON_BYTES)?;

    //registry
    registry(path)?;
    println!("Successfully installed BarLang");
    Ok(())
}

pub fn registry(path: &str) -> io::Result<()> {
    println!("Registering Registry");
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

    let (ext_key, _) = hkcr.create_subkey(".lll")?;
    ext_key.set_value("", &"BarLang")?;

    let (type_key, _) = hkcr.create_subkey("BarLang")?;
    type_key.set_value("", &"BarLang Source File")?;

    let (cmd_key, _) = type_key.create_subkey("shell\\open\\command")?;
    let command = format!(r#""{}" "%1""#, path);
    cmd_key.set_value("", &command)?;

    let (icon_key, _) = type_key.create_subkey("DefaultIcon")?;
    let icon_path = format!(r#"{}\\icon.ico"#, std::path::Path::new(path).parent().unwrap().display());
    icon_key.set_value("", &icon_path)?;

    println!("Successfully registered registry");
    Ok(())
}