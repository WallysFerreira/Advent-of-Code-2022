use super::read::get_contents;

#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

fn fill_range(path: String) -> Vec<Vec<i32>> {
    let ranges: Vec<Range> = find_range_limits(path);
    let mut ranges_filled = Vec::new();

    for i in 0..ranges.len() {
        let mut fill = Vec::new();

        for l in ranges[i].start..=ranges[i].end {
            fill.push(l);
        }

        ranges_filled.push(fill);
    }

    ranges_filled
}

pub fn count_overlaps(path: String) -> i32 {
    let ranges = fill_range(path);
    let mut count = 0;

    dbg!(&ranges);

    for i in (0..ranges.len()).step_by(2) {
        let j = i + 1;

        if ranges[i].contains(ranges[j].iter().next().unwrap()) {
            count += 1;
            continue;
        }

        if ranges[j].contains(ranges[i].iter().next().unwrap()) {
            count += 1;
        }
    }

    count
}

// Calls expand_ranges
fn find_range_limits(path: String) -> Vec<Range> {
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

    ranges_collection
}

pub fn count_full_overlaps(path: String) -> i32 {
    let ranges: Vec<Range> = find_range_limits(path);
    let mut count1: i32 = 0;
    let mut count2: i32 = 0;

    for i in (0..ranges.len()).step_by(2) {
        let j = i + 1;


        if ranges[i].start <= ranges[j].start && ranges[i].end >= ranges[j].end {
            count1 += 1;
            continue;
        }

        if ranges[j].start <= ranges[i].start && ranges[j].end >= ranges[i].end {
            count2 += 1;
        }
    }

    count1 + count2
}
// Expand both ranges and store each number into a vector
// check if one array is contained in the other
