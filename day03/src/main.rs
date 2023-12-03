// With padding there is no need to check bounds all the time
fn get_input(path: &str) -> Vec<String>{
    let mut data: Vec<String> = std::fs::read_to_string(path).unwrap().lines().map(|l| format!(".{l}.")).collect();
    let n_y: usize = data[0].len();

    let mut output = Vec::new();
    output.push((0..n_y).map(|_c| '.').collect());
    output.append(&mut data);
    output.push((0..n_y).map(|_c| '.').collect());
    return output
}
//
fn part_sum(data: &Vec<String>) -> usize{
    // Init
    let mut sum = 0;
    let num_regex: regex::Regex = regex::Regex::new(r"\d+").unwrap();
    let symbol_regex = regex::Regex::new(r"[^\d.]").unwrap();
    // Loop
    for i in 1..(data.len()-1){
        for num in num_regex.find_iter(data[i].as_str()){
            let s = num.start();
            let l = num.len();
            if     symbol_regex.find(&data[i-1][s-1..(s+l+1)]).is_some()
                || symbol_regex.find(&data[i  ][s-1..(s+l+1)]).is_some()
                || symbol_regex.find(&data[i+1][s-1..(s+l+1)]).is_some() {
                sum += num.as_str().parse::<usize>().unwrap();
            }
        }
    }
    return sum
}
//
fn gear_sum(data: &Vec<String>) -> usize{
    let num_regex = regex::Regex::new(r"\d+").unwrap();
    let mut gears = std::collections::HashMap::new();
    for i in 1..(data.len()-1){
        'numLoop: for num in num_regex.find_iter(data[i].as_str()){
            for k in (i-1)..(i+2){
                for l in num.start()-1..num.end()+1{
                    if &data[k][l..l+1] == "*"{
                        gears.entry((k,l)).or_insert(vec![]).push(num.as_str().parse::<usize>().unwrap());
                        continue 'numLoop
                    }
                }
            }
        }
    }
    return gears.iter().map(|(_,nums)| (nums.len()-1)*nums.iter().fold(1, |acc,n| acc*n)).sum();
}
//
fn main() {
    // Load data
    let test = get_input("data/test.txt");
    let data = get_input("data/data.txt");

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1: {:?} - 4361. Completed in {:.2?}", part_sum(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", part_sum(&data), now.elapsed()); // Not 521503 < z < 523494.

    // // Part 2
    let now = std::time::Instant::now();
    println!("Test 2: {:?} - 467835. Completed in {:.2?}", gear_sum(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", gear_sum(&data), now.elapsed());
}