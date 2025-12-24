use std::collections::HashMap;

fn can_construct(ransom_note: &str, magazine: &str) -> bool{

    let mut magazine_chars = HashMap::new();

    //Contagem de caracteres na Magazine
    for char in magazine.chars(){
        *magazine_chars.entry(char).or_insert(0) += 1;
    }

    //Verificar caracteres em ransom_note
    for char in ransom_note.chars(){
        if let Some(count) = magazine_chars.get_mut(&char) {
            if *count == 0 {
                return  false;
            }
            *count -= 1;
        }
        else {
            return false
        }
    }

    true
}


fn main(){

    //Exemplos de teste
    let ransom_note1 = "a";
    let magazine1 = "b";
    println!("{}", can_construct(&ransom_note1, &magazine1)); //saida: false

    let ransom_note2 = "aa";
    let magazine2 = "ab";
    println!("{}", can_construct(&ransom_note1, &magazine1)); //saida: false

    let ransom_note1 = "aa";
    let magazine1 = "aab";
    println!("{}", can_construct(&ransom_note1, &magazine1)); //saida: true
}