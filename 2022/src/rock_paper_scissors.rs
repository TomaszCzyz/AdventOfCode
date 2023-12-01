use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

/// | opp \ me | Rock | Paper | Scissors |
/// | Rock     |   0  |   1   |   -1     |
/// | Paper    |  -1  |   0   |    1     |
/// | Scissors |   1  |  -1   |    0     |
pub const BATTLE_RESULT: [[i32; 3]; 3] = [
    [0, 1, -1],
    [-1, 0, 1],
    [1, -1, 0],
];

#[derive(Copy, Clone, Debug)]
enum AttackType {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl From<i32> for AttackType {
    fn from(value: i32) -> Self {
        match value {
            0 => AttackType::Rock,
            1 => AttackType::Paper,
            2 => AttackType::Scissors,
            _ => panic!()
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum BattleResult {
    Win = 1,
    Draw = 0,
    Lose = -1,
}

impl From<i32> for BattleResult {
    fn from(value: i32) -> Self {
        match value {
            -1 => BattleResult::Lose,
            0 => BattleResult::Draw,
            1 => BattleResult::Win,
            _ => panic!()
        }
    }
}

fn opp_mapping(letter: char) -> AttackType {
    match letter {
        'A' => AttackType::Rock,
        'B' => AttackType::Paper,
        'C' => AttackType::Scissors,
        _ => panic!()
    }
}

fn me_mapping(letter: char) -> AttackType {
    match letter {
        'X' => AttackType::Rock,
        'Y' => AttackType::Paper,
        'Z' => AttackType::Scissors,
        _ => panic!()
    }
}

fn calculate_point_for_round(result: BattleResult, my_attack: AttackType) -> i32 {
    let battle_points = match result {
        BattleResult::Win => 6,
        BattleResult::Draw => 3,
        BattleResult::Lose => 0,
    };

    let attack_points = match my_attack {
        AttackType::Rock => 1,
        AttackType::Paper => 2,
        AttackType::Scissors => 3,
    };

    battle_points + attack_points
}

pub fn rock_paper_scissors_part_1(file_name: &str) -> Result<(), io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for result in reader.lines() {
        let line = result.unwrap();
        // let line = match result {
        //     Ok(line) => line,
        //     Err(_) => break
        // };

        let mut iter = line.chars();

        let opponent = opp_mapping(iter.next().unwrap());
        let me = me_mapping(iter.skip(1).next().unwrap());

        let battle_result = BattleResult::from(BATTLE_RESULT[opponent as usize][me as usize]);

        sum += calculate_point_for_round(battle_result, me);
        println!("opponent: {:?} \t\t me: {:?} \t\t\t result: {:?}", opponent, me, battle_result);
    }

    println!("total points: {}", sum);
    Ok(())
}

fn expectation_mapping(letter: char) -> BattleResult {
    match letter {
        'X' => BattleResult::Lose,
        'Y' => BattleResult::Draw,
        'Z' => BattleResult::Win,
        _ => panic!()
    }
}

pub fn rock_paper_scissors_part_2(file_name: &str) -> Result<(), io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for result in reader.lines() {
        let line = result.unwrap();
        let mut iter = line.chars();

        let opponent = opp_mapping(iter.next().unwrap());
        let expected_result = expectation_mapping(iter.skip(1).next().unwrap());

        let my_attack: AttackType = BATTLE_RESULT[opponent as usize]
            .iter()
            .position(|&x| x == (expected_result as i32))
            .map(|x| AttackType::from(x as i32))
            .unwrap();

        sum += calculate_point_for_round(expected_result, my_attack);
        println!("opponent: {:?} \t\t me: {:?} \t\t\t result: {:?}", opponent, my_attack, expected_result);
    }

    println!("total points: {}", sum);
    Ok(())
}
