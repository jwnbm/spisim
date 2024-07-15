use std::env;

mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse::parse_netlist(&args[1]) {
        Ok(elements) => {
            for element in elements {
                println!("{:?}", element);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
