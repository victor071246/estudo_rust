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
        
    for i in 1..(num+1){
        println!("Exibindo {}", i);
    }

}

fn for1(){


}

fn count_down(){

    let mut entrada_num = String::new();

    println!("Insira um número para ser lido: ");
    io::stdin().read_line(&mut entrada_num).expect("Erro ao inserir o número");

    let num = convert_to_int(&entrada_num);

    for i in (1..entrada_num+1).rev(){
        println!("Exibindo {}", i);
    }
}

fn main(){
    // count();
    // for1();
    count_down();
}