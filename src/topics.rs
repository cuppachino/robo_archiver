use dialoguer::{ theme::ColorfulTheme, MultiSelect };

fn load_topics() -> Vec<String> {
    include_str!("../topics")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

enum TopicError {
    NotEnough(usize),
    TooMany(usize),
}

fn select_topics(parent_collection: &str) -> Result<Vec<String>, TopicError> {
    const MIN_SELECTIONS: usize = 3;
    let multiselected = load_topics();
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(
            format!("Select 3 topics from the vocabulary list for the \"{}\" collection.", parent_collection)
        )
        .items(&multiselected[..])
        .interact()
        .unwrap();

    let selected_len = selections.len();
    if selected_len < MIN_SELECTIONS {
        return Err(TopicError::NotEnough(selected_len));
    } else if selected_len > MIN_SELECTIONS {
        return Err(TopicError::TooMany(selected_len));
    } else {
        let topics = selections
            .iter()
            .map(|i| multiselected[*i].to_string())
            .collect::<Vec<_>>();
        Ok(topics)
    }
}

pub fn select_topics_with_retries(parent_collection: &str) -> Vec<String> {
    loop {
        match select_topics(parent_collection) {
            Ok(topics) => {
                return topics;
            }
            Err(TopicError::NotEnough(selected_len)) => {
                println!("You need to select at least 3 topics. You selected {} Press [ENTER] to retry.", selected_len);
                // wait for a key press
                let _ = std::io::stdin().read_line(&mut String::new());
            }
            Err(TopicError::TooMany(selected_len)) => {
                println!("You may select at most 3 topics. You selected {}. Press [ENTER] to retry.", selected_len);
                // wait for a key press
                let _ = std::io::stdin().read_line(&mut String::new());
            }
        }
    }
}
