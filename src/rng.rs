use rand::{Rng, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use regex::Regex;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct DiceType {
    pub n_dice: i32,
    pub die_type: i32,
    pub bonus: i32,
}

impl DiceType {
    pub fn new(n_dice: i32, die_type: i32, bonus: i32) -> Self {
        DiceType {
            n_dice,
            die_type,
            bonus,
        }
    }
}

impl Default for DiceType {
    fn default() -> DiceType {
        DiceType {
            n_dice: 1,
            die_type: 4,
            bonus: 0,
        }
    }
}

pub struct RandomGen {
    rng: XorShiftRng,
}

impl RandomGen {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: SeedableRng::seed_from_u64(seed),
        }
    }

    pub fn range<T>(&mut self, min: T, max: T) -> T
    where
        T: rand::distributions::uniform::SampleUniform + PartialOrd,
    {
        self.rng.gen_range(min..max)
    }

    pub fn roll_dice(&mut self, n: i32, die_type: i32) -> i32 {
        (0..n).map(|_| self.range(1, die_type + 1)).sum()
    }

    pub fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    pub fn roll_str<S: ToString>(&mut self, dice: S) -> i32 {
        let d = parse_dice_string(&dice.to_string());
        self.roll_dice(d.n_dice, d.die_type) + d.bonus
    }

    pub fn random_slice_index<T>(&mut self, slice: &[T]) -> Option<usize> {
        match slice.len() {
            0 => None,
            _ => Some(self.range(0, slice.len())),
        }
    }

    pub fn random_slice_entry<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        match slice.len() {
            0 => None,
            _ => Some(&slice[self.range(0, slice.len())]),
        }
    }
}

impl Default for RandomGen {
    fn default() -> Self {
        Self::new(get_seed())
    }
}

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

fn get_seed() -> u64 {
    let mut buf = [0u8; 8];
    if getrandom::getrandom(&mut buf).is_ok() {
        u64::from_be_bytes(buf)
    } else {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
