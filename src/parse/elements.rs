use std::error::Error;
use std::fmt::Debug;

pub mod element;
pub mod resistor;
pub mod capacitor;
pub mod inductor;
pub mod inputsource;
pub mod voltagesource;

pub fn parse_value(value: &str) -> Result<f64, Box<dyn Error>> {
    if value.ends_with("MEG") {
        let numeric_part: f64 = value[..value.len()-3].parse()?;
        return Ok(numeric_part * 1e6);
    }

    let last_char: Option<char> = value.chars().last();
    let multiplier: f64 = match last_char {
        Some('k') => 1e3,
        Some('M') => 1e6,
        Some('G') => 1e9,
        Some('m') => 1e-3,
        Some('u') => 1e-6,
        Some('n') => 1e-9,
        Some('p') => 1e-12,
        Some(_) | None => 1.0,
    };

    let numeric_part: f64 = if multiplier == 1.0 {
        value.parse()?
    } else {
        value[..value.len()-1].parse()?
    };
    Ok(numeric_part * multiplier)
}

pub fn parse_unitvalue(value: &str, units: &[&str]) -> Result<f64, Box<dyn Error>> {
    let unit: Option<&&str> = units.iter().find(|&&unit| value.ends_with(unit));
    let val: &str = match unit {
        Some(unit) => &value[..value.len() - unit.len()],
        None => value,
    };
    parse_value(val)
}

pub fn parse_named_value(value: &str, name: &str, units: &[&str]) -> Result<f64, Box<dyn Error>> {
    if value.len() < 3 {
        return parse_unitvalue(value, units)
    }
    let prefix: String = format!("{}=", name);
    let val: &str = if value.starts_with(&prefix) {
        &value[prefix.len()..]
    } else {
        value
    };
    parse_unitvalue(val, units)
}
