use std::io;

const PRIMES: [u32; 9] = [23, 29, 31, 37, 41, 43, 47, 53, 59];

fn main() {
    let score: u32 = take_score_inp();
    println!("Score: {}", score);

    let mut player_1_score = score;
    let mut player_2_score = score;

    let mut player_i = 1;

    while player_1_score > 0 && player_2_score > 0 {
        if player_i == 1 {
            player_1_score = take_turn(player_i, player_1_score);
            player_i = 2;
        } else {
            player_2_score = take_turn(player_i, player_2_score);
            player_i = 1;
        }
    }

    if player_1_score == 0 {
        println!("Player 1 wins!");
    } else {
        println!("Player 2 wins!");
    }
}

fn take_score_inp() -> u32 {
    let mut score = 0;
    while score == 0 {
        let mut score_str = String::new();
        match io::stdin().read_line(&mut score_str) {
            Ok(_) => {}
            Err(_) => {
                println!("Invalid input. Please try again");
                continue;
            }
        }
        let score_potential: u32 = match score_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please try again");
                continue;
            }
        };
        if score_potential != 301 && score_potential != 501 {
            println!("Please enter a valid score");
            continue;
        }
        score = score_potential;
    }
    return score;
}

fn int_inp(prompt: String) -> u32 {
    println!("{}", prompt);
    let mut inp = String::new();
    match io::stdin().read_line(&mut inp) {
        Ok(_) => {}
        Err(_) => {
            println!("Invalid input. Please try again");
            return int_inp(prompt);
        }
    };
    let inp: u32 = match inp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please try again");
            return int_inp(prompt);
        }
    };
    return inp;
}

fn take_turn(player_i: u32, player_score: u32) -> u32 {
    let turn_score = int_inp(format!("Player {} score this round: ", player_i));

    if turn_score > 60 || PRIMES.contains(&turn_score) {
        println!(
            "Player {} score this round: {} is invalid",
            player_i, turn_score
        );
        return player_score;
    }

    if (turn_score as u32) > player_score {
        println!(
            "Turn exceeded player score. Player {} stays on {}",
            player_i, player_score
        );
        return player_score;
    }
    let new_score = player_score - (turn_score as u32);
    println!(
        "Player {} score: {} -> {}",
        player_i, player_score, new_score
    );
    return new_score;
}
