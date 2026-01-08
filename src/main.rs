use std::{cmp::Ordering, io};

fn main() {
    println!("数を当ててください！");

    let difficulty = select_difficulty();
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
            Err(GuessError::ParseError) => {
                println!("数字を入力してください");
                continue;
            }
            Err(GuessError::OutOfRange) => {
                println!("{}から{}の間で入力してください", config.min, config.max);
                continue;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum GuessError {
    ParseError,
    OutOfRange,
}

fn get_guess(config: &GameConfig) -> Result<u32, GuessError> {
    println!("予想を入力してください");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("入力の読み込みに失敗しました");

    let guess: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err(GuessError::ParseError),
    };

    if !config.is_valid_range(guess) {
        return Err(GuessError::OutOfRange);
    }

    Ok(guess)
}

#[derive(Debug, Clone, Copy)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}

struct GameConfig {
    min: u32,
    max: u32,
    name: String,
}

impl GameConfig {
    fn new(difficulty: Difficulty) -> Self {
        match difficulty {
            Difficulty::Easy => GameConfig {
                min: 1,
                max: 50,
                name: String::from("初級"),
            },
            Difficulty::Normal => GameConfig {
                min: 1,
                max: 100,
                name: String::from("中級"),
            },
            Difficulty::Hard => GameConfig {
                min: 1,
                max: 200,
                name: String::from("上級"),
            },
        }
    }

    fn is_valid_range(&self, guess: u32) -> bool {
        (self.min..=self.max).contains(&guess)
    }
}

fn select_difficulty() -> Difficulty {
    println!("難易度を選択してください（1, 2, 3）");
    println!("1: 初級（1-50）");
    println!("2: 中級（1-100）");
    println!("3: 上級（1-200）");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("難易度の読み取りに失敗しました");

        match input.trim() {
            "1" => return Difficulty::Easy,
            "2" => return Difficulty::Normal,
            "3" => return Difficulty::Hard,
            _ => {
                println!("入力された値が正しくありません");
                println!("1 ~ 3で難易度を選択してください");
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Difficulty, GameConfig};

    #[test]
    fn test_easy_valid_range() {
        let config = GameConfig::new(Difficulty::Easy);
        assert!(!config.is_valid_range(0));
        assert!(config.is_valid_range(1));
        assert!(config.is_valid_range(50));
        assert!(!config.is_valid_range(51));
    }

    #[test]
    fn test_normal_valid_range() {
        let config = GameConfig::new(Difficulty::Normal);
        assert!(!config.is_valid_range(0));
        assert!(config.is_valid_range(1));
        assert!(config.is_valid_range(100));
        assert!(!config.is_valid_range(101));
    }

    #[test]
    fn test_hard_valid_range() {
        let config = GameConfig::new(Difficulty::Hard);
        assert!(!config.is_valid_range(0));
        assert!(config.is_valid_range(1));
        assert!(config.is_valid_range(200));
        assert!(!config.is_valid_range(201));
    }
}
