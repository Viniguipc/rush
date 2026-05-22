use std::io::{self, Write};
use std::process;
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
        
        indentificar_comando(input.trim());
    }
}

fn indentificar_comando(input: &str) {
	let mut fragmentos = input.split_whitespace();
	let comando = fragmentos.next().unwrap();
	
	match comando{
		"exit" | "quit" => exit(),
		"cd" => cd(),
		
		_ => erro(),
	}
}

fn exit() {
	println!("Saindo do Rush...");
	process::exit(0);
}

fn cd(){
	println!("CD...");
}

fn erro(){
	println!("Comando Inexistente...");
}
