// enum Direction{
//     Up,
//     Down,
//     Left,
//     Rigth
// }

// fn main() {

//     let player: Direction = Direction::Rigth;
//     match player{
//         Direction::Up => println!("O jogador foi para cima"),
//         Direction::Down => println!("O jogador foi para baixo"),
//         Direction::Rigth => println!("O jogador foi para a direita"),
//         Direction::Left => println!("O jogador foi para a esquerda"),
//     }
// }

#[derive(Debug)]
enum Gender {
    Male, Female
}

fn main() {
    
    let player_male:Gender = Gender::Male;
    let player_female:Gender = Gender::Female;

    println!("{}", player_male);


}