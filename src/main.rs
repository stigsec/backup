use chrono::Local;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::write::FileOptions;

fn main() -> zip::result::ZipResult<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: backup <file/directory>");
        std::process::exit(1);
    }
    let target_path = &args[1];
    let target = Path::new(target_path);
    if !target.exists() {
        eprintln!("Error: {} does not exist.", target_path);
        std::process::exit(1);
    }

    let backup_path = env::var("BACKUP_PATH").unwrap_or_else(|_| {
        let home = env::var("HOME").expect("HOME environment variable not set");
        format!("{}/.config/backups", home)
    });
    create_dir_all(&backup_path).expect("Failed to create backup directory");

    let timestamp = Local::now().format("%Y-%m-%d").to_string();
    let base_name = target
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "backup".to_string());
    let backup_file_path = format!("{}/{}_{}.zip", backup_path, base_name, timestamp);

    let zip_file = File::create(&backup_file_path).expect("Could not create zip file");
    let mut zip = zip::ZipWriter::new(zip_file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    if target.is_file() {
        let mut f = File::open(target).expect("Failed to open file");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("Failed to read file");
        zip.start_file(base_name, options)?;
        zip.write_all(&buffer)?;
    } else {
        for entry in WalkDir::new(target).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = path
                .strip_prefix(target.parent().unwrap_or(target))
                .expect("Failed to strip prefix")
                .to_string_lossy();
            if path.is_file() {
                zip.start_file(name.to_string(), options)?;
                let mut f = File::open(path).expect("Failed to open file");
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).expect("Failed to read file");
                zip.write_all(&buffer)?;
            } else if path.is_dir() && !name.is_empty() {
                zip.add_directory(name.to_string(), options)?;
            }
        }
    }
    zip.finish()?;
    println!("Backup created: {}", backup_file_path);
    Ok(())
}
