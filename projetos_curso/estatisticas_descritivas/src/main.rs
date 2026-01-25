use std::fs::File;
use std::io::{BufRead, BufReader};

use estatisticas_descritivas::executar_estatisticas_descritivas;

fn ler_numeros(nome_arquivo: &str) -> Result<Vec<i64>, std::io::Error>{
    let arquivo = File::open(nome_arquivo)?;
    let leitor = BufReader::new(arquivo);
    let mut numeros = vec![];

    for linha in leitor.lines(){
        let linha = linha?;
        if let Ok(linha) = linha.trim().parse::<i64>(){
            numeros.push(linha);
        }
    }

    Ok(numeros)
}
fn main() {
    let numeros: Vec<i64> = ler_numeros("dados.txt").unwrap_or_else(|err| {
        eprintln!("Erro ao ler o arquivo: {}", err);
        std::process::exit(1);
    });
    executar_estatisticas_descritivas(numeros);
}
