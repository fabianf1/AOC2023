use std::time::Instant;

fn get_input(path: &str) -> Vec<String>{
    return std::fs::read_to_string(path).unwrap().lines().map(|str| str.to_string()).collect();
}
//
fn get_calibration_sum(data: &Vec<String>, spelled: bool) -> i32{
    let mut sum:i32 = 0;
    // Create vector with valid digits
    let mut digits:Vec<&str> = vec!["1","2","3","4","5","6","7","8","9"];
    if spelled{
        digits.append(&mut vec!["one","two","three","four","five","six","seven","eight","nine"])
    }
    // Loop over every line
    for line in data{
        let minInd = digits.clone().iter().map(|x| line.find(x) ).enumerate().filter(|(_, v)| v.is_some()).min_by(|(_, v), (_, v2)| v.cmp(v2)).map(|(idx, _)| idx).unwrap();
        let maxInd = digits.clone().iter().map(|x| line.rfind(x)).enumerate().filter(|(_, v)| v.is_some()).max_by(|(_, v), (_, v2)| v.cmp(v2)).map(|(idx, _)| idx).unwrap();
        sum += format!("{}{}", (minInd % 9) + 1, (maxInd % 9) + 1).parse::<i32>().unwrap();
    }
    return sum
}
//
fn main() {
    // Load data
    let test = get_input("data/test.txt");
    let test2 = get_input("data/test2.txt");
    let data = get_input("data/data.txt");

    // Part 1
    let now = Instant::now();
    println!("Test 1: {:?} - 142. Completed in {:.2?}", get_calibration_sum(&test, false), now.elapsed());
    let now = Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", get_calibration_sum(&data, false), now.elapsed());

    // Part 2
    let now = Instant::now();
    println!("Test 2: {:?} - 281. Completed in {:.2?}", get_calibration_sum(&test2, true), now.elapsed());
    let now = Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", get_calibration_sum(&data, true), now.elapsed());
}