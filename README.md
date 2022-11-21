# Daily Paper
Given a list of papers and a list of notes (in any format), opens a random paper without an associated org-roam note. 

# Requirements
 - A folder with all your papers pdf
 - A (separate) folder with all your paper notes
 - One paper note per paper
 - The same name for the pdf and the note (for instance: `mccarthy1955proposal.pdf` and `mccarty1955proposal.md`)

# How to use
## Configuration
 - In file `config/config.json`, change the paper and note folders
 - If you wish to filter your paper folder using a regular expression (for instance if you have more than one type of papers and want to restrict to journal paper), add it in the `regex` field. If you don't need this functionality, remove the field. 

## Use the Binaries
 - From the `bin` folder, download the archive. For now, only Linux binaries are supported. 
 - Extract the content of the archive
 - Configure (see Configuration section)
 - Add `path/of/the/extracted/folder/target/release/daily-paper` to your startup applications and enjoy your daily paper!

## Use the Source code
 - To use the tool through source code, you must have Rust installed on your machine
 - Clone the repository 
 - Configure (see Configuration section)
 - Run `cargo build --release`
 - Add the generated executable to your startup applications