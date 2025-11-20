fn eh_permutacao(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len(){
        return false; // Diferentes comprimentos não podem ser permutações
    }

    let mut contagem_caracteres = [0; 128]; // Assumindo caracteres ASCII

    // Conta as ocorrências de caracteres da primeira string
    for &c in str1.as_bytes(){
        contagem_caracteres[c as usize] += 1;
    }

    // Decrementa as ocorrências de caracteres com base na segunda string
    for &c in str2.as_bytes(){
        contagem_caracteres[c as usize] -= 1;
        if contagem_caracteres[c as usize] < 0{
            return false; //Mais ocorrências do caractere na segunda string
        }
    }

    true // Todos os caracteres da primeira string tem ocorrências correspondentes na segunda string
}

fn main() {

    let str1 = "abc";
    let str2 = "baa";

    if eh_permutacao(str1, str2){
        println!("As strings são permutações uma da outra");
    } else {
        println!("As strings não são permutações uma da outra");
    }

}
