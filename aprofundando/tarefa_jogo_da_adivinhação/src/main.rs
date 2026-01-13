extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main(){
    println!("Adivinhe o número!");
    let secret_number = rand::thread_rng().gen_range(0, 101);

    loop{
        println!("Digite seu palpite:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Erro ao ler o palpite");
        let guessed_number: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        
        };

        println!("Você palpitou: {}", guess);

        match guessed_number.cmp(&secret_number){
            Ordering::Less => println!("Palpite baixo demais"),
            Ordering::Greater => println!("Palpite alto demais"),
            Ordering::Equal => {
                pritnln!("Você venceu o jogo!");
                break;
            }
        }
    }
}