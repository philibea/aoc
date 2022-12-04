use std::fmt;
use std::str::FromStr;

/*
A = Rock = X
B = Paper = Y
C = Scissor = Z

X = 1
Y = 2
Z = 3

Looser = 0
Equality = 3
Win = 6
*/
#[derive(Copy, Clone)]
pub enum Enemy {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for Enemy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Rock => "A",
                Self::Paper => "B",
                Self::Scissors => "C",
            }
        )
    }
}
impl FromStr for Enemy {
    type Err = ();

    fn from_str(s: &str) -> Result<Enemy, ()> {
        match s {
            "A" => Ok(Enemy::Rock),
            "B" => Ok(Enemy::Paper),
            "C" => Ok(Enemy::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Elf {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Rock => "X",
                Self::Paper => "Y",
                Self::Scissors => "Z",
            }
        )
    }
}

impl FromStr for Elf {
    type Err = ();

    fn from_str(s: &str) -> Result<Elf, ()> {
        match s {
            "X" => Ok(Elf::Rock),
            "Y" => Ok(Elf::Paper),
            "Z" => Ok(Elf::Scissors),
            _ => Err(()),
        }
    }
}

pub fn score_usage_elf(elf: Elf) -> u32 {
    match elf {
        Elf::Rock => 1,
        Elf::Paper => 2,
        Elf::Scissors => 3,
    }
}
pub fn score_usage(elf: Elf, enemy: Enemy) -> u32 {
    match elf {
        Elf::Rock => match enemy {
            Enemy::Rock => 3,
            Enemy::Paper => 0,
            Enemy::Scissors => 6,
        },
        Elf::Paper => match enemy {
            Enemy::Paper => 3,
            Enemy::Scissors => 0,
            Enemy::Rock => 6,
        },
        Elf::Scissors => match enemy {
            Enemy::Scissors => 3,
            Enemy::Rock => 0,
            Enemy::Paper => 6,
        },
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let res = line.split(" ").collect::<Vec<_>>();
        let enemy = Enemy::from_str(res[0]).unwrap();
        let elf = Elf::from_str(res[1]).unwrap();
        
        sum += score_usage(elf, enemy) + score_usage_elf(elf);
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::{score_usage, score_usage_elf, Elf, Enemy};

    #[test]
    fn score_usage_elf_test() {
        assert_eq!(score_usage_elf(Elf::Rock), 1);
        assert_eq!(score_usage_elf(Elf::Paper), 2);
        assert_eq!(score_usage_elf(Elf::Scissors), 3);
    }

    #[test]
    fn score_usage_test() {
        // Elf Rock
        assert_eq!(score_usage(Elf::Rock, Enemy::Paper), 0);
        assert_eq!(score_usage(Elf::Rock, Enemy::Scissors), 6);
        assert_eq!(score_usage(Elf::Rock, Enemy::Rock), 3);

        // Elf Scissors
        assert_eq!(score_usage(Elf::Scissors, Enemy::Paper), 6);
        assert_eq!(score_usage(Elf::Scissors, Enemy::Scissors), 3);
        assert_eq!(score_usage(Elf::Scissors, Enemy::Rock), 0);

        // Elf paper
        assert_eq!(score_usage(Elf::Paper, Enemy::Paper), 3);
        assert_eq!(score_usage(Elf::Paper, Enemy::Scissors), 0);
        assert_eq!(score_usage(Elf::Paper, Enemy::Rock), 6);
    }
}
