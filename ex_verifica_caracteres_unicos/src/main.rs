fn tem_caracteres_unicos(input: &str) -> bool{
    let mut conjunto_de_caracteres = [false; 128];

    for &c in input.as_bytes(){
        let indice = c as usize;
        // println!("Caractere: {}, indice: {}", c as char, indice);

        if conjunto_de_caracteres[indice]{
            // println!("Caractere duplicado encontrado!");
            return false; // caractere já foi encontrado
        }
        conjunto_de_caracteres[indice] = true;
    }
    true // Todos os caracteres são únicos


}

fn main() {
    let string_de_teste = "abcdef";
    if tem_caracteres_unicos(string_de_teste){
        println!("A string possui todos os caracteres únicos");
    } else{
        println!("A string não possui todos os caracteres únicos")
    }
}
