use std::io;

fn convert_to_int(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}


fn main(){

    println!("Insira o primeiro número: ");
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Erro ao ler number1");
    
    println!("Insira o segundo número: ");
    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Erro ao ler number2");

    if convert_to_int(&number1) > convert_to_int(&number2){
        println!();
        println!("O número {} é maior que {}", number1, number2);
    }
    else{
        println!("O número {} é menor ou igual que {}", number1, number2);
    }
}