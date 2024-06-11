use std::io::BufRead;

use robo_archiver::ArchiveError;
use crate::data::{ MarcData, CallNumber };

#[derive(Debug)]
pub struct Subfield {
    code: char,
    value: String,
}

#[derive(Debug)]
pub struct Record {
    tag: String,
    ind: String,
    subfields: Vec<Subfield>,
}

/// This function prompts a user to input a string, which should be a valid MARC record with line breaks.
/// After the user pastes the MARC record, and submits an empty line, the function returns the input.
pub fn accept_marc() -> String {
    println!("Paste a MARC record:");
    let mut marc = String::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            println!("MARC record accepted.");
            println!("{}", "-".repeat(termsize::get().unwrap().cols as usize));
            break;
        }
        marc.push_str(&line);
        marc.push_str("\n");
    }
    marc
}

pub fn parse_marc<B>(call_number: CallNumber, buffered: B) -> Result<MarcData, ArchiveError>
    where B: BufRead
{
    let mut records = Vec::new();

    for line in buffered.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 3 {
            let subfields_str = parts[2];
            let mut subfields = Vec::new();
            let subfields_parts: Vec<&str> = subfields_str.split('$').collect();
            for subfield in subfields_parts.into_iter().skip(1) {
                // skip the first empty part
                let code = subfield.chars().next().unwrap();
                let value = subfield[1..].to_string();
                subfields.push(Subfield { code, value });
            }

            if subfields.is_empty() {
                // then this is probably a serial record, such as the ocn893691141 or OCoLC.
                // we can just push the whole thing as a subfield.
                subfields.push(Subfield {
                    code: '_',
                    value: subfields_str.to_string(),
                });
            }

            let record = Record {
                tag: parts[0].to_string(),
                ind: parts[1].to_string(),
                subfields,
            };
            records.push(record);
        }
    }

    MarcData::try_from_records(call_number, records)
}

impl MarcData {
    fn try_from_records(
        call_number: CallNumber,
        records: Vec<Record>
    ) -> Result<Self, ArchiveError> {
        // marc 100, 110, 700, 710
        let mut creators: Vec<String> = Vec::new();
        // marc 260 or 264.b
        let mut publisher: String = String::new();
        // marc 610, 650, possibly any 600.
        let mut subject_headings: Vec<String> = Vec::new();
        // marc 001 or 003.
        let mut oclc_number: Option<String> = None;

        for record in records {
            match record.tag.as_str() {
                "700" => {
                    creators.extend(
                        record.subfields
                            .iter()
                            .filter(|sf| sf.code == 'a')
                            .map(|sf| {
                                // split on comma and reverse the order.
                                // e.g. "Smith, John A. (Date)" -> "John A. Smith (Date)"
                                let parts = sf.value.split(", ").collect::<Vec<&str>>();
                                let string = if parts.len() == 2 {
                                    format!("{} {}", parts[1], parts[0])
                                } else {
                                    sf.value.clone()
                                };
                                string.trim_end_matches(is_grammatical_punctuation).to_string()
                            })
                    );
                }
                "100" | "110" | "710" => {
                    creators.extend(
                        record.subfields
                            .iter()
                            .filter(|sf| sf.code == 'a')
                            .map(|sf|
                                sf.value
                                    .clone()
                                    .trim_end_matches(is_grammatical_punctuation)
                                    .to_string()
                            )
                    );
                }
                "260" | "264" => {
                    publisher = record.subfields
                        .iter()
                        .filter(|sf| sf.code == 'b')
                        .map(|sf| sf.value.trim_end_matches(is_grammatical_punctuation).to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                        .trim_end_matches(is_grammatical_punctuation)
                        .to_string();
                }
                "610" | "650" => {
                    let mut subject = Vec::default();
                    for Subfield { code, value } in record.subfields {
                        match code {
                            'a' => subject.push(value),
                            'x' | 'v' | 'z' => {
                                subject.push("--".to_string()); // em dash
                                subject.push(value);
                            }
                            'd' => {
                                subject.push(", ".to_string()); // em dash
                                subject.push(value);
                            }
                            _ => {
                                println!("[WARN] UNKNOWN SUBFIELD CODE FOR SUBJECTS, IGNORING: {}", code);
                            }
                        }
                    }
                    subject_headings.push(
                        subject.join("").trim_end_matches(is_grammatical_punctuation).to_string()
                    );
                }
                "001" | "003" => {
                    if
                        oclc_number.is_none() &&
                        record.subfields.first().is_some_and(|sf| sf.code == '_')
                    {
                        let value = record.subfields.first().unwrap().value.clone();
                        // rip off any leading non-numeric characters
                        let value = value.trim_start_matches(|c: char| !c.is_numeric());
                        // pad the left side with zeros so that it's 9 characters long
                        let value = format!("{:0>9}", value);
                        oclc_number = Some(value);
                    }
                }
                _ => {}
            }
        }

        Ok(MarcData {
            call_number,
            creators,
            publisher,
            subject_headings,
            oclc_number: oclc_number.expect("No OCLC number found."),
        })
    }
}

fn is_grammatical_punctuation(c: char) -> bool {
    match c {
        '.' | ',' | ' ' | ';' => true,
        _ => false,
    }
}
