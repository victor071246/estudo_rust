fn eh_palindromo(x: i32) -> bool{
    // Casos especiais: Números negativos e números terminados em zero
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut original = x;
    let mut invertido = 0;

    //524
    // digito = 4, invertido = 4, original = 52
    // digito = 2, invertido = 42, original = 5
    // digito = 5, invertido = 425, original = 0 || nulo

    //524 == 425 ?

    while original != 0{

        let digito = original % 10;
        invertido = invertido * 10 + digito;
        original /= 10;

    }

    return x == invertido;

    // x % 2 , x = 7 , 7 % 2 => 7 / 2 = 1
}

fn main() {

    let num1 = 9;
    let num2 = -121;
    let num3 = 10;

    println!("É {} um palíndromo? {}", num1, eh_palindromo(num1));
    println!("É {} um palíndromo? {}", num2, eh_palindromo(num2));
    println!("É {} um palíndromo? {}", num3, eh_palindromo(num3));

}