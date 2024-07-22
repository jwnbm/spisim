use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod elements;
use elements::{
    Element,
    resistor::parse_resistor,
    capacitor::parse_capacitor,
    inductor::parse_inductor,
    voltagesource::parse_voltage_source,
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

        let element: Element = match line.chars().next() {
            Some('R') => parse_resistor(&line)?,
            Some('C') => parse_capacitor(&line)?,
            Some('L') => parse_inductor(&line)?,
            Some('V') => parse_voltage_source(&line)?,
            // 他の要素も必要に応じて追加
            Some(_) | None => return Err(format!("Unknown element type: {}", line).into()),
        };
        elements.push(element);
    }

    Ok(elements)
}
