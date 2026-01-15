mod metodo;

fn main() {
    let usuario = metodo::Pessoa{nome: String::from("João"), sobrenome: String::from("Silva")};
    usuario.qual_nome();
    usuario.nome_completo();
}
