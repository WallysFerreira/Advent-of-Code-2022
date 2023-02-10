use super::read::get_contents;

#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

// Use for loop to expand ranges and insert into two vectors
fn expand_ranges(ranges: Vec<Range>) -> (Vec<i32>, Vec<i32>){
    for i in 0..ranges.len() {
        for j in ranges[i].start..ranges[i].end {

        }
    }
    
    (Vec::from([0]), Vec::from([0]))
}

// Calls expand_ranges
fn find_range_limits(path: String) {
    let contents = get_contents(path);
    let mut ranges_collection: Vec<Range> = Vec::new();

    for line in contents.lines() {
        let ranges: Vec<&str> = line.split(',').collect();

        let range1_limits: Vec<&str> = ranges[0].split('-').collect();
        let range1 = Range {
            start: range1_limits[0].parse::<i32>().expect("Cant parse to integer"),
            end: range1_limits[1].parse::<i32>().expect("Cant parse to integer"),
        };

        let range2_limits: Vec<&str> = ranges[1].split('-').collect();
        let range2 = Range {
            start: range2_limits[0].parse::<i32>().expect("Cant parse to integer"),
            end: range2_limits[1].parse::<i32>().expect("Cant parse to integer"),
        };

        ranges_collection.push(range1);
        ranges_collection.push(range2);
    }

    println!("{:?}", ranges_collection);
}

pub fn count_overlaps(path: String) -> i32 {
    find_range_limits(path);

    0
}
// Expand both ranges and store each number into a vector
// check if one array is contained in the other
