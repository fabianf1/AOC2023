fn get_data(path: &str) -> Vec<Vec<char>>{
    std::fs::read_to_string(path).unwrap().lines().map(|l| l.chars().collect()).collect()
}
//
fn shortest_path_sum(data:  &Vec<Vec<char>>, exp: usize) -> usize{
    // Can be more efficient with vectors, but this is more clear
    let mut r_empty: std::collections::HashSet<usize> = std::collections::HashSet::from_iter(0..data.len());
    let mut c_empty: std::collections::HashSet<usize> = std::collections::HashSet::from_iter(0..data[0].len());
    let mut g = Vec::new();
    for i in 0..data.len(){
        for j in 0..data[0].len(){
            if data[i][j] == '#'{
                r_empty.remove(&i);
                c_empty.remove(&j);
                g.push((i,j));
            }
        }
    }
    // Expand space
    for i in 0..g.len(){
        let add_x = (0..g[i].0).filter(|x: &usize| r_empty.get(x).is_some()).count() * (exp-1);
        let add_y = (0..g[i].1).filter(|x: &usize| c_empty.get(x).is_some()).count() * (exp-1);
        g[i] = (g[i].0+add_x, g[i].1+add_y);
    }
    // Loop over pairs
    let mut sum = 0;
    for i in 0..g.len(){
        for j in i+1..g.len(){
            sum+= g[i].0.abs_diff(g[j].0) + g[i].1.abs_diff(g[j].1);
        }
    }
    return sum
}
//
fn main() {
    // Load data
    let test = get_data("data/test.txt");
    let data = get_data("data/data.txt");

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1: {:?} - 374. Completed in {:.2?}", shortest_path_sum(&test, 2), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", shortest_path_sum(&data, 2), now.elapsed());

    // Part 2
    let now = std::time::Instant::now();
    println!("Test 2.1: {:?} - 1030. Completed in {:.2?}", shortest_path_sum(&test, 10), now.elapsed());
    println!("Test 2.1: {:?} - 8410. Completed in {:.2?}", shortest_path_sum(&test, 100), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", shortest_path_sum(&data, 1000000), now.elapsed()); // 523
}