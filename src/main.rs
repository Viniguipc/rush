use std::io::{self, Write};
use std::env;

fn main() {
	let mut input = String::new();
	
	println!("Rush");
	
    loop{
		let dir_atual = env::current_dir().unwrap();
		
        print!("{} > ", dir_atual.display());
        io::stdout().flush().unwrap();
        
        input.clear();
        
        io::stdin().read_line(&mut input).unwrap();
        
        print!("Você digitou: {}", input);
    }
}
