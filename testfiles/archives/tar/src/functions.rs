use super::config::Colorname;
use super::errors::FxError;
use super::term::*;

use crossterm::style::Stylize;
use log::{info, warn};
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub const SPACES: u16 = 3;

/// Generate modified time as `String`.
pub fn format_time(time: &Option<String>) -> String {
    match time {
        Some(datetime) => format!("{} {}", &datetime[0..10], &datetime[11..16]),
        None => "".to_string(),
    }
}

/// Rename the put file, in order to avoid the name conflict.
pub fn rename_file(file_name: &str, name_set: &HashSet<String>) -> String {
    let mut count: usize = 1;
    let (stem, extension) = {
        let file_name = PathBuf::from(file_name);
        (
            file_name.file_stem().unwrap().to_owned(),
            file_name.extension().map(|x| x.to_owned()),
        )
    };
    let mut new_name = file_name.to_owned();

    while name_set.contains(&new_name) {
        let mut suffix = OsString::from("_");
        suffix.push({
            let count: OsString = count.to_string().into();
            count
        });
        let mut rename = stem.to_os_string();
        if let Some(ref ext) = extension {
            rename.push(suffix);
            rename.push(".");
            rename.push(ext);
        } else {
            rename.push(suffix);
        }
        new_name = rename.into_string().unwrap();
        count += 1;
    }
    new_name
}

/// Rename the put directory, in order to avoid the name conflict.
pub fn rename_dir(dir_name: &str, name_set: &HashSet<String>) -> String {
    let mut count: usize = 1;
    let mut new_name = dir_name.to_owned();
    while name_set.contains(&new_name) {
        let mut suffix = "_".to_string();
        suffix.push_str(&count.to_string());
        let mut rename = dir_name.to_owned();
        rename.push_str(&suffix);
        new_name = rename;
        count += 1;
    }
    new_name
}

pub fn reset_info_line() {
    to_info_bar();
    clear_current_line();
}

pub fn delete_cursor() {
    print!(" ");
    move_left(1);
}

/// Print the result of operation, such as put/delete/redo/undo.
pub fn print_info<T: std::fmt::Display>(message: T, then: u16) {
    delete_cursor();
    reset_info_line();
    info!("{}", message);
    print!("{}", message);

    hide_cursor();
    move_to(1, then);
    print_pointer();
    move_left(1);
}

/// When something goes wrong or does not work, print information about it.
pub fn print_warning<T: std::fmt::Display>(message: T, then: u16) {
    delete_cursor();
    warn!("{}", message);
    to_info_bar();
    clear_current_line();

    set_color(&TermColor::ForeGround(&Colorname::White));
    set_color(&TermColor::BackGround(&Colorname::LightRed));
    print!("{}", message,);
    reset_color();

    hide_cursor();
    move_to(1, then);
    print_pointer();
    move_left(1);
}

/// Print process of put/delete.
pub fn print_process<T: std::fmt::Display>(message: T) {
    print!("{}", message);
    move_left(7);
}

/// Print the number of process (put/delete).
pub fn display_count(i: usize, all: usize) -> String {
    let mut result = String::new();
    result.push_str(&(i + 1).to_string());
    result.push('/');
    result.push_str(&all.to_string());
    result
}

/// Convert extension setting in the config to HashMap.
pub fn to_extension_map(
    config: &Option<HashMap<String, Vec<String>>>,
) -> Option<HashMap<String, String>> {
    let mut new_map = HashMap::new();
    match config {
        Some(config) => {
            for (command, extensions) in config.iter() {
                for ext in extensions.iter() {
                    new_map.insert(ext.to_lowercase(), command.clone());
                }
            }
        }
        None => return None,
    }
    Some(new_map)
}

/// Create the duration as String. Used after print_process(put/delete).
pub fn duration_to_string(duration: Duration) -> String {
    let s = duration.as_secs_f32();
    let mut result: String = s.to_string().chars().take(4).collect();
    result.push('s');
    result
}

