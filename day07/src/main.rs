fn get_data(path: &str) -> Vec<(String, u64)>{
    return std::fs::read_to_string(path).unwrap().lines()
        .map(|s| s.split_once(" ").unwrap())
        .map(|(h,b)| (String::from(h), b.parse().unwrap()))
        .collect();
}
fn get_type(a: &String, joker: bool) -> u64{
    let mut map: std::collections::HashMap<char, u64> = std::collections::HashMap::new();
    for c in a.chars().filter(|&x| !joker || x != 'J'){
        map.insert(c, map.get(&c).unwrap_or(&0) + 1);
    }
    let mut occ = map.into_values().collect::<Vec<_>>();
    occ.sort();
    if occ.len()==0 {
        occ.push(5)
    }
    else if joker {
        *occ.last_mut().unwrap() += a.chars().filter(|&x| x == 'J').count() as u64;
    }
    match &occ[..]{
        &[..,5]   => return 6,
        &[..,4]   => return 5,
        &[..,2,3] => return 4,
        &[..,3]   => return 3,
        &[..,2,2] => return 2,
        &[..,2]   => return 1,
        _         => return 0, 
    }
}
//
fn card_sort(a: &(String, u64), b: &(String, u64), joker: bool) -> std::cmp::Ordering{
    let cmp = get_type(&b.0, joker).cmp(&get_type(&a.0, joker));
    if cmp != std::cmp::Ordering::Equal{
        return cmp;
    }
    let mut order = "AKQJT98765432";
    if joker{
        order = "AKQT98765432J";
    }
    return a.0.chars().fold(0, |acc, c| acc*13 + order.find(c).unwrap()).cmp(&b.0.chars().fold(0, |acc, c| acc*13 + order.find(c).unwrap()));
}
//
fn total_winnings(data: &Vec<(String, u64)>, joker: bool) -> u64{
    let mut sorted = data.clone();
    sorted.sort_by(|a,b| card_sort(b, a, joker));
    return sorted.iter().enumerate().fold(0,|acc,(i, x)| acc + x.1 * ((i+1) as u64))
}
//
fn main() {
    // Load data
    let test = get_data("data/test.txt");
    let data = get_data("data/data.txt");

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1: {:?} - 6440. Completed in {:.2?}", total_winnings(&test, false), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", total_winnings(&data, false), now.elapsed());

    // Part 2
    let now = std::time::Instant::now();
    println!("Test 2: {:?} - 5905. Completed in {:.2?}", total_winnings(&test, true), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", total_winnings(&data, true), now.elapsed());
}