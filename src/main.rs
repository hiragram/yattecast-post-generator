#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate chrono;

use std::io::{self, BufRead, Write, BufReader};
use std::collections::BTreeMap;
use std::fs::File;
use chrono::{Utc, Local, DateTime, Date};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Metadata<'a> {
    title: &'a str,
    description: &'a str,
    actors: Vec<&'a str>,
    audio_file_path: &'a str,
    audio_file_size: i32,
    duration: i32,
    date: &'a str
}

fn main() {
    // ask title
    print!("title? : "); io::stdout().flush();
    let title = read();

    // ask description
    print!("description? : "); io::stdout().flush();
    let description = read();

    // ask actor
    print!("actors?(comma separated) : "); io::stdout().flush();
    let actors_raw = read();
    let actors = actors_raw.split(",").collect::<Vec<&str>>();

    // ask audio file path
    print!("audio file path? : "); io::stdout().flush();
    let audio_source_path = read();

    // build metadata
    let meta = Metadata {
        title: &title,
        description: &description,
        actors: actors,
        audio_file_path: "",
        audio_file_size: 0,
        duration: 0,
        date: &Utc::now().format("%Y-%m-%d %H:%M:%S +0000").to_string(),
    };

    let yaml_str = serde_yaml::to_string(&meta).unwrap();

    // ask reference urls
    print!("reference URLs?(comma separated) : "); io::stdout().flush();
    let references_raw = read();
    let references = references_raw.split(",").collect::<Vec<&str>>();

    let references_section = format!("## References\n\n{}", references.join("\n"));

    let content_str = format!("{}\n---\n\n{}", yaml_str, references_section);

    // write file
    let mut file = File::create("./post.md").unwrap();
    write!(file, "{}", content_str);
    file.flush();
}

fn read() -> std::string::String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    return s.trim().parse().ok().unwrap();
}
