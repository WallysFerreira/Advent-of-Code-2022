use super::get_contents;

// 1st find letters that are in both halves of string (case sensitive)
// then sum priorities of each letters
/*
pub fn sum_priorities(path: String) -> i32 {
    let items = find_items(get_halves(path));

    0
}
*/

pub fn get_halves(path: String) -> (String, String) {
    let contents = get_contents(path);

    println!("{:?}", contents);
    ("n".to_string(), "b".to_string())
}
/*
pub fn find_items((halve1, halve2): (String, String)) -> Vec<char> {
}
*/
