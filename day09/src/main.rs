fn get_data(path: &str) -> Vec<Vec<i64>>{
    std::fs::read_to_string(path).unwrap().lines().map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect()
}
// 
fn get_diff(data: &Vec<i64>) -> Vec<Vec<i64>>{
    let mut diff: Vec<Vec<i64>> = vec![data.clone()];
    while diff.last().unwrap().iter().filter(|n| *n==&0).count() != diff.last().unwrap().len(){
        diff.push(diff.last().unwrap().windows(2).map(|a| a[1]-a[0]).collect());
    }
    return diff
}
//
fn get_extrap_sum(data: &Vec<Vec<i64>>) -> i64{
    data.iter().fold(0, |acc, x| acc + get_diff(x).iter().fold(0, |acc,x| acc + x.last().unwrap()))
}
//
fn get_previous_sum(data: &Vec<Vec<i64>>) -> i64{
    data.iter().fold(0, |acc, x| acc + get_diff(x).iter().fold(0, |acc,x| x.first().unwrap() - acc))
}
//
fn main() {
    // Load data
    let test = get_data("data/test.txt");
    let data = get_data("data/data.txt");

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1: {:?} - 114. Completed in {:.2?}", get_extrap_sum(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", get_extrap_sum(&data), now.elapsed());

    // Part 2
    let now = std::time::Instant::now();
    println!("Test 2: {:?} - 2. Completed in {:.2?}", get_previous_sum(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", get_previous_sum(&data), now.elapsed());
}