use std::{cmp::Ordering, io};

fn main() {
    println!("数を当ててください！");

    // 乱数生成
    let secret_number = rand::random_range(1..=100);
    let mut attempts = 0;

    // ゲームループ
    loop {
        match get_guess() {
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
                println!("1から100の間で入力してください");
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

fn get_guess() -> Result<u32, GuessError> {
    println!("予想を入力してください");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("入力の読み込みに失敗しました");

    validate_guess(&input)
}

fn validate_guess(input: &str) -> Result<u32, GuessError> {
    let guess: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err(GuessError::ParseError),
    };

    if !(1..=100).contains(&guess) {
        return Err(GuessError::OutOfRange);
    }

    Ok(guess)
}

#[cfg(test)]
mod tests {
    use crate::{GuessError, validate_guess};

    #[test]
    fn test_valid_input() {
        assert_eq!(validate_guess("50"), Ok(50));
        assert_eq!(validate_guess("1"), Ok(1));
        assert_eq!(validate_guess("100"), Ok(100));
    }

    #[test]
    fn test_out_of_range() {
        assert_eq!(validate_guess("0"), Err(GuessError::OutOfRange));
        assert_eq!(validate_guess("101"), Err(GuessError::OutOfRange));
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(validate_guess("abc"), Err(GuessError::ParseError));
        assert_eq!(validate_guess(""), Err(GuessError::ParseError));
        assert_eq!(validate_guess("12.5"), Err(GuessError::ParseError));
        assert_eq!(validate_guess("１０"), Err(GuessError::ParseError));
    }
}
