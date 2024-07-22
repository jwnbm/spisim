use std::error::Error;

use super::*;

pub fn parse_capacitor(line: &str) -> Result<Element, Box<dyn Error>>
{
    if !line.starts_with("C") {
        return Err(format!("Syntax error: {}", line).into());
    }
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() < 4 {
        return Err(format!("Syntax error: {}", line).into());
    }

    let name: String = tokens[0].to_string();
    let node1: String = tokens[1].to_string();
    let node2: String = tokens[2].to_string();
    let mut index: usize = 3;
    let ( model, capacitance ) = match parse_named_value(tokens[index], "C", &["F"]) {
        Ok(v) => ( None, v ),
        _ => {
            index = 4;
            let cap: f64 = match tokens.get(index) {
                Some(value) => parse_named_value(value, "C", &["F"])?,
                None => 0.0,
            };
            ( Some(tokens[3].to_string()), cap )
        }
    };
    index += 1;
    let initial: Option<f64> = match tokens.get(index) {
        Some(value) => Some(parse_named_value(value, "IC", &["V", "v"])?),
        None => None,
    };
    Ok(Element::Capacitor {
        name,
        node1,
        node2,
        model,
        capacitance,
        initial,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_capacitor_pass_normal() -> Result<(), Box<dyn Error>> {
        let actual: Element = parse_capacitor("CLOAD N1 N2 20pF")?;
        match actual {
            Element::Capacitor { name, node1, node2, model, capacitance, initial } => {
                assert_eq!(name, "CLOAD");
                assert_eq!(node1, "N1");
                assert_eq!(node2, "N2");
                assert!(model.is_none());
                assert_eq!(capacitance, 20e-12);
                assert!(initial.is_none());
            }
            _ => {
                panic!();
            }
        }
        Ok(())
    }
}
