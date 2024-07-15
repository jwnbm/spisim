use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod elements;
use elements::{
    element::Element,
    resistor::Resistor,
    capacitor::Capacitor,
    inductor::Inductor,
    voltagesource::VoltageSource,
 };

pub fn parse_netlist(filename: &str) -> Result<Vec<Box<dyn Element>>, Box<dyn Error>> {
    let path: &Path = Path::new(filename);
    let file: File = File::open(&path)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut elements: Vec<Box<dyn Element>> = Vec::new();

    for line in reader.lines() {
        let line: String = line?.trim().to_string();
        if line.is_empty() || line.starts_with('*') {
            continue; // コメント行や空行をスキップ
        }

        let element: Box<dyn Element> = match line.chars().next() {
            Some('R') => Resistor::new(&line)?,
            Some('C') => Capacitor::new(&line)?,
            Some('L') => Inductor::new(&line)?,
            Some('V') => VoltageSource::new(&line)?,
            // 他の要素も必要に応じて追加
            Some(_) | None => return Err(format!("Unknown element type: {}", line).into()),
        };
        elements.push(element);
    }

    Ok(elements)
}
