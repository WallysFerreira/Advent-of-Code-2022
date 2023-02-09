use super::get_contents;

// 1st find letters that are in both halves of string (case sensitive)
// then sum priorities of each letters
pub fn sum_priorities(path: String) -> i32 {
    let items = find_items(get_halves(path));

    println!("{:?}", items);
    0
}


pub fn get_halves(path: String) -> Vec<String> {
    let contents = get_contents(path);
    let mut halves = Vec::new();

    for lines in contents.split('\n') {
        if !lines.is_empty() {
            println!("{}", lines);
            let (h1, h2) = lines.split_at(lines.len() / 2);
            halves.push(h1.to_string());
            halves.push(h2.to_string());
        }
    }

    halves
}

pub fn find_items(halves: Vec<String>) -> Vec<char> {
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
