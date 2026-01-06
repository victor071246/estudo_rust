fn conta_maiusculas(texto: &str) -> u32 {

    let mut contador = 0;
    for caractere in texto.chars(){
        if caractere.is_uppercase() {
            contador += 1;
        }
    }

    contador
}

fn main(){

    let texto = "Este é um Texto Exemplo";
    // &str
    let contador = conta_maiusculas(texto);
    println!("Número de letras maiúsculas : {}", contador);
}