mod days;

fn main() {
    println!("Answer to day 1: {}", days::day1::top_elf_sum("./inputs/day1/input2".to_string()));
    println!("Answer to day1 part2: {}", days::day1::top_three_sum("./inputs/day1/input2".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::days;

    #[test]
    fn test_day1() {
        assert_eq!(days::day1::top_elf_number("./inputs/day1/input".to_string()), 4);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(days::day1::top_three_sum("./inputs/day1/input".to_string()), 45000);
    }
}

