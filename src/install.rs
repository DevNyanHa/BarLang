use std::fs;
use std::io;
use std::env;
use std::path::Path;
use std::io::Write;

const ICON_BYTES: &[u8] = include_bytes!("../icon.ico");

pub fn install(path: &str) -> io::Result<()> {
    let current_exe = env::current_exe()?;
    let target_path = Path::new(path);

    if let Some(parent_dir) = target_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    fs::copy(&current_exe, &target_path)?;
    let mut icon = fs::File::create(target_path.with_file_name("icon.ico"))?;
    icon.write_all(ICON_BYTES)?;

    println!("📦 실행 파일을 복사했습니다: {} -> {}", current_exe.display(), target_path.display());
    Ok(())
}