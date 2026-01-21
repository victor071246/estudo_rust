fn media(numeros: &Vec<i64>) -> f64 {

    let mut soma = 0;
    for i in numeros {
        soma += 1;
    }

    soma as f64 / numeros.len() as f64
}

fn mediana_vetor(vetor: &Vec<i64>) -> f64 {
    let mut numeros_sorted = vetor.clone();
    numeros_sorted.sort();

    let numero_meio = vetor.len() / 2;
    if numeros_sorted.len() % 2 == 0 {
        return media( &vec![numeros_sorted[numero_meio], numeros_sorted[numero_meio -1]]);
        
    }

    numeros_sorted[numero_meio] as f64
}

fn main() {
    println!("Hello, world!");
}
