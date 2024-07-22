use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod parse_util;
mod elements;
mod commands;

 #[derive(Debug)]
pub struct Netlist {
     pub elements: Vec<elements::Element>,
     pub commands: Vec<commands::Command>,
 }

pub fn parse_netlist(filename: &str) -> Result<Netlist, Box<dyn Error>> {
    let path: &Path = Path::new(filename);
    let file: File = File::open(&path)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut elements: Vec<elements::Element> = Vec::new();
    let mut commands: Vec<commands::Command> = Vec::new();

    for line in reader.lines() {
        let line: String = line?.trim().to_string();
        if line.is_empty() || line.starts_with('*') {
            continue; // コメント行や空行をスキップ
        }
        else if line.starts_with('.') {
            let command: commands::Command = commands::parse_command(&line)?;
            commands.push(command);
        }
        else {
            let element: elements::Element = elements::parse_element(&line)?;
            elements.push(element);
        }
    }

    Ok(Netlist { elements, commands })
}
