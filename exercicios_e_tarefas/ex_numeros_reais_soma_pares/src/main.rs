// Leia uma sequência de números reais e exiba a soma dos números pares

fn main() {
    println!("Digite uma sequência de números reais: ");

    let mut input = String::new();

    //Leia a sequência de núemros do usuário

    std::io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");

    let mut numbers: Vec<f64> = input.trim().split_whitespace().map( |x| x.parse::<f64>().expect("Erro, insira apenas números reais!!!")).collect();

    let mut sum: f64 = 0.0;

    for num in &numbers{

        if num % 2.0 == 0.0{
            sum += num;
        }
    }

    println!("A soma dos números pares é: {}", sum);

}
