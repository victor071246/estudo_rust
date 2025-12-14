use std::collections::{HashMap, hash_map};

fn main() {

    let mut hash_map = HashMap::new();
    hash_map.insert("Matemática", 90);
    hash_map.insert("Português", 72);
    hash_map.insert("Biologia", 58);
    hash_map.insert("Informática", 96);

    println!("Quantas materias o aluno cursou? {}", hash_map.len());

    match hash_map.get("Informatica"){
        Some(k) => println!("O aluno cursou Informática e tirou {}", k),
        None => println!("O aluno não cursou Informática")
    }

    hash_map.remove("Português");
    println!(" O aluno estuda Português? {}", hash_map.contains_key("Português"));

}
