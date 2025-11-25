fn reverse_string(input: &str) -> String {

    let reversed: String = input.chars().rev().collect();
    reversed
}

fn main() {
    let input_string1 = String::from("hello");
    let input_string2 = String::from("rust");

    let reversed_string1 = reverse_string(&input_string1);
    let reversed_string2 = reverse_string(&input_string2);

    println!("Original: {}", input_string1);
    println!("Reversed: {}", reversed_string1);

    println!();
    println!("Original: {}", input_string2);
    println!("Reversed: {}", reversed_string2);
}