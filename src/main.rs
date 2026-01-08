mod config;
mod error;
mod input;

use config::GameConfig;
use error::GameError;
use input::{ask_retry, get_guess, select_difficulty};
use std::cmp::Ordering;

fn main() {
    println!("数を当ててください！");

    loop {
        // 難易度選択
        let difficulty = match select_difficulty() {
            Ok(d) => d,
            Err(GameError::IoError(e)) => {
                eprintln!("{}", e);
                continue;
            }
            _ => continue,
        };

        let config = GameConfig::new(difficulty);

        println!("{}モードでゲームを開始します！", config.name);

        // 乱数生成
        let secret_number = rand::random_range(config.min..=config.max);
        let mut attempts = 0;

        // ゲームループ
        loop {
            match get_guess(&config) {
                Ok(guess) => {
                    attempts += 1;

                    match guess.cmp(&secret_number) {
                        Ordering::Less => println!("もっと大きい数です"),
                        Ordering::Greater => println!("もっと小さい数です"),
                        Ordering::Equal => {
                            println!("正解！{}回で当たりました", attempts);
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            }
        }

        // リトライ確認
        match ask_retry() {
            Ok(true) => continue,
            Ok(false) => {
                println!("ゲームを終了します");
                break;
            }
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        };
    }
}
