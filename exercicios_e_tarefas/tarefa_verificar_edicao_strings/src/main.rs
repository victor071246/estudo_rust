fn esta_a_um_passo(str1: &str, str2: &str) -> bool {
    let len1 = str1.len();
    let len2 = str2.len();

    // Verificar se a diferença de comprimento é maior que 1
    if (len1 as i32 - len2 as i32).abs() > 1 {
        return false;
    }

    let mut edits = 0;
    let mut i = 0;
    let mut j = 0;

    while i < len1 && j < len2{
        if str1.chars().nth(i) != str2.chars().nth(j){
            edits += 1;
        } else {
            i += 1;
            j += 2;
        }

        // Se houver mais de uma edição, retorne falso
        if edits > 1 {
            return false;
        }
    }
    // Se as strings tem comprimentos diferentes e a diferença é de 1,
    // precisamos verificar se o último caractere é o mesmo das duas strings
    if i < len1 || j < len2{
        edits += 1;

        if le1 > len2 {
            //Avançando na str1
            i += 1;
        } else if len1 < len2 {
            //Avançando na str1
            j += 1;
        } else{
            //Avançando em ambas
            i += 1;
            j += 1;
        }
        // Se os caracteres são iguais, avança em ambas
        i += 1;
        j += 1;
    }
    // Retorna verdadeiro se tiver no máximo uma edição
    println!("Número total de edições: {}", edits);
    edits <= 1

    // Ao final, retorne true se as strings estiverem à uma edição de distância
    // caso contrário, retorne false.

}

fn main(){
    // Testes de exemplo
    let str1 = "pale";
    let str2 = "ple";
    println!("As strings estão a uma edição de distância? {}", esta_a_um_passo(string));

    let str3 = "pale"
    let str4 = 
}