mod days;

fn main() {
    println!("Answer to day 1: {}", days::day1::top_elf_sum("./inputs/day1/input2".to_string()));
    println!("Answer to day 1 part 2: {}", days::day1::top_three_sum("./inputs/day1/input2".to_string()));
    println!("Answer to day 2: {}", days::day2::get_score_2_plays(2, "./inputs/day2/inputr".to_string()));
    println!("Answer to day 2 part 2: {}", days::day2::get_score_by_outcome(2, "./inputs/day2/inputr".to_string()));
    println!("Answer to day 3: {}", days::day3::sum_priorities(1, "./inputs/day3/inputr".to_string()));
    println!("Answer to day 3 part 2: {}", days::day3::sum_priorities(2, "./inputs/day3/inputr".to_string()));
    println!("Answer to day 4: {}", days::day4::count_full_overlaps("./inputs/day4/inputr".to_string()));
    println!("Answer to day 4 part 2: {}", days::day4::count_overlaps("./inputs/day4/inputr".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::days;

    #[test]
    fn test_day1() {
        assert_eq!(days::day1::top_elf_sum("./inputs/day1/input".to_string()), 24000);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(days::day1::top_three_sum("./inputs/day1/input".to_string()), 45000);
    }

    #[test]
    fn test_day2() {
        assert_eq!(days::day2::get_score_2_plays(2, "./inputs/day2/input".to_string()), 15);
    }

    #[test]
    fn test_day2_2() {
        assert_eq!(days::day2::get_score_2_plays(2, "./inputs/day2/input2".to_string()), 45);
    }

    #[test]
    fn test_day2_part2() {
        assert_eq!(days::day2::get_score_by_outcome(2, "./inputs/day2/input".to_string()), 12);
    }

    #[test]
    fn test_day2_part2_2() {
        assert_eq!(days::day2::get_score_by_outcome(2, "./inputs/day2/input2".to_string()), 45);
    }

    #[test]
    fn test_day3() {
        assert_eq!(days::day3::sum_priorities(1, "./inputs/day3/input".to_string()), 157);
    }

    #[test]
    fn test_day3_part2() {
        assert_eq!(days::day3::sum_priorities(2, "./inputs/day3/input".to_string()), 70);
    }

    #[test]
    fn test_day4() {
        assert_eq!(days::day4::count_full_overlaps("./inputs/day4/input".to_string()), 2);
    }

    #[test]
    fn test_day4_part2() {
        assert_eq!(days::day4::count_overlaps("./inputs/day4/input".to_string()), 4);
    }
}

