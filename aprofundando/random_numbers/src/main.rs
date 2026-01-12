extern crate rand;
use rand::Rng;


fn main() {

    let valores_randomicos = rand::thread_rng().gen_range(5., 11.);
    println!("{}", valores_randomicos);

    let valores_randomicos_bool = rand::thread_rng().gen_weighted_bool(5);
    println!("{}", valores_randomicos_bool)

}
