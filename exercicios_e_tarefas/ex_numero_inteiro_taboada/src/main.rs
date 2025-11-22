fn main() {

    println!("Digite um número inteiro: ");

    let mut num = String::new();

    //Lê o número do usuário

    std::io::stdin().read_line(&mut num).expect("Falha ao ler a entrada");

    let num: i32 = num.trim().parse().expect("Insira um valor número inteiro");

    for i in 1..11{
        println!("{} x {} = {}", num, i, num * i);
    }

}