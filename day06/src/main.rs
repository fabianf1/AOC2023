fn ways_to_win(data: &str) -> usize{
    let nums:Vec<Vec<usize>> = data.lines().map(|s| s.split_once(":").unwrap().1.split_whitespace().map(|ss| ss.parse().unwrap()).collect()).collect();

    let mut ways = Vec::new();
    for i in 0..nums[0].len(){
        let mut num = 0;
        for j in 0..nums[0][i]{
            if (nums[0][i]-j)*j > nums[1][i]{
                num+=1;
            }
        }
        ways.push(num);
    }
    return ways.iter().fold(1, |acc,x| acc*x);
}
//
fn ways_to_win_fixed(data: &str) -> usize{
    ways_to_win(&data.replace(" ", ""))
}
//
fn main() {
    // Load data
    let test = std::fs::read_to_string("data/test.txt").unwrap();
    let data = std::fs::read_to_string("data/data.txt").unwrap();

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1: {:?} - 288. Completed in {:.2?}", ways_to_win(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", ways_to_win(&data), now.elapsed());

    // Part 2
    let now = std::time::Instant::now();
    println!("Test 2: {:?} - 71503. Completed in {:.2?}", ways_to_win_fixed(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", ways_to_win_fixed(&data), now.elapsed());
}