mod days;

fn main() {
    println!("{}", days::day1::top_three_sum("./inputs/day1/input2.txt".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::days;

    #[test]
    fn test_day1() {
        assert_eq!(days::day1::find_top_elf("./inputs/day1/input.txt".to_string()), 4);
    }

    #[test]
    fn test_day1_2() {
        assert_eq!(days::day1::find_top_elf("./inputs/day1/input2.txt".to_string()), 216);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(days::day1::top_three_sum("./inputs/day1/input2.txt".to_string()), 195436);
    }
}

