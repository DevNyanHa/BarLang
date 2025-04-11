use winreg::enums::*;
use winreg::RegKey;
use std::io;

pub fn register(path: &str) -> io::Result<()> {
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

    println!("🔗 레지스트리 등록 완료!");
    Ok(())
}