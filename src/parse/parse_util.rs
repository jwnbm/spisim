use std::error::Error;

/// SI接頭辞を考慮して文字列を数値に変換して返す。
/// ## Example
/// ```
/// assert_eq!(parse_value("2.0"), 2.0);
/// assert_eq!(parse_value(".03"), 0.03);
/// assert_eq!(parse_value("4e3"), 4e3);
/// assert_eq!(parse_value("4u"), 4e-6);
/// assert_eq!(parse_value("4MEG"), 4e6);
/// ```
pub fn parse_value(value: &str) -> Result<f64, Box<dyn Error>> {
    if value.ends_with("MEG") {
        let numeric_part: f64 = value[..value.len()-3].parse()?;
        return Ok(numeric_part * 1e6);
    }

    let last_char: Option<char> = value.chars().last();
    let multiplier: f64 = match last_char {
        Some('k') | Some('K') => 1e3,
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

/// 単位付きの文字列を数値に変換して返す。
/// ## Example
/// ```
/// assert_eq!(parse_value("2.0V", &["V", "v"]), 2.0);
/// assert_eq!(parse_value(".03A", &["A"]), 0.03);
/// assert_eq!(parse_value("4e3", &["s"]), 4e3);
/// assert_eq!(parse_value("4uF", &["F"]), 4e-6);
/// ```
pub fn parse_unitvalue(value: &str, units: &[&str]) -> Result<f64, Box<dyn Error>> {
    let unit: Option<&&str> = units.iter().find(|&&unit| value.ends_with(unit));
    let val: &str = match unit {
        Some(unit) => &value[..value.len() - unit.len()],
        None => value,
    };
    parse_value(val)
}

/// 名前付きの文字列を数値に変換して返す。
/// ## Example
/// ```
/// assert_eq!(parse_value("2.0V", "V1", &["V", "v"]), 2.0);
/// assert_eq!(parse_value("IC=.03A", "IC", &["A"]), 0.03);
/// assert_eq!(parse_value("START=4e3", "START", &["s"]), 4e3);
/// assert_eq!(parse_value("C=4uF", "C", &["F"]), 4e-6);
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value_pass_normal() -> Result<(), Box<dyn Error>> {
        
        let actual: f64 = parse_value("2.0")?;
        let expected: f64 = 2.0;
        assert_eq!(actual, expected, "we are testing parse_value with {} and {}", actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse_value_pass_start_point() -> Result<(), Box<dyn Error>> {
        let actual: f64 = parse_value(".03")?;
        let expected: f64 = 0.03;
        assert_eq!(actual, expected, "we are testing parse_value with {} and {}", actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse_value_pass_exp() -> Result<(), Box<dyn Error>> {
        let actual: f64 = parse_value("4e3")?;
        let expected: f64 = 4e3;
        assert_eq!(actual, expected, "we are testing parse_value with {} and {}", actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse_value_pass_u() -> Result<(), Box<dyn Error>> {
        let actual: f64 = parse_value("4u")?;
        let expected: f64 = 4e-6;
        assert_eq!(actual, expected, "we are testing parse_value with {} and {}", actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse_value_pass_meg() -> Result<(), Box<dyn Error>> {
        let actual: f64 = parse_value("4MEG")?;
        let expected: f64 = 4e6;
        assert_eq!(actual, expected, "we are testing parse_value with {} and {}", actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse_value_error() -> Result<(), Box<dyn Error>> {
        let actual:Result<f64, Box<dyn Error>> = parse_value("a");
        assert!(actual.is_err(), "Expected an error, but got a result: {:?}", actual);
        Ok(())
    }

    #[test]
    fn test_parse_unitvalue_pass_normal() -> Result<(), Box<dyn Error>> {
        
        let actual: f64 = parse_unitvalue("2.0V", &["V", "v"])?;
        let expected: f64 = 2.0;
        assert_eq!(actual, expected, "we are testing parse_value with {} and {}", actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse_unitvalue_pass_start_point() -> Result<(), Box<dyn Error>> {
        let actual: f64 = parse_unitvalue(".03A", &["A"])?;
        let expected: f64 = 0.03;
        assert_eq!(actual, expected, "we are testing parse_value with {} and {}", actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse_unitvalue_pass_exp() -> Result<(), Box<dyn Error>> {
        let actual: f64 = parse_unitvalue("4e3", &["s"])?;
        let expected: f64 = 4e3;
        assert_eq!(actual, expected, "we are testing parse_value with {} and {}", actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse_unitvalue_pass_u() -> Result<(), Box<dyn Error>> {
        let actual: f64 = parse_unitvalue("4uF", &["F"])?;
        let expected: f64 = 4e-6;
        assert_eq!(actual, expected, "we are testing parse_value with {} and {}", actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse_unitvalue_pass_meg() -> Result<(), Box<dyn Error>> {
        let actual: f64 = parse_unitvalue("4MEGHz", &["Hz"])?;
        let expected: f64 = 4e6;
        assert_eq!(actual, expected, "we are testing parse_value with {} and {}", actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse_unitvalue_error() -> Result<(), Box<dyn Error>> {
        let actual:Result<f64, Box<dyn Error>> = parse_unitvalue("a", &["Hz"]);
        assert!(actual.is_err(), "Expected an error, but got a result: {:?}", actual);
        Ok(())
    }

    #[test]
    fn test_parse_named_value_pass_normal() -> Result<(), Box<dyn Error>> {
        
        let actual: f64 = parse_named_value("IC=2.0V", "IC", &["V", "v"])?;
        let expected: f64 = 2.0;
        assert_eq!(actual, expected, "we are testing parse_value with {} and {}", actual, expected);
        Ok(())
    }
}
