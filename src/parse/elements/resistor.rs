use std::error::Error;

use super::*;

pub fn parse_resistor(line: &str) -> Result<Element, Box<dyn Error>>
{
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() < 4 {
        return Err(format!("Syntax error: {}", line).into());
    }

    let name: String = tokens[0].to_string();
    let node1: String = tokens[1].to_string();
    let node2: String = tokens[2].to_string();
    let mut index: usize = 3;
    let (model, resistance) = match parse_named_value(tokens[index], "R", &[]) {
        Ok(v) => ( None, v ),
        _ => {
            index = 4;
            let res: f64 = match tokens.get(index) {
                Some(value) => parse_named_value(value, "R", &[])?,
                None => 0.0,
            };
            ( Some(tokens[3].to_string()), res )
        }
    };
    Ok(Element::Resistor {
        name,
        node1,
        node2,
        model,
        resistance,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_resistor_pass_normal() -> Result<(), Box<dyn Error>> {
        let actual: Element = parse_resistor("R1 N1 N2 1K")?;
        match actual {
            Element::Resistor { name, node1, node2, model, resistance } => {
                assert_eq!(name, "R1");
                assert_eq!(node1, "N1");
                assert_eq!(node2, "N2");
                assert!(model.is_none());
                assert_eq!(resistance, 1e3);
            }
            _ => {
                panic!();
            }
        }
        Ok(())
    }
}
