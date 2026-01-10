use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut arquivo = File::open("rust_wiki.txt").expect("Não foi possível ler o arquivo");
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo).expect("Não foi possível ler o arquivo e alocar na variável conteúdo");
    println!("O conteúdo em arquivo é:\n {}", conteudo);

}