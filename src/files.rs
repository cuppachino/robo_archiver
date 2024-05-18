use std::path::PathBuf;

use robo_archiver::ArchiveError;

use crate::DigitalFormat;

const SKIP_DIRS: [&str; 2] = ["target", "__MACOSX"];
const SKIP_EXTS: [&str; 2] = ["rs", "toml"];
const SKIP_FILES: [&str; 1] = [".DS_Store"];

pub fn load_directory<T>(path: T, is_recursive: bool) -> Vec<PathBuf> where T: Into<PathBuf> {
    let path: PathBuf = path.into();
    let mut files = Vec::new();

    let entries = path.read_dir().expect("Failed to open directory. Is the path a folder?");

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            if SKIP_DIRS.contains(&path.file_name().unwrap().to_str().unwrap()) {
                continue;
            }
            if is_recursive {
                files.extend(load_directory(&path, is_recursive));
            }
        } else {
            if SKIP_FILES.contains(&path.file_name().unwrap().to_str().unwrap()) {
                continue;
            }

            if let Some(ext) = path.extension() {
                if SKIP_EXTS.contains(&ext.to_str().unwrap()) {
                    continue;
                }
            }
            files.push(path);
        }
    }

    if files.is_empty() {
        eprintln!("No files found in the directory.");
    }

    files
}

/// The data extracted from the file name.
#[derive(Debug, Clone)]
pub struct IssueFileData {
    /// The name of the issue.
    pub node_title: String,
    /// The date of the issue.
    pub date_original: String,
    /// The date range of the issue.
    pub date_range: String,
    /// The format of the file.
    pub format: DigitalFormat,
}

impl IssueFileData {
    // Returns a formatted string with the date appended to the end of the file name, separated by a comma.
    pub fn node_title_with_date(&self) -> String {
        format!("{}, {}", self.node_title, self.date_original)
    }
}

/// Takes a date string in the format `yyyy-mm-dd` or `yyyy-mm` or `yyyy` and returns a date range string.
///
/// E.g. `1967-04` -> `1960s (1960-1969)`.
///
/// E.g. `1967-04-22` -> `1960s (1960-1969)`.
fn data_to_date_range(date: String) -> String {
    let parts: Vec<&str> = date.split('-').collect();
    let year = parts[0].parse::<i32>().unwrap();
    let decade = year - (year % 10);
    format!("{}s ({}-{})", decade, decade, decade + 9)
}

/// Transform a file name into an `IssueFileData` struct.
///
/// E.g. `An_Arizona_Desert-ation_1967-04.pdf` -> `IssueFileData { node_title: "An Arizona Desert-ation", date_original: "1967-04", date_range: "1960s (1960-1969)" }`.
fn extract_data_from_file_name(file_path: PathBuf) -> Result<IssueFileData, ArchiveError> {
    // remove the file extension
    let (file_name, ext) = file_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .rsplit_once('.')
        .expect("Failed to split file name and extension. Does the file have an extension?");

    // the data will always be numbers separated by a dash, so
    // we can walk back from the end of the string and split on the first underscore
    let mut date_original = String::new();
    for c in file_name.chars().rev() {
        if c.is_numeric() || c == '-' {
            date_original.push(c);
        } else {
            break;
        }
    }
    date_original = date_original.chars().rev().collect();
    // remove the date from the file name
    let node_title = file_name
        .trim_end_matches(&date_original)
        .trim_end_matches('_')
        .to_string()
        .replace('_', " ");

    // if either are empty, we have a problem, warn the user with a message
    if node_title.is_empty() || date_original.is_empty() {
        return Err(ArchiveError::UnparseableFileName(file_path.to_string_lossy().to_string()));
    }

    let date_range = data_to_date_range(date_original.clone());

    Ok(IssueFileData {
        node_title,
        date_original: date_original.to_string(),
        date_range,
        format: DigitalFormat::from(ext),
    })
}

pub fn process_files(file_paths: Vec<PathBuf>) -> Result<Vec<Vec<IssueFileData>>, ArchiveError> {
    let mut periodicals: Vec<Vec<IssueFileData>> = Vec::new();

    // group the files by their node_title.
    for file_path in file_paths {
        let data = extract_data_from_file_name(file_path)?;
        let node_title = &data.node_title;
        if
            let Some(item) = periodicals
                .iter_mut()
                .find(|p| { p.iter().any(|i| &i.node_title == node_title) })
        {
            item.push(data);
        } else {
            periodicals.push(vec![data]);
        }
    }

    // sort the issues by date_original
    for periodical in &mut periodicals {
        periodical.sort_by(|a, b| a.date_original.cmp(&b.date_original));
    }

    Ok(periodicals)
}
