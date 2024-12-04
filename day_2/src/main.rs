// The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are 
// separated by spaces. For example:

// 7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9

// The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either 
// gradually increasing or gradually decreasing. 

// So, a report only counts as safe if both of the following are true:
//     - The levels are either all increasing or all decreasing.
//     - Any two adjacent levels differ by at least one and at most three.

fn check_safety(level: &[i32]) -> bool {
    if level.len() < 2 {
        return false;
    }

    let ascending = level[0] < level[1];

    for i in 0..(level.len() - 1) {
        if (level[i] == level[i + 1])
        || ((level[i] as i32 - level[i + 1] as i32).abs() > 3) 
        || (ascending && level[i] > level[i + 1]) 
        || (!ascending && level[i] < level[i + 1]) {
            return false;
        }
    }

    true
}

fn main() {
    assert!(check_safety(&[7, 6, 4, 2, 1]));
    assert!(!check_safety(&[1, 2, 7, 8, 9]));
    assert!(!check_safety(&[9, 7, 6, 2, 1]));
    assert!(!check_safety(&[1, 3, 2, 4, 5]));
    assert!(!check_safety(&[8, 6, 4, 4, 1]));
    assert!(check_safety(&[1, 3, 6, 7, 9]));

    println!("Complete!");
}
