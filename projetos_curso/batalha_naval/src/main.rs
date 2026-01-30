use std::io;
use rand::Rng;

const BOARD_SIZE: usize = 10;

struct Ship{
    x: usize,
    y: usize,
    length: usize,
    direction: Direction
}

enum Direction{
    Horizontal,
    Vertical
}

fn main() {
    let mut board = [['_'; BOARD_SIZE]; BOARD_SIZE];
    let ships = vec![
        Ship{x: 2, y: 4, length: 2, direction: Direction::Horizontal},
        Ship{x: 4, y: 5, length: 3, direction: Direction::Vertical},
        Ship{x: 8, y: 8, length: 1, direction: Direction::Horizontal}
    ];

    for ship in ships.iter(){
        for i in 0..ship.length{
            let(x, y) = match ship.direction{
                Direction::Horizontal => (ship.x + i, ship.y),
                Direction::Vertical => (ship.x, ship.y + i)
            };
            board[x][y] = 'S';
        }
    }

    println!("Bem-Vindo ao Jogo de Batalha Naval");
    println!("Tente acertar todos os navios antes de acabarem seus tiros!");

    let mut shots = BOARD_SIZE*BOARD_SIZE;
    let mut ships_left = ships.len();
    while shots > 0 && ships_left > 0{
        println!("Tabuleiro atual: ");
        for row in board.iter(){
            for &cell in row.iter(){
                print!("{}", cell)
            }
            println!();
        }
        println!("Você tem {} disparos restantes", shots);
        println!("Digite as coordenadas (linha: X, coluna: Y) para atirar");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess: Vec<usize> = guess.trim().split_whitespace().map(|substring| substring.parse().unwrap()).collect();
        let x = guess[0];
        let y = guess[1];

        if x >= BOARD_SIZE || y >= BOARD_SIZE{
            println!("Coordenadas fora do tabuleiro, tente novamente");
            continue;
        }

        if board[x][y] == 'S'{
            println!("Você acertou um navio");
            board[x][y] = 'X';
            ships_left -= 1;
        } else {
            println!("Você errou");
            board[x][y] = 'O';
        }

        shots -= 1;
    }
    
    if ships_left == 0{
        println!("Parabéns, você ganhou o jogo da batalha naval");
    }
    else {
        println!("Você perdeu o jogo da batalha naval, tente novamente");
    }
    
}
