use crate::config::Difficulty;
use crate::config::GameConfig;
use crate::error::GameError;
use std::io;

fn read_input() -> Result<String, GameError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(GameError::IoError)?;
    Ok(input.trim().to_string())
}

pub fn get_guess(config: &GameConfig) -> Result<u32, GameError> {
    println!("予想を入力してください");

    let input = read_input()?;

    let guess: u32 = match input.parse() {
        Ok(num) => num,
        Err(_) => return Err(GameError::ParseError),
    };

    if !config.is_valid_range(guess) {
        return Err(GameError::OutOfRange {
            min: config.min,
            max: config.max,
        });
    }

    Ok(guess)
}

pub fn ask_retry() -> Result<bool, GameError> {
    println!("もう一度プレイしますか？（y/n）");

    loop {
        let input = read_input()?;

        match input.to_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => {
                println!("y/nで回答してください");
                continue;
            }
        }
    }
}

pub fn select_difficulty() -> Result<Difficulty, GameError> {
    println!("難易度を選択してください（1, 2, 3）");
    println!("1: 初級（1-50）");
    println!("2: 中級（1-100）");
    println!("3: 上級（1-200）");

    loop {
        let input = read_input()?;

        match input.as_str() {
            "1" => return Ok(Difficulty::Easy),
            "2" => return Ok(Difficulty::Normal),
            "3" => return Ok(Difficulty::Hard),
            _ => {
                println!("入力された値が正しくありません");
                println!("1 ~ 3で難易度を選択してください");
                continue;
            }
        }
    }
}
