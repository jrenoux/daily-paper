mod data_model;

use std::ffi::OsStr;
use std::fs;
use std::ops::Deref;
use regex::Regex;
use rand::seq::SliceRandom;
use crate::data_model::Config;

fn main() {
    // 1. Read the config file to get the folders location
    let config_file: String = "config/config.json".to_string();
    let config_str = fs::read_to_string(&config_file).expect(format!("Config file {} could not be read", &config_file).as_str());

    let config: Config = serde_json::from_str(&config_str).unwrap();
    println!("{:?}", config);

    // we create the regex for notes and papers (there are stuff in this folder which are not papers)
    let re = Regex::new(r"^[[:alpha:]]+\d{4}.*$").unwrap(); // TODO Change here if your format is different

    // 2. Get all paper names in the paper folder
    let papers_path = fs::read_dir(&config.paper_folder).expect(format!("Could not read folder {} ", &config.paper_folder).as_str());
    let mut paper_names: Vec<String> = papers_path
        .map(|p| p.unwrap().path().file_stem().unwrap().to_str().unwrap().to_string())
        .filter(|pn| re.is_match(pn))
        .collect();
    paper_names.sort();
    // 3. Get all notes name in the note folder
    let notes_path = fs::read_dir(&config.note_folder).expect(format!("Could not read folder {} ", &config.note_folder).as_str());

    let mut note_names : Vec<String> = notes_path
        .map(|n| n.unwrap().path().file_stem().unwrap().to_str().unwrap().to_string())
        .filter(|nn|re.is_match(nn))
        .collect();
    note_names.sort();

    // 4. Filter out the ones that have a note
    println!("{} files in papers", paper_names.len());
    println!("{} files in notes", note_names.len());

    let mut papers_without_note = vec![];
    let mut note_without_paper = vec![];
    for p in &paper_names {
        if !note_names.contains(&p) {
            papers_without_note.push(p);
        }
    }
    papers_without_note.sort();

    for n in &note_names {
        if !paper_names.contains(n) {
            note_without_paper.push(n)
        }
    }
    note_without_paper.sort();

    println!("{} papers without a note", papers_without_note.len());
    println!("{} notes without a paper", note_without_paper.len());
    // TODO weird stuff here as most notes appear twice in the vector

    // 5. select one random paper
    match papers_without_note.choose(&mut rand::thread_rng()) {
        Some(paper) => {
            // 6. open it
            let path = format!("{}/{}.pdf", &config.paper_folder, paper );
            println!("{}", path);
            open::that(path);
        }
        None => {}
    }
}
