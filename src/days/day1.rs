use super::read::get_contents;

#[derive(Debug)]
struct TopElfs {
    number: i32,
    sum: i32,
}

#[allow(dead_code)]
fn find_top_three(path: String) -> Vec<TopElfs> {
    let contents = get_contents(path);
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

pub fn top_elf_number(path: String) -> i32 {
    let elfs = find_top_three(path);

    elfs[0].number
}

