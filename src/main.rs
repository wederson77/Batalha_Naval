use std::io;

const BOARD_SIZE: usize = 10;

struct Ship{
    x: usize,
    y: usize,
    lengh: usize,
    direction: Direction
}

enum Direction{
    Horizonal,
    Vertical,
}

fn main() {
    let mut board = [['_'; BOARD_SIZE]; BOARD_SIZE];
    let ships = vec![
        Ship{x: 2, y: 4, lengh: 2, direction: Direction::Horizonal},
        Ship{x: 4, y: 5, lengh: 3, direction: Direction::Vertical},
        Ship{x: 8, y: 8, lengh: 1, direction: Direction::Horizonal},
    ];

    for ship in ships.iter(){
        for i in 0..ship.lengh{
            let(x, y) = match ship.direction{
                Direction::Horizonal => (ship.x + i, ship.y),
                Direction::Vertical => (ship.x, ship.y + i),
                };
                board[x][y] = 'S';

            }
        }
    

    println!("Bem vindo ao jogo de Batalha Naval!");
    println!("Tente acertar todos os navios antes de acabarem seus tiros!");

    let mut shots = BOARD_SIZE * BOARD_SIZE;
    let mut ships_left = ships.len();
    while shots > 0 && ships_left > 0 {
        println!("Tabuleiro atual:");
        for row in board.iter(){
            for &cell in row.iter(){
                print!("{}", cell);
            }

            println!();
        }
        
        println!("\nVocê em {} tiros restanes", shots);
        println!("Digite as coordenadas (linha e coluna) para atirar:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess: Vec<usize> = guess
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let x = guess[0];
        let y = guess[1];

        if x >= BOARD_SIZE || y >= BOARD_SIZE{
            println!("Codernadas fora do tabuleiro, tente novamente");
            continue;
        }

        if board[x][y] == 'S'{
            println!("Você acertou um navio!");
            board[x][y] = 'X';
            ships_left -= 1;
        }else{
            println!("Você errou.");
            board[x][y] = 'O';
        }
            
        shots -= 1;   
    }

    if ships_left == 0{
        println!("Parabéns, você ganhou o jogo de Batalha Naval");

    }else{
        println!("Você perdeu o jogo de Batalha Naval, tente novamente!");
    }
}
