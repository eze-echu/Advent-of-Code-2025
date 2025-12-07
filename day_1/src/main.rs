#![feature(test)]
mod test;

fn main() {
    let mut zero_counter = 0;
    let mut skip_count = 0;
    let read_input = std::fs::read_to_string("input.txt");
    if let Ok(input) = read_input {
        let mut dial = 50;
        let instructions = input.split("\n").collect::<Vec<&str>>();
        instructions.iter().for_each(|&instruction| {
            let skip = spin(&mut dial, instruction);
            if dial == 0 {
                zero_counter += 1;
            }
            skip_count += skip;
        });
        println!("Counter = {}", zero_counter);
        println!("Skip Count = {}", skip_count);
    } else {
        eprintln!("{}", read_input.unwrap_err());
    }
}
//  Counter = 1129
//  Skip Count = 6638
pub fn spin(dial: &mut i32, instruction: &str) -> i32 {
    let og = *dial;
    let mut amount;
    match instruction.chars().nth(0).unwrap_or('\0') {
        'R' => {
            amount = String::from(instruction)[1..].parse::<i32>().unwrap();
        }
        'L' => {
            amount = -String::from(instruction)[1..].parse::<i32>().unwrap();
        }
        _ => {
            amount = 0;
        }
    }
    let skips = calculate_skips_over_zero(*dial, amount);
    calculate_spin_position(dial, amount);
    assert!(*dial >= 0 && *dial < 100);
    skips
}
fn calculate_skips_over_zero(dial: i32, amount_spun: i32) -> i32 {
    let spins = amount_spun.abs() / 100;
    if amount_spun < 0 && dial - amount_spun.abs() % 100 <= 0 && dial != 0 {
        let spins = spins + 1;
        return spins;
    } else if amount_spun > 0 && dial + amount_spun.abs() >= 100 && dial != 0 {
        let spins = spins + 1;
        return spins;
    }
    spins
}

fn calculate_spin_position(dial: &mut i32, amount_spun: i32) {
    if amount_spun < 0 {
        *dial -= amount_spun.abs();
        if *dial < 0 {
            *dial = (100 - ((*dial % 100) * -1)) % 100;
        }
    } else {
        *dial += amount_spun;
        if *dial >= 100 {
            *dial = *dial % 100;
        }
    }
}

