use std::error::Error;

use crate::parse::parse_util::*;
use super::*;

pub fn parse_dc_command(line: &str) -> Result<Command, Box<dyn Error>>
{
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() != 5 {
        return Err(format!("Invalid .DC command format: {:?}", tokens).into());
    }
    Ok(Command::DC {
        source: tokens[1].to_string(),
        start: parse_value(tokens[2])?,
        stop: parse_value(tokens[3])?,
        step: parse_value(tokens[4])?,
    })
}