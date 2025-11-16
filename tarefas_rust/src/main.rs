use std::io;

fn convert_to_int(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn count(){

    let mut entrada_num = String::new();

    println!("Insira um número para ser lido: ");
    io::stdin().read_line(&mut entrada_num).expect("Erro ao inserir o número");

    let num = convert_to_int(&entrada_num);

    println!("O número inserido foi: {}", num)
}

fn main(){
    count();
}