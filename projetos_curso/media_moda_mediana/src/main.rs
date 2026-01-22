use std::collections::HashMap;

fn media(numeros: &Vec<i32>) -> f64 {

    let mut soma = 0;
    for i in numeros{

        soma += i;
    }

    soma as f64 / numeros.len() as f64
}

fn mediana(numeros: &Vec<i32>) -> f64 {
    let mut numeros_sorted = numeros.clone();
    numeros_sorted.sort();
    println!("O vetor está em ordem crescente {:?}", numeros_sorted);

    let numero_meio = numeros.len() / 2;
    if numeros_sorted.len() % 2 == 0{
        return media(&vec![numeros_sorted[numero_meio], numeros_sorted[numero_meio-1]]);
    }

    numeros_sorted[numero_meio] as f64
}

fn moda(numeros: Vec<i32>) -> i32 {

    let mut hash_map = HashMap::new();
    for i in numeros{
        let contar = map.entry(i).or_insert(0);
        *contar += 1;

        pritnln!("{}" map)
    }

    0

}




fn main() {
    let numeros = vec![0, 4, 6, 3, 4, 1, 4, 5, 2, 1];
    println!("{}", media(&numeros));

    println!("{}", mediana(&numeros))
}