/// Get the size format of item.
pub fn to_proper_size(byte: u64) -> String {
    let mut result: String;
    if byte < 1000 {
        result = byte.to_string();
        result.push('B');
    } else if byte < 1_000_000 {
        result = (byte / 1_000).to_string();
        result.push_str("KB");
    } else if byte < 1_000_000_000 {
        result = (byte / 1_000_000).to_string();
        result.push_str("MB");
    } else {
        result = (byte / 1_000_000_000).to_string();
        result.push_str("GB");
    }
    result
}

/// Generate the contents of item to show as a preview.
pub fn list_up_contents(path: &Path) -> Result<Vec<String>, FxError> {
    let mut file_v = Vec::new();
    let mut dir_v = Vec::new();
    let mut result = Vec::new();
    for item in std::fs::read_dir(path)? {
        let item = item?;
        if item.file_type()?.is_dir() {
            dir_v.push(item.file_name().into_string().unwrap_or_default());
        } else {
            file_v.push(item.file_name().into_string().unwrap_or_default());
        }
    }
    dir_v.sort_by(|a, b| natord::compare(a, b));
    file_v.sort_by(|a, b| natord::compare(a, b));
    result.append(&mut dir_v);
    result.append(&mut file_v);
    Ok(result)
}

/// Generate the contents tree.
pub fn make_tree(v: Vec<String>) -> Result<String, FxError> {
    let len = v.len();
    let mut result = String::new();
    for (i, path) in v.iter().enumerate() {
        if i == len - 1 {
            let mut line = "└ ".to_string();
            line.push_str(path);
            result.push_str(&line);
        } else {
            let mut line = "├ ".to_string();
            line.push_str(path);
            line.push('\n');
            result.push_str(&line);
        }
    }
    Ok(result)
}

/// Format texts to print. Used when printing help or text preview.
pub fn format_txt(txt: &str, column: u16, is_help: bool) -> Vec<String> {
    let mut v = Vec::new();
    let mut column_count = 0;
    let mut line = String::new();
    for c in txt.chars() {
        if c == '\n' {
            v.push(line);
            line = String::new();
            column_count = 0;
            continue;
        }
        line.push(c);
        column_count += 1;
        if column_count == column {
            v.push(line);
            line = String::new();
            column_count = 0;
            continue;
        }
    }
    if !line.is_empty() {
        v.push(line);
    }
    if is_help {
        v.push("Press Enter to go back.".to_string());
    }
    v
}

/// Print help text.
pub fn print_help(v: &[String], skip_number: usize, row: u16) {
    let mut row_count = 0;
    for (i, line) in v.iter().enumerate() {
        if i < skip_number {
            continue;
        }

        move_to(1, (i + 1 - skip_number) as u16);
        if row_count == row - 1 {
            print!("{}", "...".negative());
            break;
        }
        print!("{}", line);
        row_count += 1;
    }
}

pub fn is_editable(s: &str) -> bool {
    s.is_ascii()
}

pub fn init_log(config_dir_path: &Path) -> Result<(), FxError> {
    let mut log_name = chrono::Local::now().format("%F-%H-%M-%S").to_string();
    log_name.push_str(".log");
    let config = ConfigBuilder::new()
        .set_time_offset_to_local()
        .unwrap()
        .build();
    let log_path = config_dir_path.join("log");
    if !log_path.exists() {
        std::fs::create_dir(&log_path)?;
    }
    let log_path = log_path.join(log_name);
    WriteLogger::init(LevelFilter::Info, config, std::fs::File::create(log_path)?)?;
    info!("===START===");

    Ok(())
}

pub fn check_version() -> Result<(), FxError> {
    let output = std::process::Command::new("cargo")
        .args(["search", "felix", "--limit", "1"])
        .output()?
        .stdout;
    if !output.is_empty() {
        if let Ok(ver) = std::str::from_utf8(&output) {
            let latest: String = ver.chars().skip(9).take_while(|x| *x != '\"').collect();
            let current = env!("CARGO_PKG_VERSION");
            if latest != current {
                println!("felix v{}: Latest version is {}.", current, latest);
            } else {
                println!("felix v{}: Up to date.", current);
            }
        } else {
            println!("Cannot read the version.");
        }
    } else {
        println!("Cannot fetch the latest version: Check your internet connection.");
    }
    Ok(())
}

