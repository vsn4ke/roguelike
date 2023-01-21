use bracket_lib::random::{DiceType, RandomNumberGenerator};
use regex::Regex;

pub fn parse_dice_string(dice: &str) -> DiceType {
    let dice = &dice.split_whitespace().collect::<Vec<_>>().join("");
    let re = Regex::new(r"([0-9]+)d([0-9]+)([\+\-][0-9]+)?").unwrap();

    let mut result: DiceType = DiceType::default();
    for cap in re.captures_iter(dice) {
        if let Some(group) = cap.get(1) {
            result.n_dice = group.as_str().parse::<i32>().unwrap();
        }
        if let Some(group) = cap.get(2) {
            result.die_type = group.as_str().parse::<i32>().unwrap();
        }
        if let Some(group) = cap.get(3) {
            result.bonus = group.as_str().parse::<i32>().unwrap();
        }
    }
    result
}

pub fn roll_dice(n: i32, die_type: i32) -> i32 {
    let mut rng = RandomNumberGenerator::new();
    (0..n).map(|_| rng.range(1, die_type + 1)).sum()
}

pub fn roll(dice: &str) -> i32 {
    let d = parse_dice_string(dice);
    roll_dice(d.n_dice, d.die_type) + d.bonus
}
