use std::env;

mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse::parse_netlist(&args[1]) {
        Ok(netlist) => {
            println!("elements");
            for element in netlist.elements {
                println!("{:?}", element);
            }
            println!("commands");
            for command in netlist.commands {
                println!("{:?}", command);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
