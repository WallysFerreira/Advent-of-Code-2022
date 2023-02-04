use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
fn find_top_elf(path: String) -> i32 {
    let mut f = File::open(path).expect("Could not open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read from file");
    let mut right_elf = 0;
    let mut elf_count = 1;
    let mut sum = 0;
    let mut biggest_sum = 0;

    for line in contents.lines() {
        if line.is_empty() {
            elf_count += 1; 
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }

        if sum > biggest_sum {
            biggest_sum = sum;
            right_elf = elf_count;
        }
    }

    println!("{} is carrying the most amount of calories: {}", right_elf, biggest_sum);   
    right_elf
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        assert_eq!(find_top_elf("./inputs/day1/input.txt".to_string()), 4);
    }

    #[test]
    fn test_day1_2() {
        assert_eq!(find_top_elf("./inputs/day1/input2.txt".to_string()), 216);
    }
}

