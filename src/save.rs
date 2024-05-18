use robo_archiver::ArchiveError;

use crate::Periodical;

const DEFAULT_FILE_NAME: &str = "archive.csv";

const HEADERS: [&str; 56] = [
    "NODE_TITLE",
    "ASSETS", // 1, skip
    "ATTACHMENTS", // 2, skip
    "#REDACT", // 3, skip
    "Part Of", // 4, skip
    "Previous Issue",
    "Next Issue",
    "Creator",
    "Contributor",
    "Publisher",
    "Volume",
    "Issue",
    "Description",
    "Subject",
    "Date Original",
    "Date Range",
    "Type",
    "Original Format",
    "Language",
    "Contributing Institution",
    "Collection",
    "Subcollection",
    "Rights Statement",
    "State Agency", // 24, skip
    "State Sub-Agency", // 25, skip
    "Federal Legislative Branch Agency", // 26, skip
    "Federal Executive Department", // 27, skip
    "Federal Executive Department Sub-Agency or Bureau", // 28, skip
    "Federal Independent Agency", // 29, skip
    "Federal Board, Commission, or Committee", // 30, skip
    "Federal Quasi-Official Agency", // 31, skip
    "Federal Court or Judicial Agency", // 32, skip
    "City or Town", // 33, skip
    "Geographic Feature", // 34, skip
    "Tribal Homeland", // 35, skip
    "Road", // 36, skip
    "County", // 37, skip
    "State", // 38, skip
    "Country", // 39, skip
    "Agency", // 40, skip
    "Event", // 41, skip
    "Oral History", // 42, skip
    "Person", // 43, skip
    "Place", // 44, skip
    "Topic",
    "Acquisition Note", // 46, skip
    "Call Number",
    "Vertical File", // 48, skip
    "OCLC Number",
    "Date Digitized", // 50, skip
    "Digital Format",
    "File Size", // 52, skip
    "Digitizing Institution",
    "Date Ingested", // 54, skip
    "Batch Number", // 55, skip
    "Admin Notes", // 56, skip
];

macro_rules! blank {
    () => {
        "".to_string()
    };
}

