use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
   
pub mod day1 {
    use super::*;

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
        let mut top_three: Vec<TopElfs> = Vec::new();

        let mut elf1 = TopElfs { number: 0, sum: 0 };
        let mut elf2 = TopElfs { number: 0, sum: 0 };
        let mut elf3 = TopElfs { number: 0, sum: 0 };

        let mut sum = 0;
        let mut sums: Vec<i32> = Vec::new();
        for line in contents.lines() {
            if line.is_empty() { 
                sums.push(sum);
                sum = 0;
            } else {
                sum += line.parse::<i32>().unwrap();
            }
        } 

        let mut i = 0;
        let mut j = sums.len() - 1;
        for _ in sums.iter() {
            if sums[i] > sums[j] {
                elf1.sum = sums[i];
                elf1.number = i as i32 + 1;
                if sums[j] > elf2.sum && sums[j] < elf1.sum {
                    elf3.sum = elf2.sum;
                    elf3.number = elf2.number;
                    elf2.sum = sums[j];
                    elf2.number = j as i32 + 1;
                } else if sums[j] > elf3.sum && sums[j] < elf2.sum {
                    elf3.sum = sums[j];
                    elf3.number = j as i32 + 1;
                }

                j -= 1;
            } else {
                elf1.sum = sums[j];
                elf1.number = j as i32 + 1;

                if sums[i] > elf2.sum && sums[i] < elf1.sum {
                    elf3.sum = elf2.sum;
                    elf3.number = elf2.number;
                    elf2.sum = sums[i];
                    elf2.number = i as i32 + 1;
                } else if sums[i] > elf3.sum && sums[i] < elf2.sum {
                    elf3.sum = sums[i];
                    elf3.number = i as i32 + 1;
                }

                i += 1;
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

    pub fn top_elf_sum(path: String) -> i32 {
        let elfs = find_top_three(path);
        let sum = elfs[0].sum;

        sum
    }
}

pub fn get_contents(path: String) -> String {
    let mut f = File::open(path).expect("Could not open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file");

    contents
}

pub mod day2 {
    use super::*;

    #[allow(dead_code)]
    pub fn get_score(which: usize, path: String) -> i32 {
        let chars: Vec<char> = get_contents(path).replace(" ", "").replace("\n", "").chars().collect();
        let mut player1_score = 0;
        let mut player2_score = 0;
        let letter_to_play = HashMap::from([
            ('A', String::from("Rock")),
            ('X', String::from("Rock")),
            ('B', String::from("Paper")),
            ('Y', String::from("Paper")),
            ('C', String::from("Scissors")),
            ('Z', String::from("Scissors")),
        ]);
        let score_by_play = HashMap::from([
            ("Rock".to_string(), 1),
            ("Paper".to_string(), 2),
            ("Scissors".to_string(), 3),
        ]);

        let mut play1: String;
        let mut play2: String;
        for i in (0..chars.len()).step_by(2) {
            let j = i + 1;
            play1 = String::from(letter_to_play.get(&chars[i]).unwrap());
            play2 = String::from(letter_to_play.get(&chars[j]).unwrap());

            if play1 == play2 {
                player1_score += 3;
                player2_score += 3;
            } else if play1 == "Paper" && play2 == "Rock" {
                player1_score += 6;
            } else if play1 == "Scissors" && play2 == "Paper" {
                player1_score += 6;
            } else if play1 == "Rock" && play2 == "Scissors" {
                player1_score += 6;
            } else {
                player2_score += 6;
            }

            player1_score += score_by_play.get(&play1).unwrap();
            player2_score += score_by_play.get(&play2).unwrap();
        }

        let mut scores = Vec::new();
        scores.push(player1_score);
        scores.push(player2_score);
        scores[which - 1]
    }
}
