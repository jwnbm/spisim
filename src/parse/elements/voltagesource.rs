use std::error::Error;

use super::*;
use inputsource::*;

pub fn parse_voltage_source(line: &str) -> Result<Element, Box<dyn Error>>
{
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() < 4 {
        return Err(format!("Syntax error: {}", line).into());
    }
    let name: String = tokens[0].to_string();
    let node1: String = tokens[1].to_string();
    let node2: String = tokens[2].to_string();
    let vtype:Box<dyn InputSource>  = parse_input_source(&tokens[3..].join(" "), true)?;
    Ok(Element::VoltageSource {
        name: name,
        node1: node1,
        node2: node2,
        vtype: vtype,
    })
}
