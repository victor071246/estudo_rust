use std::collections::HashMap;

fn moda(numeros: &[i32]) -> Option<i32>{
    let mut map = HashMap::new();

    //Contar as ocorrências de cada número
    for &i in numeros{
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    //Encontrar a chave com o maior valor (moda)
    let mut chave_moda = None;
    let mut maior_contagem = 0;

    for (chave, &valor) in &map{
        if valor > maior_contagem{
            maior_contagem = valor;
            chave_moda = Some(*chave);
        }
    }

    // Verificar se há empates (se múltiplos números tem a mesma contagem mais alta)
    let mut chaves_empate = vec![];
    for (chave, &valor) in &map{
        if valor == maior_contagem {
            chaves_empate.push(*chave);
        }
    }

    // Lidar com o caso em que há um empate (retornar None)
    if chaves_empate.len() > 1{
        None
    } else {
        chave_moda
    }
}

fn main() {
    let numeros = vec![1, 1, 2, 3, 4, 4, 4, 5, 6, 0];
    match moda(&numeros) {
        Some(moda) => println!("A moda é: {}", moda),
        None => println!("Existe múltiplas modas")
    }
}
