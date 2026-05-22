use std::path::Path;
use std::io::{self, Write};
use std::process::{self,Command};
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
		"cd" => cd(fragmentos),
		
		_ => comando_externo(comando, fragmentos),
	}
}

fn exit() {
	println!("Saindo do Rush...");
	process::exit(0);
}

fn cd(mut argumentos: std::str::SplitWhitespace){
	let destino = argumentos.next();
	
	match destino {
		Some(pasta) => {
            // Converte a string pura para um formato de Caminho (Path) do SO
            let caminho = Path::new(pasta);
            
            // Tenta alterar o diretório do processo atual
            if let Err(erro) = env::set_current_dir(&caminho) {
                // Se der erro (ex: pasta não existe ou sem permissão), avisamos o usuário
                println!("Rush: cd: {}: {}", pasta, erro);
            }
        }
		None => {
			println!("Rush: erro: faltou o diretório de destino");
		}
	}
}

fn comando_externo(comando: &str, argumentos: std::str::SplitWhitespace){
	let executar = Command::new(comando)
		.args(argumentos)
		.status();
		
	match executar {
		Ok(status) => {
			if !status.success() {
                println!("Comando falhou com código: {}", status.code().unwrap_or(1));
            }
		}
		Err(_) => {
			println!("Rush: comando não encontrado: {}", comando);
		}
	}
}
