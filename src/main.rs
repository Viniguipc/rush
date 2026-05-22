use std::io::{self, Write};

fn main() {
	let mut input = String::new();
	println!("Rush");
	
    loop{
        print!("~/rush > ");
        io::stdout().flush().unwrap();
        
        input.clear();
        
        io::stdin().read_line(&mut input).unwrap();
        
        print!("Você digitou: {}", input);
    }
}
