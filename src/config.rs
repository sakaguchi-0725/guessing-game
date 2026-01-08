#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

pub struct GameConfig {
    pub min: u32,
    pub max: u32,
    pub name: String,
}

impl GameConfig {
    pub fn new(difficulty: Difficulty) -> Self {
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

    pub fn is_valid_range(&self, guess: u32) -> bool {
        (self.min..=self.max).contains(&guess)
    }
}

#[cfg(test)]
mod tests {
    use super::{Difficulty, GameConfig};

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
