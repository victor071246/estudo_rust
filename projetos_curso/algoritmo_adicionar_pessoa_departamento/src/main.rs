use std::{collections::HashMap, io};

fn departamento(){
    //let mut departamento_pessoas = HashMap::new();

    loop{
        println!("Digite o comando do tipo: 'Adicionar <Pessoa> para <Departamento>");
        let mut comando = String::new();
        io::stdin().read_line(&mut comando).expect("Erro ao ler variável comando");

        let comando = comando.trim();
        let mut token_comando = comando.split_whitespace();
        println!("{:?}", token_comando.nth(1));

        let pessoa = match token_comando.nth(1){
            Some(p) => p,
            None => {
                println!("O comando digitado não é valido");
                continue;
            }
        };
    }
}

fn main() {
    departamento();
}
