struct User{
    username: String,
    email: String,
    ativo: bool,
    genero: String,
}

fn user(usuario: &User){
    println!("O nome do usuário é {}", usuario.username);
}

fn main(){

    let mut pessoa = User {username:String::from("Victor"), email:String::from("v@gmail.com"), ativo:true,  genero:String::from("Homem")};
    pessoa.ativo = false;
    println!("O nome do usuário é {}, seu email é {} e seu genêro é {}", pessoa.username, pessoa.email, pessoa.genero);
    user(&pessoa);
    user(&pessoa);
    

}