use std::io;

fn main(){
    let mut mensagem_usuario = String::new();
    println!("Digite algo: \n\n");

    match io::stdin().read_line(&mut mensagem_usuario.to_uppercase()){
        Ok(_) => println!("Sucesso, você digitou {}", mensagem_usuario),
        Err(e) => println!("Erro ao ler a mensagem do usuário: {}\n", e)
    }
}