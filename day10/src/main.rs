fn get_data(path: &str) -> Vec<Vec<char>>{
    let mut data: Vec<Vec<char>> = std::fs::read_to_string(path).unwrap().lines().map(|l| format!(".{l}.")).map(|l| l.chars().collect()).collect();

    let n_y: usize = data[0].len();

    let mut output = Vec::new();
    output.push((0..n_y).map(|_c| '.').collect());
    output.append(&mut data);
    output.push((0..n_y).map(|_c| '.').collect());
    return output
}
//
fn get_connections(data: &[Vec<char>], idx: (usize, usize)) -> Vec<(usize, usize)>{
    let mut output = Vec::new();
    let is_start = data[idx.0][idx.1] == 'S';
    // Check top
    if (is_start || data[idx.0][idx.1] == '|' || data[idx.0][idx.1] == 'L' || data[idx.0][idx.1] == 'J') && 
    (data[idx.0-1][idx.1] == '|' || data[idx.0-1][idx.1] == '7' || data[idx.0-1][idx.1] == 'F' || data[idx.0-1][idx.1] == 'S'){
        output.push((idx.0-1,idx.1));
    }
    // Check left
    if (is_start || data[idx.0][idx.1] == '-' || data[idx.0][idx.1] == 'J' || data[idx.0][idx.1] == '7') && 
        (data[idx.0][idx.1-1] == '-' || data[idx.0][idx.1-1] == 'L' || data[idx.0][idx.1-1] == 'F' || data[idx.0][idx.1-1] == 'S'){
        output.push((idx.0,idx.1-1));
    }
    // Check bottom
    if (is_start || data[idx.0][idx.1] == '|' || data[idx.0][idx.1] == '7' || data[idx.0][idx.1] == 'F') && 
        (data[idx.0+1][idx.1] == '|' || data[idx.0+1][idx.1] == 'L' || data[idx.0+1][idx.1] == 'J' || data[idx.0+1][idx.1] == 'S'){
        output.push((idx.0+1,idx.1));
    }
    // Check right
    if (is_start || data[idx.0][idx.1] == '-' || data[idx.0][idx.1] == 'L' || data[idx.0][idx.1] == 'F') && 
        (data[idx.0][idx.1+1] == '-' || data[idx.0][idx.1+1] == 'J' || data[idx.0][idx.1+1] == '7' || data[idx.0][idx.1+1] == 'S'){
            output.push((idx.0,idx.1+1));
    }
    return output;
}
//
fn get_max_steps(data: &[Vec<char>]) -> usize{
    // Find start
    let s_idx: (usize, usize) = data.iter().enumerate().map(|(i, x)| {
        (i, x.iter().enumerate().filter(|(_j,c)| *c==&'S').map(|(j,_c)| j).collect::<Vec<usize>>())
    }) .filter(|(_i,x)| x.len()>0)
    .map(|(i,x)| (i,x[0])).next().unwrap();
    // Find connections to start
    let mut visited = std::collections::HashSet::new();
    visited.insert(s_idx);
    let mut conn = get_connections(data, s_idx)[0];
    while let Some(next) = get_connections(data, conn).iter().filter(|x| visited.get(x).is_none()).next(){
        visited.insert(conn);
        conn = *next;
    }
    return visited.len() / 2 + 1
}
//
fn get_enclosed_area(data: &[Vec<char>]) -> usize{
    // Find start
    let s_idx: (usize, usize) = data.iter().enumerate().map(|(i, x)| {
        (i, x.iter().enumerate().filter(|(_j,c)| *c==&'S').map(|(j,_c)| j).collect::<Vec<usize>>())
    }) .filter(|(_i,x)| x.len()>0)
    .map(|(i,x)| (i,x[0])).next().unwrap();
    // Find loop and orientation
    let mut curr = get_connections(data, s_idx)[0];
    let mut visited = Vec::new();
    visited.push(s_idx);
    visited.push(curr);
    let mut visited_set: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::from_iter(visited.iter().cloned());

    let mut rot = 0;
    while let Some(next) = get_connections(data, curr).iter().filter(|x| visited_set.get(x).is_none()).next(){
        let prev = *get_connections(data, curr).iter().filter(|x| visited_set.get(x).is_some()).next().unwrap();
        
        // Check turns
        if (prev.0 < next.0 && data[curr.0][curr.1] == 'L') ||
        (prev.0 > next.0 && data[curr.0][curr.1] == 'J') ||
        (prev.0 > next.0 && data[curr.0][curr.1] == '7') ||
        (prev.0 < next.0 && data[curr.0][curr.1] == 'F') {
            rot -= 1;
        }
        else if  data[curr.0][curr.1] != '-' && data[curr.0][curr.1] != '|'{
            rot += 1;
        }
        // Insert
        curr = *next;
        visited.push(curr);
        visited_set.insert(curr);
    }
    visited.push(s_idx);
    // Always clockwise travel; Saves trouble later
    if rot < 0{
        visited.reverse();
    }
    // Fill new map
    let mut encl = std::collections::HashSet::new();
    for n in visited.windows(2){
        if n[0].0 == n[1].0 && n[0].1 < n[1].1{ // Right
            if visited_set.get(&(n[1].0+1, n[1].1)).is_none(){
                encl.insert((n[1].0+1, n[1].1));
            }
            if visited_set.get(&(n[0].0+1, n[0].1)).is_none(){
                encl.insert((n[0].0+1, n[0].1));
            }
        }
        if  n[0].0 == n[1].0 && n[0].1 > n[1].1 {  // left
            if visited_set.get(&(n[1].0-1, n[1].1)).is_none(){
                encl.insert((n[1].0-1, n[1].1));
            }
            if visited_set.get(&(n[0].0-1, n[0].1)).is_none(){
                encl.insert((n[0].0-1, n[0].1));
            }
        }
        if n[0].1 == n[1].1 && n[0].0 > n[1].0{  // Up
            if visited_set.get(&(n[1].0,n[1].1+1)).is_none(){
                encl.insert((n[1].0,n[1].1+1));
            }
            if visited_set.get(&(n[0].0,n[0].1+1)).is_none(){ 
                encl.insert((n[0].0,n[0].1+1));
            }
        }
        if n[0].1 == n[1].1 && n[0].0 < n[1].0{ // Down
            if visited_set.get(&(n[1].0,n[1].1-1)).is_none(){
                encl.insert((n[1].0,n[1].1-1));
            }
            if visited_set.get(&(n[0].0,n[0].1-1)).is_none(){
                encl.insert((n[0].0,n[0].1-1));
            }
        }
    }
    // Extend
    let mut curr_len = 0;
    while curr_len != encl.len(){
        curr_len = encl.len();
        for n in encl.clone().iter(){
            if  visited_set.get(&(n.0-1, n.1)).is_none(){ // Up
                encl.insert((n.0-1, n.1));
            }
            if  visited_set.get(&(n.0+1, n.1)).is_none(){ // Down
                encl.insert((n.0+1, n.1));
            }
            if visited_set.get(&(n.0, n.1-1)).is_none(){ // Left
                encl.insert((n.0, n.1-1));
            }
            if visited_set.get(&(n.0, n.1+1)).is_none(){ // Right
                encl.insert((n.0, n.1+1));
            }
        }
    }
    return encl.len();
}
//
fn main() {
    // Load data
    let test = get_data("data/test.txt");
    let test2 = get_data("data/test2.txt");
    let test3 = get_data("data/test3.txt");
    let test4 = get_data("data/test4.txt");
    let test5 = get_data("data/test5.txt");
    let data = get_data("data/data.txt");

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1.1: {:?} - 4. Completed in {:.2?}", get_max_steps(&test), now.elapsed());
    println!("Test 1.2: {:?} - 8. Completed in {:.2?}", get_max_steps(&test2), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", get_max_steps(&data), now.elapsed());

    // Part 2
    let now = std::time::Instant::now();
    println!("Test 2.1: {:?} - 4. Completed in {:.2?}", get_enclosed_area(&test3), now.elapsed());
    println!("Test 2.2: {:?} - 8. Completed in {:.2?}", get_enclosed_area(&test4), now.elapsed());
    println!("Test 2.3: {:?} - 10. Completed in {:.2?}", get_enclosed_area(&test5), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", get_enclosed_area(&data), now.elapsed()); // 523
}