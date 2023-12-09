fn get_data(path: &str) -> (String, std::collections::HashMap<String, Vec<String>>){
    let binding = std::fs::read_to_string(path).unwrap();
    let data = binding.split_once("\r\n\r\n").unwrap();

    let mut map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    for l in data.1.lines().map(|s| s.split_once(" = (").unwrap()){
        let a: Vec<String> = l.1.split_whitespace().map(|s| s[0..3].to_owned()).collect();
        map.insert( l.0.to_owned(), a );
    }

    return (data.0.to_owned(), map)
}
//
fn steps_required(order: &String, map: &std::collections::HashMap<String, Vec<String>>) -> u64{
    let mut order_iter = order.chars().cycle();
    let mut curr = "AAA";
    let mut steps = 0;
    while curr != "ZZZ"{
        curr = match order_iter.next().unwrap(){
            'L' => &map.get(curr).unwrap()[0],
            _ => &map.get(curr).unwrap()[1],
        };
        steps += 1;
    }
    return steps
}
// Euclid's algorithm
fn get_gcd(num1: u64, num2: u64) -> u64{
    let mut a = num1.max(num2);
    let mut b = num1.min(num2);
    while a!=b && b>0 {
        let t = a;
        a = b;
        b = t%b;
    }
    return a;
}
//
fn steps_required_ghost(order: &String, map: &std::collections::HashMap<String, Vec<String>>) -> u64{
    // Find starting nodes
    let mut curr = Vec::new();
    for node in map.keys(){
        if node.chars().nth(2).unwrap() == 'A'{
            curr.push(node.as_str());
        }
    }
    // Number of steps for each ghost
    let mut step_vec: Vec<u64> = Vec::new();
    for mut node in curr{
        let mut order_iter = order.chars().cycle();
        let mut steps = 0;
        while node.chars().nth(2).unwrap() != 'Z'{
            node = match order_iter.next().unwrap(){
                'L' => &map.get(node).unwrap()[0],
                _ => &map.get(node).unwrap()[1],
            };
            steps += 1;
        }
        step_vec.push(steps)
    }
    // Find LCM using GCD
    for i in (0..(step_vec.len()-1)).rev(){
        let gcd = get_gcd(step_vec[i], step_vec[i+1]);
        step_vec[i] = step_vec[i] * step_vec[i+1] / gcd;
    }
    return step_vec[0]
}
//
fn main() {
    // Load data
    let test = get_data("data/test.txt");
    let test2 = get_data("data/test2.txt");
    let test3 = get_data("data/test3.txt");
    let data = get_data("data/data.txt");

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1.1: {:?} - 2. Completed in {:.2?}", steps_required(&test.0, &test.1), now.elapsed());
    println!("Test 1.2: {:?} - 6. Completed in {:.2?}", steps_required(&test2.0, &test2.1), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", steps_required(&data.0, &data.1), now.elapsed());

    // Part 2
    let now = std::time::Instant::now();
    println!("Test 2: {:?} - 6. Completed in {:.2?}", steps_required_ghost(&test3.0, &test3.1), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", steps_required_ghost(&data.0, &data.1), now.elapsed());
}