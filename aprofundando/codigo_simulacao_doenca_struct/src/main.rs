mod doenca;

use doenca::Doenca;
fn main(){
    let gripe = Doenca::new(String::from("Gripe"), vec![String::from("Tosse"), String::from("Espirro"), String::from("Febre")], String::from("Vírus da gripe"), String::from("Repouse e medicamentos") );

    println!("{}", gripe);
}