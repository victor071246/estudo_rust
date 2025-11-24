struct User{
    username: String,
    email: String,
    ativo: bool,
    genero: String,
}


fn main(){

    let mut pessoa = User {username: String::from("João Pessoa"), email: String::from("joaopessoa@gmail.com"), ativo: true, genero: String::from("Homem")};
    pessoa.ativo = false;

    println!("O nome do usuário é {}, seu email é {} e seu genêro é {}", pessoa.username, pessoa.email, pessoa.genero);

}