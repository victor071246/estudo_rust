fn main(){

    let numero = 2;

    match numero{
        1 => println!("O número é 1"),
        2..10 => println!("O número está entre 2 e 10"),
        _ => println!("Numero não é 1, nem 2, nem 3")
    }

    let nome = "Rodrigo";

    match nome {
        "João" => println!("João é dentista"),
        "Rodrigo" => println!("Rodrigo é estudante"),
        _ => println!("Não sei a profissão")
    }

}