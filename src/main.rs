pub mod cli;
pub mod data;
pub mod files;
pub mod marc;
pub mod save;
pub mod topics;

use clap::Parser;
use files::{ load_directory, process_files };
use robo_archiver::ArchiveError;
use crate::{
    cli::Args,
    data::*,
    marc::{ accept_marc, parse_marc },
    save::write_periodicals_to_file,
    topics::select_topics_with_retries,
};

fn main() -> Result<(), ArchiveError> {
    // Parse command line arguments.
    let args = Args::parse();
    let languages = args.languages.unwrap_or_else(|| vec!["English".to_string()]);
    let file_exts = args.file_ext;
    let out_path = args.out_path;
    let collection = args.collection.map(PeriodicalCollection::from).unwrap_or_default();
    let contributing_institution = args.contributing_institution
        .map(ContributingInstitution::from)
        .unwrap_or_default();
    let rights_statement = args.rights_statement.map(RightsStatement::from).unwrap_or_default();
    let digitizing_instituion = args.digitization_institution
        .map(DigitizingInstitution::from)
        .unwrap_or_default();

    let periodicals = {
        let path = args.file_dir.unwrap_or_else(|| ".".to_string());
        let is_recursive = args.recursive;
        let file_paths = load_directory(path, is_recursive, file_exts);
        let data = process_files(file_paths)?;
        process_periodicals(
            data,
            languages,
            digitizing_instituion,
            rights_statement,
            collection,
            contributing_institution
        )?
    };

    write_periodicals_to_file(periodicals, out_path)?;

    Ok(())
}

fn prompt_marc(collection_name: &str) -> MarcData {
    let call_number = {
        let input = prompt_user_input(
            format!("Enter the call number (or just hit [ENTER] if PERIODICAL) for the \"{}\" collection:", collection_name).as_str()
        );
        if input.is_empty() {
            CallNumber::Periodical
        } else {
            CallNumber::Shelf(input)
        }
    };
    let marc = accept_marc();
    let marc = std::io::BufReader::new(marc.as_bytes());
    parse_marc(call_number, marc).unwrap()
}

fn prompt_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn process_periodicals(
    data: Vec<Vec<IssueFileData>>,
    languages: Vec<String>,
    digitizing_instituion: DigitizingInstitution,
    rights_statement: RightsStatement,
    collection: PeriodicalCollection,
    contributing_institution: ContributingInstitution
) -> Result<Vec<Periodical>, ArchiveError> {
    let mut periodicals: Vec<Periodical> = Vec::new();
    for issue_datas in data.iter() {
        let periodical_collection = issue_datas
            .first()
            .expect("Expected periodical to have at least one issue")
            .node_title.clone();
        let marc = prompt_marc(periodical_collection.as_str());
        let mut issues: Vec<Issue> = Vec::new();

        for (i, issue_data) in issue_datas.iter().enumerate() {
            let previous_issue = if i == 0 {
                None
            } else {
                issue_datas.get(i - 1).map(|i| { i.node_title_with_date() })
            };
            let next_issue = issue_datas.get(i + 1).map(|i| { i.node_title_with_date() });

            let issue = Issue {
                marc: marc.clone(),
                node_title: issue_data.node_title_with_date(),
                data_original: vec![issue_data.date_original.clone()],
                date_range: vec![issue_data.date_range.clone()],
                digital_format: issue_data.format.clone(),
                parent_collection: issue_data.node_title.clone(),
                next_issue,
                previous_issue,
                contributors: Vec::new(),
                languages: languages.clone(),
                issue_no: None,
                volume_no: None,
                item_type: IssueType::Text,
                format_type: IssueFormatType::Periodical,
                digitizing_institution: digitizing_instituion.clone(),
                rights_statement: rights_statement.clone(),
            };
            issues.push(issue);
        }

        let parent_collection = issues.first().unwrap().parent_collection.clone();

        let periodical = Periodical {
            description: prompt_user_input(
                format!("Enter the description of the \"{}\" periodical:", parent_collection).as_str()
            ),
            collection: collection.clone(),
            contributing_institution: contributing_institution.clone(),
            issues,
            topics: select_topics_with_retries(&parent_collection),
        };

        periodicals.push(periodical);
    }
    Ok(periodicals)
}
