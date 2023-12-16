fn get_data(path: &str) -> Vec<String>{
    std::fs::read_to_string(path).unwrap().split(",").map(|s| s.to_owned()).collect()
}
//
fn hash_sum(data: &Vec<String>) -> u64{
    data.iter().fold(0, |acc, s| acc + s.as_bytes().iter().fold(0, |acc2, b| (acc2 + (*b as u64)) * 17 % 256))
}
//
fn get_focusing_power(data: &Vec<String>) -> u64{
    let mut b: std::collections::HashMap<u64, Vec<(String, u64)>> = std::collections::HashMap::new();
    for s in data{
        let idx = s.as_bytes().iter().take_while(|c| c.is_ascii_alphabetic()).fold(0, |acc, b| (acc + (*b as u64)) * 17 % 256);
        if s.contains("="){
            let (lab, num) = s.split_once("=").unwrap();
            if b.get(&idx).is_none(){
                b.insert(idx, vec![(lab.to_owned(), num.parse().unwrap())]);
            }
            else if let Some(l) = b.get_mut(&idx).unwrap().iter_mut().find(|(l,_n)| l == lab){
                l.1 = num.parse::<u64>().unwrap();
            }
            else{
                b.get_mut(&idx).unwrap().push((lab.to_owned(), num.parse().unwrap()));
            }

        }
        else{
            if b.get(&idx).is_some(){
                if let Some(l) = b.get(&idx).unwrap().iter().enumerate().find(|(_i,(l, _n))| *l == s[..s.len()-1]){
                    let l = l.0.clone(); // To satisfy the borrow checker
                    b.get_mut(&idx).unwrap().remove(l);
                }
            }            
        }
    }
    // Calculate
    let mut sum = 0;
    for (i, v) in b.iter(){
        for (j,(_lab, num)) in v.iter().enumerate(){
            sum += (i+1) * (j as u64 + 1) * num;
        }
    }
    return sum
}
//
fn main() {
    // Load data
    let test = get_data("data/test.txt");
    let test_hash = get_data("data/test_hash.txt");
    let data = get_data("data/data.txt");

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1.1: {:?} - 52. Completed in {:.2?}", hash_sum(&test_hash), now.elapsed());
    println!("Test 1.2: {:?} - 1320. Completed in {:.2?}", hash_sum(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?} . Completed in {:.2?}", hash_sum(&data), now.elapsed());

    // Part 2
    let now = std::time::Instant::now();
    println!("Test 2: {:?} - 145. Completed in {:.2?}", get_focusing_power(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?} . Completed in {:.2?}", get_focusing_power(&data), now.elapsed());
}