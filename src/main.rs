use std::{cmp::Ordering, fmt, io};

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

fn ask_retry() -> Result<bool, GameError> {
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

#[derive(Debug)]
enum GameError {
    IoError(io::Error),
    ParseError,
    OutOfRange { min: u32, max: u32 },
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::IoError(e) => write!(f, "入力エラーが発生しました: {}", e),
            GameError::ParseError => write!(f, "数字を入力してください"),
            GameError::OutOfRange { min, max } => {
                write!(f, "{}から{}の間で入力してください", min, max)
            }
        }
    }
}

fn read_input() -> Result<String, GameError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(GameError::IoError)?;
    Ok(input.trim().to_string())
}

fn get_guess(config: &GameConfig) -> Result<u32, GameError> {
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

fn select_difficulty() -> Result<Difficulty, GameError> {
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
