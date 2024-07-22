use std::error::Error;

use super::*;

pub fn parse_inductor(line: &str) -> Result<Element, Box<dyn Error>>
{
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() < 4 {
        return Err(format!("Syntax error: {}", line).into());
    }

    let name: String = tokens[0].to_string();
    let node1: String = tokens[1].to_string();
    let node2: String = tokens[2].to_string();
    let mut index: usize = 3;
    let ( model, inductance, ) = match parse_named_value(tokens[index], "L", &["H"]) {
        Ok(v) => ( None, v ),
        _ => {
            index = 4;
            let ind = match tokens.get(index) {
                Some(value) => parse_named_value(value, "L", &["H"])?,
                None => 0.0
            };
            ( Some(tokens[3].to_string()), ind )
        }
    };
    index += 1;
    let initial: Option<f64> = match tokens.get(index) {
        Some(value) => Some(parse_named_value(value, "IC", &["A", "a"])?),
        None => None,
    };
    Ok(Element::Inductor {
        name,
        node1,
        node2,
        model,
        inductance,
        initial,
    })
}
