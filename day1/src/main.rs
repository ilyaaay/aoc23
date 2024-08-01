use std::{fs, io::Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;

    println!("{}", calc_first_part(input.as_str()));
    println!("{}", calc_second_part(input.as_str()));

    Ok(())
}

#[inline(always)]
fn calc_first_part(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|x| {
            match (
                x.chars().filter(|ch| ch.is_digit(10)).nth(0),
                x.chars().filter(|ch| ch.is_digit(10)).nth_back(0),
            ) {
                (Some(x), Some(y)) => format!("{x}{y}").parse::<i32>().ok(),
                _ => None,
            }
        })
        .sum()
}

#[inline(always)]
fn calc_second_part(input: &str) -> i32 {
    1
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_calc_first_part() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(calc_first_part(input), 142);
    }

    #[test]
    fn test_calc_second_part() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        calc_second_part(input);
    }
}
