use anyhow::{anyhow, Result};
use infer;
use std::{env, fs, path, thread, time};

fn setup_folders() -> Result<()> {
    let folders = vec![
        "Image",
        "Video",
        "Audio",
        "Archive",
        "Book",
        "Documents",
        "Font",
        "Application",
    ];
    let working_dir = env::current_dir()?;
    if let Some(name) = working_dir.file_name() {
        if let Some(filename_str) = name.to_str() {
            if folders.contains(&filename_str) {
                return Err(anyhow!("Parent folder shares name with MIME type"));
            }
        }
    }
    for path in folders {
        let path = path::Path::new(path);
        if path.exists() {
            continue;
        } else {
            fs::create_dir(path)?
        }
    }
    Ok(())
}

fn check_type(path: &path::PathBuf) -> Result<String> {
    let path_read = &fs::read(&path)?;
    if infer::is_image(path_read) {
        return Ok(String::from("Image"));
    }

    if infer::is_video(path_read) {
        return Ok(String::from("Video"));
    }

    if infer::is_audio(path_read) {
        return Ok(String::from("Audio"));
    }

    if infer::is_archive(path_read) {
        return Ok(String::from("Archive"));
    }

    if infer::is_book(path_read) {
        return Ok(String::from("Book"));
    }

    if infer::is_document(path_read) {
        return Ok(String::from("Document"));
    }

    if infer::is_font(path_read) {
        return Ok(String::from("Font"));
    }

    if infer::is_app(path_read) {
        return Ok(String::from("Application"));
    }

    return Err(anyhow!("File is not a known type"));
}

fn move_file(path: path::PathBuf) -> Result<()> {
    let file_type: String = check_type(&path)?;

    let mut new_path = path::PathBuf::from(&file_type);
    new_path.push(&path.file_name().unwrap().to_str().unwrap());
    println!(
        "Moving {} to {}",
        &path.file_name().unwrap().to_str().unwrap(),
        &file_type
    );
    fs::rename(path, new_path)?;
    thread::sleep(time::Duration::from_millis(1));
    Ok(())
}

fn main() -> Result<()> {
    setup_folders()?;

    let working_dir = env::current_dir()?;

    for entry in fs::read_dir(&working_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path == env::current_exe()? {
            continue;
        }

        if path.is_dir() {
            continue;
        }
        let file_type = match infer::get_from_path(&path) {
            Ok(file) => file,
            Err(_) => continue,
        };

        match file_type {
            Some(_) => {
                move_file(path)?;
            }
            None => continue,
        }
    }

    Ok(())
}
