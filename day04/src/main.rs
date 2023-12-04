//
fn get_input(path: &str) -> Vec<Vec<Vec<u32>>> {
    return std::fs::read_to_string(path).unwrap().lines()
        .map(|s| {
            s.split(&[':', '|']).skip(1)
            .map(|ss| ss.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .collect()
        })
        .collect();
}
// 
fn get_points(data: &[Vec<Vec<u32>>]) -> usize{
    let mut sum = 0;
    for card in data{
        let count = card[1].iter().fold(0, |acc, x| acc + card[0].contains(x) as u32);
        if count > 0{
            sum += (2 as usize).pow(count - 1);
        }
    }
    return sum
}
//
fn get_num_cards(data: &[Vec<Vec<u32>>]) -> usize{
    let wins =  data.iter()
    .map(|s| s[1].iter().fold(0, |acc, x| acc + s[0].contains(x) as usize))
    .collect::<Vec<usize>>();

    let mut cards = vec![1; wins.len()];
    for idx in 0..data.len(){
        for num in 0..wins[idx]{
            let new_idx = idx + num + 1;
            cards[new_idx] += cards[idx];
        }
    }
    return cards.iter().fold(0, |acc, x| acc + x)
}
//
fn main() {
    // Load data
    let test = get_input("data/test.txt");
    let data = get_input("data/data.txt");

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1: {:?} - 13. Completed in {:.2?}", get_points(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", get_points(&data), now.elapsed());

    // // Part 2
    let now = std::time::Instant::now();
    println!("Test 2: {:?} - 30. Completed in {:.2?}", get_num_cards(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", get_num_cards(&data), now.elapsed());
}