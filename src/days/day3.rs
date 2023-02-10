use super::read::get_contents;
use std::collections::HashMap;

// 1st find letters that are in both halves of string (case sensitive)
// then sum priorities of each letters
pub fn sum_priorities(what: i32, path: String) -> i32 {
    let items: Vec<char>;
    if what == 1 {
        items = find_items(get_halves(path));
    } else {
        items = find_badges(path);
    }
    let mut sum = 0;
    let values = HashMap::from([
        ('a', 1), ('A', 27), ('b', 2), ('B', 28), ('c', 3), ('C', 29), ('d', 4), ('D', 30),
        ('e', 5), ('E', 31), ('f', 6), ('F', 32), ('g', 7), ('G', 33), ('h', 8), ('H', 34),
        ('i', 9), ('I', 35), ('j', 10), ('J', 36), ('k', 11), ('K', 37), ('l', 12), ('L', 38),
        ('m', 13), ('M', 39), ('n', 14), ('N', 40), ('o', 15), ('O', 41), ('p', 16), ('P', 42),
        ('q', 17), ('Q', 43), ('r', 18), ('R', 44), ('s', 19), ('S', 45), ('t', 20), ('T', 46),
        ('u', 21), ('U', 47), ('v', 22), ('V', 48), ('w', 23), ('W', 49), ('x', 24), ('X', 50),
        ('y', 25), ('Y', 51), ('z', 26), ('Z', 52)]);

    for item in items {
       sum += values.get(&item).unwrap();
    }

    sum
}

fn get_halves(path: String) -> Vec<String> {
    let contents = get_contents(path);
    let mut halves = Vec::new();

    for lines in contents.split('\n') {
        if !lines.is_empty() {
            let (h1, h2) = lines.split_at(lines.len() / 2);
            halves.push(h1.to_string());
            halves.push(h2.to_string());
        }
    }

    halves
}

fn find_items(halves: Vec<String>) -> Vec<char> {
    let mut items: Vec<char> = Vec::new();

    for i in (0..halves.len()).step_by(2){
        let j = i + 1;
        
        let half1: String = halves[i].clone();
        let half2: String = halves[j].clone();
        let mut found: bool = false;
        
        for half1_item in half1.chars() {
            for half2_item in half2.chars() {
                if half1_item == half2_item {
                    items.push(half1_item);
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }

    items
}

// Separate into groups of 3 lines
// find items that are in all 3 lines
fn find_badges(path: String) -> Vec<char>{
    let contents = get_contents(path);
    let lines: Vec<&str> = contents.split('\n').collect::<Vec<&str>>();
    let mut badges: Vec<char> = Vec::new();

    for i in (0..lines.len()).step_by(3) {
        let j = i + 1;
        let k = i + 2;
        let mut found = false;
        
        for item in lines[i].chars() {
            if lines[j].chars().any(|item2| item2 == item) && lines[k].chars().any(|item3| item3 == item) {
                if !found {
                    found = true;
                    badges.push(item);
                }
            }
        }
    }
    
    badges
}
