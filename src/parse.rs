use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod elements;
use elements::{
    Element,
    parse_element
 };

pub fn parse_netlist(filename: &str) -> Result<Vec<Element>, Box<dyn Error>> {
    let path: &Path = Path::new(filename);
    let file: File = File::open(&path)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut elements: Vec<Element> = Vec::new();

    for line in reader.lines() {
        let line: String = line?.trim().to_string();
        if line.is_empty() || line.starts_with('*') {
            continue; // コメント行や空行をスキップ
        }

        let element: Element = parse_element(&line)?;
        elements.push(element);
    }

    Ok(elements)
}
