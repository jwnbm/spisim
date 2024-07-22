use std::error::Error;
use std::fmt::Debug;

pub mod inputsource;

#[derive(Debug)]
pub enum Element
{
    Capacitor {
        name: String,
        node1: String,
        node2: String,
        model: Option<String>,
        capacitance: f64,
        initial: Option<f64>,
    },
    Inductor {
        name: String,
        node1: String,
        node2: String,
        model: Option<String>,
        inductance: f64,
        initial: Option<f64>,
    },
    Resistor {
        name: String,
        node1: String,
        node2: String,
        model: Option<String>,
        resistance: f64,
    },
    VoltageSource {
        name: String,
        node1: String,
        node2: String,
        vtype: inputsource::InputSource,
    }
}

mod resistor;
mod capacitor;
mod inductor;
mod voltagesource;

pub fn parse_element(line: &str) -> Result<Element, Box<dyn Error>>
{
    match line.chars().next() {
        Some('R') => resistor::parse_resistor(&line),
        Some('C') => capacitor::parse_capacitor(&line),
        Some('L') => inductor::parse_inductor(&line),
        Some('V') => voltagesource::parse_voltage_source(&line),
        // 他の要素も必要に応じて追加
        Some(_) | None => return Err(format!("Unknown element type: {}", line).into()),
    }
}
