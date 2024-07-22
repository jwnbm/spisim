use std::error::Error;
use std::fmt::Debug;

use crate::parse::parse_util::*;

#[derive(Debug)]
pub enum Command
{
    DC {
        source: String,
        start: f64,
        stop: f64,
        step: f64
    },
}

mod dc;

pub fn parse_command(line: &str) -> Result<Command, Box<dyn Error>> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    match tokens[0] {
        ".DC" => dc::parse_dc_command(&line),
        // 他の解析コマンドもここに追加できます
        _ => Err(format!("Unknown analysis command: {:?}", tokens).into()),
    }
}