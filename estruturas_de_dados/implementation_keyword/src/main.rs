struct User {
    username: String,
    email: String,
    ativo: bool,
    genero: String
}

impl User {
    fn nome_do_usuario(&self){
        println!("O nome do usuário é {}", self.username);
    }

    fn ativo_ou_inativo(&self){
        println!("O usuário está ativo? {}", self.ativo);
    }
}

fn main() {

    let pessoa = User{ username: String::from("joao123"), email: String::from("joao123@gmail"), ativo: false, genero: String::from("Homem")};
    pessoa.nome_do_usuario();
    pessoa.ativo_ou_inativo();
}