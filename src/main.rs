pub mod data;
pub mod marc;
pub mod cli;
pub mod topics;
pub mod files;

use clap::Parser;
use files::{ load_directory, process_files };
use robo_archiver::ArchiveError;
use crate::{
    cli::Args,
    data::*,
    marc::{ accept_marc, parse_marc },
    topics::select_topics_with_retries,
};

fn main() -> Result<(), ArchiveError> {
    // Parse command line arguments.
    let args = Args::parse();
    let call_number = match args.call_number {
        Some(call_number) => CallNumber::Shelf(call_number),
        None => CallNumber::Periodical,
    };
    let collection = args.collection.map(PeriodicalCollection::from).unwrap_or_default();
    let contributing_institution = args.contributing_institution
        .map(ContributingInstitution::from)
        .unwrap_or_default();
    let rights_statement = args.rights_statement.map(RightsStatement::from).unwrap_or_default();
    let digitizing_instituion = args.digitization_institution
        .map(DigitizingInstitution::from)
        .unwrap_or_default();

    // Parse user input for MARC record.
    let marc = {
        let marc = accept_marc();
        let marc = std::io::BufReader::new(marc.as_bytes());
        parse_marc(call_number, marc).unwrap()
    };

    let periodicals = {
        let path = args.file_dir.unwrap_or_else(|| ".".to_string());
        let is_recursive = args.recursive;
        process_periodicals(
            path,
            is_recursive,
            marc,
            digitizing_instituion,
            rights_statement,
            collection,
            contributing_institution
        )?
    };

    Ok(())
}

fn prompt_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn process_periodicals(
    path: String,
    is_recursive: bool,
    marc: MarcData,
    digitizing_instituion: DigitizingInstitution,
    rights_statement: RightsStatement,
    collection: PeriodicalCollection,
    contributing_institution: ContributingInstitution
) -> Result<Vec<Periodical>, ArchiveError> {
    let file_paths = load_directory(path, is_recursive);
    let data = process_files(file_paths)?;
    let mut periodicals: Vec<Periodical> = Vec::new();
    for issue_datas in data.iter() {
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
                languages: Vec::new(),
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
