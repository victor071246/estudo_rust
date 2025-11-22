fn compress_string(s: &str) -> String {
    let mut compressed = String::new();
    let mut count = 0;

    // Percorre a string para realizar a compressão
    for (i, c) in s.chars().enumerate(){
        count += 1;

        // Verificar se o próximo caractere é diferente ou se chegamos ao final da string
        if i + 1 >= s.len() || c != s.chars().nth(i+1).unwrap(){
            compressed.push(c);
            compressed.push_str(&count.to_string());
            count = 0; // Reinicia a contagem para o próximo caractere
        }
    }

    if compressed.len() >= s.len() {
        s.to_string()
    }
    else {
        compressed
    }
}

fn main(){
    //Teste de exemplo
    let original_str = "aabccccaaa";
    let compressed_str = compress_string(original_str);
    println!("Original: {}", original_str);
    println!("Compressed: {}", compressed_str);

    let other_str = "abcdefgh";
    let compressed_other = compress_string(other_str);
    println!("Original: {}", other_str);
    println!("Compressed: {}", compressed_other);
}