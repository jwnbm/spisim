use std::error::Error;

use super::*;
use element::*;
use inputsource::*;

#[derive(Debug)]
pub struct VoltageSource {
    name: String,
    node1: String,
    node2: String,
    vtype: Box<dyn InputSource>,
}

impl Element for VoltageSource {}

impl VoltageSource {
    pub fn new(line: &str) -> Result<Box<dyn Element>, Box<dyn Error>> {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() < 4 {
            return Err(format!("Syntax error: {}", line).into());
        }
        let name: String = tokens[0].to_string();
        let node1: String = tokens[1].to_string();
        let node2: String = tokens[2].to_string();
        let vtype:Box<dyn InputSource>  = parse_input_source(&tokens[3..].join(" "), true)?;
        Ok(Box::new(VoltageSource {
            name: name,
            node1: node1,
            node2: node2,
            vtype: vtype,
        }))
    }
}