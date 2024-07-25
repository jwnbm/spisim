use std::{collections::HashMap, f64::INFINITY};
use inputsource::InputSource;
use nalgebra::{DMatrix, DVector};

use crate::parse::elements::*;

pub fn build_node_map(elements: &[Element]) -> HashMap<String, usize> {
    let mut node_map: HashMap<String, usize> = HashMap::new();
    let mut current_index: usize = 0;
    for element in elements {
        match element {
            Element::Resistor { node1, node2, .. }
            | Element::Capacitor { node1, node2, .. }
            | Element::Inductor { node1, node2, .. }
            | Element::VoltageSource { node1, node2, .. } => {
                if !node_map.contains_key(node1) && node1 != "0" {
                    node_map.insert(node1.clone(), current_index);
                    current_index += 1;
                }
                if !node_map.contains_key(node2) && node2 != "0" {
                    node_map.insert(node2.clone(), current_index);
                    current_index += 1;
                }
            }
        }
    }
    node_map
}

pub fn build_op_matrix(elements: &[Element], node_map: &HashMap<String, usize>) -> (DMatrix<f64>, DVector<f64>) {
    let num_nodes: usize = node_map.len();
    let voltage_sources: Vec<&Element> = elements.iter().filter(|e| matches!(e, Element::VoltageSource { .. })).collect();
    let voltage_count: usize = voltage_sources.len();
    let size: usize = num_nodes + voltage_count;

    let mut matrix = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);

    for element in elements {
        match element {
            Element::Resistor { node1, node2, resistance, .. } => {
                let conductance: f64 = 1.0 / resistance;
                if node1 != "0" && node2 != "0" {
                    let idx1: usize = *node_map.get(node1).unwrap();
                    let idx2: usize = *node_map.get(node2).unwrap();
                    matrix[(idx1, idx1)] += conductance;
                    matrix[(idx2, idx2)] += conductance;
                    matrix[(idx1, idx2)] -= conductance;
                    matrix[(idx2, idx1)] -= conductance;
                } else if node1 != "0" {
                    let idx1: usize = *node_map.get(node1).unwrap();
                    matrix[(idx1, idx1)] += conductance;
                } else if node2 != "0" {
                    let idx2: usize = *node_map.get(node2).unwrap();
                    matrix[(idx2, idx2)] += conductance;
                }
            }
            Element::Capacitor { .. } => {
            }
            Element::Inductor { node1, node2, .. } => {
                if node1 != "0" && node2 != "0" {
                    let idx1: usize = *node_map.get(node1).unwrap();
                    let idx2: usize = *node_map.get(node2).unwrap();
                    matrix[(idx1, idx1)] += 1.0;
                    matrix[(idx2, idx2)] += 1.0;
                    matrix[(idx1, idx2)] -= 1.0;
                    matrix[(idx2, idx1)] -= 1.0;
                } else if node1 != "0" {
                    let idx1: usize = *node_map.get(node1).unwrap();
                    matrix[(idx1, idx1)] += 1.0;
                } else if node2 != "0" {
                    let idx2: usize = *node_map.get(node2).unwrap();
                    matrix[(idx2, idx2)] += 1.0;
                }
            }
            Element::VoltageSource { name, node1, node2, vtype, } => {
                let vs_idx: usize = num_nodes
                    + voltage_sources.iter()
                    .position(|e| {
                        if let Element::VoltageSource { name: e_name, .. } = e {
                            e_name == name
                        } else {
                            false
                        }
                    })
                    .unwrap();

                if node1 != "0" {
                    let idx1: usize = *node_map.get(node1).unwrap();
                    matrix[(idx1, vs_idx)] = 1.0;
                    matrix[(vs_idx, idx1)] = 1.0;
                }
                if node2 != "0" {
                    let idx2: usize = *node_map.get(node2).unwrap();
                    matrix[(idx2, vs_idx)] = -1.0;
                    matrix[(vs_idx, idx2)] = -1.0;
                }
                let voltage: f64 = match vtype {
                    InputSource::DC { voltage } => *voltage,
                    _ => 0.0,
                };
                rhs[vs_idx] = voltage;
            }
        }
    }

    (matrix, rhs)
}