impl From<Periodical> for Vec<[String; 56]> {
    fn from(periodical: Periodical) -> Self {
        let mut records: Vec<[String; 56]> = Vec::new();
        for (i, issue) in periodical.issues.into_iter().enumerate() {
            let node_title = issue.node_title; // 0
            let prev_issue = issue.previous_issue.unwrap_or_else(|| "".to_string()); // 5
            let next_issue = issue.next_issue.unwrap_or_else(|| "".to_string()); // 6
            let creator = issue.marc.creators.join("|").to_string(); // 7
            let contributor = issue.contributors.join("|").to_string(); // 8
            let publisher = issue.marc.publisher; // 9
            let volume = issue.volume_no.unwrap_or_else(|| "".to_string()); // 10
            let issue_no = issue.issue_no.map(|i| i.to_string()).unwrap_or_else(|| "".to_string()); // 11
            let description = /* 12 */ match i {
                0 => periodical.description.clone(),
                _ => "".to_string(),
            };
            let subject = issue.marc.subject_headings.join("|").to_string(); // 13
            let date_original = issue.data_original.join("--").to_string(); // 14
            let date_range = issue.date_range.join("|").to_string(); // 15
            let item_type = issue.item_type.to_string(); // 16
            let original_format = issue.format_type.to_string(); // 17
            let languages = issue.languages.join("|").to_string(); // 18
            let contributing_institution = match i {
                0 => periodical.contributing_institution.to_string(), // 19
                _ => "".to_string(),
            };
            let collection = match i {
                0 => periodical.collection.to_string(), // 20
                _ => "".to_string(),
            };
            let subcollection = issue.parent_collection.to_string(); // 21
            let rights_statement = issue.rights_statement.to_string(); // 22
            let topics = match i {
                0 => periodical.topics.join("|").to_string(), // 45
                _ => "".to_string(),
            };
            let call_number = issue.marc.call_number.to_string(); // 47
            let oclc_number = issue.marc.oclc_number; // 49
            let digital_format = issue.digital_format.to_string(); // 51
            let digitizing_institution = issue.digitizing_institution.to_string(); // 53

            let record: [String; 56] = [
                node_title, // 0
                blank!(/* 1 */),
                blank!(/* 2 */),
                blank!(/* 3 */),
                blank!(/* 4 */),
                prev_issue, // 5
                next_issue, // 6
                creator, // 7
                contributor, // 8
                publisher, // 9
                volume, // 10
                issue_no, // 11
                description, // 12
                subject, // 13
                date_original, // 14
                date_range, // 15
                item_type, // 16
                original_format, // 17
                languages, // 18
                contributing_institution, // 19
                collection, // 20
                subcollection, // 21
                rights_statement, // 22
                blank!(/* 23 */),
                blank!(/* 24 */),
                blank!(/* 25 */),
                blank!(/* 26 */),
                blank!(/* 27 */),
                blank!(/* 28 */),
                blank!(/* 29 */),
                blank!(/* 30 */),
                blank!(/* 31 */),
                blank!(/* 32 */),
                blank!(/* 34 */),
                blank!(/* 35 */),
                blank!(/* 36 */),
                blank!(/* 37 */),
                blank!(/* 38 */),
                blank!(/* 39 */),
                blank!(/* 40 */),
                blank!(/* 41 */),
                blank!(/* 42 */),
                blank!(/* 43 */),
                blank!(/* 44 */),
                topics, // 45
                blank!(/* 46 */),
                call_number, // 47
                blank!(/* 48 */),
                oclc_number, // 49
                blank!(/* 50 */),
                digital_format, // 51
                blank!(/* 52 */),
                digitizing_institution, // 53
                blank!(/* 54 */),
                blank!(/* 55 */),
                blank!(/* 56 */),
            ];

            records.push(record);
        }

        records
    }
}

pub fn write_periodicals_to_file(
    periodicals: Vec<Periodical>,
    out_path: Option<String>
) -> Result<(), ArchiveError> {
    // If the user didn't specify an output path, save to the default file name.
    // if the file already exists, append a (1), (2), etc. to the file name.
    let out_path = {
        let out_path = out_path.unwrap_or_else(|| DEFAULT_FILE_NAME.to_string());
        // convert the out_path to a PathBuf
        let mut out_path = std::path::Path::new(&out_path).to_path_buf();
        let current_dir = std::env::current_dir()?;

        if out_path.exists() {
            let mut i = 1;
            loop {
                let file_name = out_path.file_name().unwrap().to_str().unwrap();

                // remove the extension, and replace any (n) with an empty string
                let file_name = file_name.rsplit_once('.').unwrap().0;

                // check if the file name has a (n) at the end
                let new_file_name = if let Some((file_name, n)) = file_name.rsplit_once(" (") {
                    if n.ends_with(')') {
                        let n = n.trim_end_matches(')').parse::<u32>().unwrap();
                        format!("{} ({}).csv", file_name, n + i)
                    } else {
                        format!("{} ({}).csv", file_name, i)
                    }
                } else {
                    format!("{} ({}).csv", file_name, i)
                };

                let new_path = current_dir.join(new_file_name);
                if !new_path.exists() {
                    out_path = new_path;
                    break;
                }
                i += 1;
            }
        }
        out_path
    };

    println!("Saving to: {:?}", out_path);

    let mut wtr = csv::Writer::from_path(out_path)?;

    wtr.write_record(HEADERS)?;

    for periodical in periodicals {
        let records: Vec<[String; 56]> = periodical.into();
        for record in records {
            wtr.write_record(record)?;
        }
    }

    wtr.flush()?;

    Ok(())
}
