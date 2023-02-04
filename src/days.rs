use std::fs::File;
use std::io::prelude::*;
   
pub mod day1 {
    use super::*;

    #[allow(dead_code)]
    pub fn find_top_elf(path: String) -> i32 {
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

    #[derive(Debug)]
    struct TopElfs {
        number: i32,
        sum: i32,
    }

    #[allow(dead_code)]
    fn find_top_three(path: String) -> Vec<TopElfs> {
        let mut f = File::open(path).expect("Could not open file");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Could not read from file");
        let mut elf_count = 1;
        let mut sum = 0;
        let mut top_three: Vec<TopElfs> = Vec::new();

        let mut elf1 = TopElfs { number: 0, sum: 0 };
        let mut elf2 = TopElfs { number: 0, sum: 0 };
        let mut elf3 = TopElfs { number: 0, sum: 0 };

        for line in contents.lines() {
            if line.is_empty() {
                elf_count += 1; 
                sum = 0;
            } else {
                sum += line.parse::<i32>().unwrap();
            }

            if sum > elf1.sum {
                elf1.sum = sum;
                elf1.number = elf_count;
            } else if sum > elf2.sum {
                elf2.sum = sum;
                elf2.number = elf_count;
            } else if sum > elf3.sum {
                elf3.sum = sum;
                elf3.number = elf_count;
            }
        }

        top_three.push(elf1);
        top_three.push(elf2);
        top_three.push(elf3);

        top_three
    }

    #[allow(dead_code)]
    pub fn top_three_sum(path: String) -> i32 {
        let elfs = find_top_three(path);
        let mut sum = 0;

        for i in 0..3 {
            sum += elfs[i].sum;
        }

        sum
    }
}

