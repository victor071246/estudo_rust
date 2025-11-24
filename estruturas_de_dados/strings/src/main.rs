
fn main() {

    let mut minhaString: String = String::from("Olá meu nome é Victor");
    println!("O tamanho dessa string é {} caracteres incluindo o espaço", minhaString.len());
    println!("A minha string está vazia? {}", minhaString.is_empty());
    for token in minhaString.split_whitespace(){
        println!("imprimindo '{}' de '{}'", token, minhaString);
    }
    println!("O nome Henrique está contido na String? {}", minhaString.contains("Henrique"));
    minhaString.push_str(", bem vindo à aula.");
    println!("{}", minhaString);

}