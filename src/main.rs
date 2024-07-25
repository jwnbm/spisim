use std::env;
use std::collections::HashMap;

mod parse;
mod build_matrix;

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse::parse_netlist(&args[1]) {
        Ok(netlist) => {
            println!("elements");
            for element in &netlist.elements {
                println!("{:?}", element);
            }
            println!("commands");
            for command in &netlist.commands {
                println!("{:?}", command);
            }
            let node_map: HashMap<String, usize> = build_matrix::build_node_map(&netlist.elements);
            let (matrix, rhs) = build_matrix::build_op_matrix(&netlist.elements, &node_map);

            if let Some(solution) = matrix.lu().solve(&rhs) {
                
                println!("Solution:");
                for i in 0..solution.len() {
                    println!("{:.6} ", solution[i]);
                }
            } else {
                println!("No solution found.");
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
