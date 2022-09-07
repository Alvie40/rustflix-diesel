use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stock: String = "AAPL".to_string();

    if args.len() < 2 {
        eprintln!("Please specify an argument")
    } else {
        stock = args[1].clone()
    }
    print!("args {:?}", stock);






}
