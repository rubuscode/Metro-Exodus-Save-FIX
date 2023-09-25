use colored::Colorize;
use std::io;
use std::fs;
use std::path::Path;
use chrono::{Datelike, Timelike};

fn c_get(text: &str) -> String {
    println!("{}: ", text.green());
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(error) => {
            eprintln!("[-] error: {}", error);
            String::new()
        }
    }
}

fn copy_quick_save_to_backup(save_folder_path: &str, backup_dir: &str) {
    let date_time = chrono::Local::now();
    let formatted_date = format!(
        "{:02}.{:02}.{} {:02}{:02}{:02}",
        date_time.day(),
        date_time.month(),
        date_time.year(),
        date_time.hour(),
        date_time.minute(),
        date_time.second()
    );

    let quick_save_file = fs::read_dir(save_folder_path)
        .expect("[-] failed to read directory")
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().ok()?;
                if file_name.contains("quick_save") {
                    Some(file_name)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .next()
        .expect("[-] quick save was not found");

    let quick_save_path = format!("{}/{}", save_folder_path, quick_save_file);
    let quick_save_modification_time = fs::metadata(&quick_save_path)
        .expect("[-] failed to get file metadata")
        .modified()
        .expect("[-] failed to get modification time");

    let backup_save_path = format!("{}/{} {}", backup_dir, quick_save_file, formatted_date);

    if !Path::new(backup_dir).exists() {
        create_backup_directory_if_not_exists(backup_dir);
    }

    let mut save_already_stored = false;

    if let Ok(entries) = fs::read_dir(backup_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if let Ok(metadata) = fs::metadata(&file_path) {
                    let saved_file_modification_time = metadata.modified().expect("[-] failed to get saved file modification time");
                    if quick_save_modification_time == saved_file_modification_time {
                        println!("[+] save already stored");
                        save_already_stored = true;
                        break;
                    }
                }
            }
        }
    }

    if !save_already_stored {
        if let Err(err) = fs::copy(&quick_save_path, &backup_save_path) {
            panic!("[-] error copying file: {}", err);
        } else {
            println!("[+] {} was copied to {}", quick_save_path, backup_save_path);
        }
    }
}

fn create_backup_directory_if_not_exists(dir: &str) {
    if let Err(_) = fs::create_dir(dir) {
        println!("[/] backup directory exists");
    } else {
        println!("[+] backup directory created");
    }
}

fn main() {
    println!("{}", "[!] before start:".cyan());
    println!("{}", "1. disable your game\n2. disable steam cloud for metro exodus [\n - open steam\n - rclick on metro exodus\n - click \"properties\"\n - in general tab, u must to uncheck \"Preserve saved games in Steam Cloud for Metro Exodus\" in \"STEAM CLOUD\" subtab\n]\n".yellow());
    
    let save_folder = c_get("Saved Game path with your ID");
    
    create_backup_directory_if_not_exists(&save_folder);
    copy_quick_save_to_backup(&save_folder, &format!("{}/backup", save_folder));
}