pub fn convert_to_permissions(permissions: u32) -> String {
    let permissions = format!("{permissions:o}");
    let permissions: String = permissions.chars().rev().take(3).collect();
    permissions.chars().rev().collect()
}

pub fn extract_tar(p: PathBuf, dest: PathBuf) -> Result<(), FxError> {
    let file = std::fs::File::open(p)?;
    let file = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(file);
    archive.unpack(dest)?;

    Ok(())
}

pub fn extract_zip(p: PathBuf, dest: PathBuf) -> Result<(), FxError> {
    let file = std::fs::File::open(p)?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(dest).unwrap();
    Ok(())
}

//cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        let time1 = Some("1970-01-01 00:00:00".to_string());
        let time2 = None;
        assert_eq!(format_time(&time1), "1970-01-01 00:00".to_string());
        assert_eq!(format_time(&time2), "".to_string());
    }

    #[test]
    fn test_display_count() {
        assert_eq!(display_count(1, 4), "2/4".to_string());
    }

    #[test]
    fn test_proper_size() {
        assert_eq!(to_proper_size(50), "50B".to_string());
        assert_eq!(to_proper_size(2000), "2KB".to_string());
        assert_eq!(to_proper_size(3000000), "3MB".to_string());
        assert_eq!(to_proper_size(6000000000), "6GB".to_string());
    }

    #[test]
    fn test_duration_to_string() {
        assert_eq!(
            duration_to_string(Duration::from_millis(5432)),
            "5.43s".to_string()
        );
    }

    #[test]
    fn test_make_tree() {
        let v = vec![
            "data".to_string(),
            "01.txt".to_string(),
            "2.txt".to_string(),
            "a.txt".to_string(),
            "b.txt".to_string(),
        ];
        let tree = make_tree(v).unwrap();
        let formatted = format_txt(&tree, 50, false);
        assert_eq!(
            tree,
            ("├ data\n├ 01.txt\n├ 2.txt\n├ a.txt\n└ b.txt").to_string()
        );
        assert_eq!(tree.lines().count(), formatted.len());
    }

    #[test]
    fn test_is_editable() {
        let s1 = "Hello, world!";
        let s2 = "image.jpg";
        let s3 = "a̐éö̲\r\n";
        let s4 = "日本の首都は東京です";
        assert!(is_editable(s1));
        assert!(is_editable(s2));
        assert!(!is_editable(s3));
        assert!(!is_editable(s4));
    }

    #[test]
    fn test_convert_to_permissions() {
        let file = 33188;
        let dir = 16877;
        assert_eq!(&convert_to_permissions(file), "644");
        assert_eq!(&convert_to_permissions(dir), "755");
    }

    #[test]
    fn test_extract_tar() {
        let p = PathBuf::from("/home/kyohei/Downloads/felix-1.3.2.tar.gz");
        let dest = PathBuf::from("/home/kyohei/test/");
        assert!(extract_tar(p, dest).is_ok());

        let p = PathBuf::from("/home/kyohei/Downloads/berkeley-mono-typeface.zip");
        let dest = PathBuf::from("/home/kyohei/test/");
        if let Err(e) = extract_tar(p, dest) {
            eprintln!("{}", e);
        }
    }

    #[test]
    fn test_extract_zip() {
        let p = PathBuf::from("/home/kyohei/Downloads/berkeley-mono-typeface.zip");
        let dest = PathBuf::from("/home/kyohei/test/");
        assert!(extract_zip(p, dest).is_ok());

        let p = PathBuf::from("/home/kyohei/Downloads/felix-1.3.2.tar.gz");
        let dest = PathBuf::from("/home/kyohei/test/");
        if let Err(e) = extract_zip(p, dest) {
            eprintln!("{}", e);
        }
    }
}
