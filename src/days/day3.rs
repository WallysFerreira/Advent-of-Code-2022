use super::get_contents;

// 1st find letters that are in both halves of string (case sensitive)
// then sum priorities of each letters
/*
pub fn sum_priorities(path: String) -> i32 {
    let items = find_items(get_halves(path));

    0
}
*/

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
/*
pub fn find_items((halve1, halve2): (String, String)) -> Vec<char> {
}
*/
