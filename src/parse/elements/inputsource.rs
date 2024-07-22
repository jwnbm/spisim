use std::error::Error;

use super::*;

#[derive(Debug)]
pub enum InputSource {
    DC {
        voltage: f64,
    },
    AC {
        mag: f64,
        phase: Option<f64>,
    },
    Pulse {
        v1: f64,
        v2: f64,
        tdelay: f64,
        trise: f64,
        tfall: f64,
        width: f64,
        period: f64,
    },
    Sin {
        voffset: f64,
        vpeak: f64,
        freq: f64,
        tdelay: Option<f64>,
        damp_factor: Option<f64>,
        phase: Option<f64>,
    },
    Exp {
        v1: f64,
        v2: f64,
        trise_delay: f64,
        tau_rise: f64,
        tfall_delay: f64,
        tau_fall: f64,
    },
    PWL {
        points: Vec<(f64, f64)>,
    },
    SFFM {
        voffset: f64,
        vpeak: f64,
        fcarrier: f64,
        mod_index: f64,
        fsignal: f64,
    }
}

pub fn parse_input_source(token: &str, is_voltage: bool) -> Result<InputSource, Box<dyn Error>> {

    fn extract_parentheses_content(text: &str) -> Option<&str> {
        if let Some(start) = text.find('(') {
            if let Some(end) = text[start..].find(')') {
                return Some(&text[start + 1..start + end]);
            }
        }
        None
    }

    fn parse_param(params: &[&str], index: usize, unit: &[&str], token: &str) -> Result<f64, Box<dyn std::error::Error>> {
        match params.get(index) {
            Some(value) => parse_unitvalue(value, unit).map_err(|_| format!("Syntax error: {}", token).into()),
            None => Err(format!("Syntax error: {}", token).into()),
        }
    }

    let units: [&str; 2] = if is_voltage { ["V", "v"] } else { ["A", "a"] };

    if token.starts_with("DC") {
        let tokens: Vec<&str> = token.split_whitespace().collect();
        let voltage: f64 = parse_param(&tokens,1, &units, token)?;
        Ok(InputSource::DC { voltage })
    }
    else if token.starts_with("AC") {
        let tokens: Vec<&str> = token.split_whitespace().collect();
        let mag: f64 = parse_param(&tokens,1, &units, token)?;
        let phase: Option<f64> = match parse_param(&tokens,1, &units, token) {
            Ok(value) => Some(value),
            _ => None,
        };
        Ok(InputSource::AC { mag, phase })
    }
    else if token.starts_with("PULSE") {
        let params: Vec<&str> = match extract_parentheses_content(token) {
            Some(content) => content.split_whitespace().collect(),
            None => return Err(format!("Syntax error: {}", token).into())
        };
        let v1: f64 = parse_param(&params, 0, &units, token)?;
        let v2: f64 = parse_param(&params, 1, &units, token)?;
        let tdelay: f64 = parse_param(&params, 2, &["s"], token)?;
        let trise: f64 = parse_param(&params, 3, &["s"], token)?;
        let tfall: f64 = parse_param(&params, 4, &["s"], token)?;
        let width: f64 = parse_param(&params, 5, &["s"], token)?;
        let period: f64 = parse_param(&params, 6, &["s"], token)?;
        Ok(InputSource::Pulse { v1, v2, tdelay, trise, tfall, width, period })
    }
    else if token.starts_with("SIN") {
        let params: Vec<&str> = match extract_parentheses_content(token) {
            Some(content) => content.split_whitespace().collect(),
            None => return Err(format!("Syntax error: {}", token).into())
        };
        let voffset: f64 = parse_param(&params, 0, &units, token)?;
        let vpeak: f64 = parse_param(&params, 1, &units, token)?;
        let freq: f64 = parse_param(&params, 2, &["Hz"], token)?;
        let tdelay: Option<f64> = match parse_param(&params, 3, &["s"], token) {
            Ok(v) => Some(v),
            _ => None,
        };
        let damp_factor: Option<f64> = match parse_param(&params, 4, &["s"], token) {
            Ok(v) => Some(v),
            _ => None,
        };
        let phase: Option<f64> = match parse_param(&params, 4, &["s"], token) {
            Ok(v) => Some(v),
            _ => None,
        };
        Ok(InputSource::Sin{ voffset, vpeak, freq, tdelay, damp_factor, phase })
    }
    else if token.starts_with("EXP") {
        let params: Vec<&str> = match extract_parentheses_content(token) {
            Some(content) => content.split_whitespace().collect(),
            None => return Err(format!("Syntax error: {}", token).into())
        };
        let v1: f64 = parse_param(&params, 0, &units, token)?;
        let v2: f64 = parse_param(&params, 1, &units, token)?;
        let trise_delay: f64 = parse_param(&params, 2, &["s"], token)?;
        let tau_rise: f64 = parse_param(&params, 3, &["s"], token)?;
        let tfall_delay: f64 = parse_param(&params, 4, &["s"], token)?;
        let tau_fall: f64 = parse_param(&params, 5, &["s"], token)?;
        Ok(InputSource::Exp{ v1, v2, trise_delay, tau_rise, tfall_delay, tau_fall })
    }
    else if token.starts_with("PWL") {
        let params: Vec<&str> = match extract_parentheses_content(token) {
            Some(content) => content.split_whitespace().collect(),
            None => return Err(format!("Syntax error: {}", token).into())
        };
        let mut points: Vec<f64> = Vec::new();
        let mut index: usize = 0;
        let units = if is_voltage { ["V", "v", "s"] } else { ["A", "a", "s"] };
        while let Ok(v) = parse_param(&params, index, &units, token) {
            points.push(v);
            index += 1;
        }
        let points: Vec<(f64, f64)> = points.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
        Ok(InputSource::PWL{ points })
    }
    else if token.starts_with("SFFM") {
        let params: Vec<&str> = match extract_parentheses_content(token) {
            Some(content) => content.split_whitespace().collect(),
            None => return Err(format!("Syntax error: {}", token).into())
        };
        let voffset: f64 = parse_param(&params, 0, &units, token)?;
        let vpeak: f64 = parse_param(&params, 1, &units, token)?;
        let fcarrier: f64 = parse_param(&params, 2, &["s"], token)?;
        let mod_index: f64 = parse_param(&params, 3, &[], token)?;
        let fsignal: f64 = parse_param(&params, 4, &[], token)?;
        Ok(InputSource::SFFM{ voffset, vpeak, fcarrier, mod_index, fsignal })
    }
    else {
        let tokens: Vec<&str> = token.split_whitespace().collect();
        let voltage: f64 = parse_param(&tokens,0, &units, token)?;
        Ok(InputSource::DC { voltage })
    }
}
