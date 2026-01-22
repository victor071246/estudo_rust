fn media(numeros: &Vec<f64>) -> f64 {
    let sum: f64 = numeros.iter().sum();
    sum / numeros.len() as f64
}

fn mediana(vetor: &Vec<i64>) -> Result<f64, &'static str> {
    if vetor.is_empty(){
        return Err("O vetor está vazio");
    }

    let mut sorted_numeros = vetor.clone();
    sorted_numeros.sort();
    let len = sorted_numeros.len();
    if len % 2 == 1{
        Ok(sorted_numeros[len/2] as  f64)
    } else {
        let mid1 = sorted_numeros[len/2 - 1] as f64;
        let mid2 = sorted_numeros[len/2] as f64;
        Ok(media(&vec![mid1, mid2]))
    }
}

fn main() {
    let numeros_impares = vec![3, 1, 2]; //mediana: 2
    let numeros_pares = vec![4, 1, 3, 2]; // mediana: 2.5
    let numeros_vazio:Vec<i64> = vec![];

    match mediana(&numeros_impares){
        Ok(mediana) => println!("A mediana dos números ímpares é: {}", mediana),
        Err(e) => println!("Erro: {}", e)
    }

    match mediana(&numeros_pares){
        Ok(mediana) => println!("A mediana dos números ímpares é: {}", mediana),
        Err(e) => println!("Erro: {}", e)
    }

    match mediana(&numeros_vazio){
        Ok(mediana) => println!("A mediana dos números ímpares é: {}", mediana),
        Err(e) => println!("Erro: {}", e)
    }


}
