//
fn get_input(path: &str) -> (Vec<u64>, Vec<Vec<Vec<u64>>>) {
    // Seeds
    let seeds: Vec<u64> = std::fs::read_to_string(path).unwrap().split("\r\n\r\n").next().unwrap().split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
    // Maps
    let mut maps = Vec::new();
    for map in std::fs::read_to_string(path).unwrap().split("\r\n\r\n").skip(1){
        maps.push(map.lines().skip(1).map(|s| {
            s.split_whitespace().map(|ss| ss.parse().unwrap()).collect::<Vec<u64>>()
        }).collect::<Vec<Vec<u64>>>());
    }

    return (seeds, maps)
}
// 
fn get_lowest_location(seeds: &Vec<u64>, maps: &Vec<Vec<Vec<u64>>>) -> u64{
    let mut locations = Vec::new();
    for seed in seeds{
        let mut cur_num = seed.clone();
        'map: for map in maps{
            for mapping in map{
                if cur_num >= mapping[1] && cur_num < mapping[1] + mapping[2]{
                    cur_num = mapping[0] + (cur_num - mapping[1]);
                    continue 'map
                }
            }
        }
        locations.push(cur_num)
    }
    return locations.iter().min().unwrap().clone();
}
// Does not merge overlapping indices, but why bother?
fn get_lowest_location_from_range(seeds: &Vec<u64>, maps: &Vec<Vec<Vec<u64>>>) -> u64{
    // Initialize
    let mut idx = Vec::new();
    for i in (0..seeds.len()).step_by(2){
        idx.push([seeds[i], seeds[i+1]]);
    }
    // Map
    for map in maps{
        //println!("{:?}", idx);
        let mut new_idx = Vec::new();
        for m in map{
            let mut idx_nm = Vec::new();
            for i in &mut idx{
                if i[0] > m[1]+m[2] || i[0]+i[1] < m[1] { // Out of range
                    idx_nm.push([i[0], i[1]]);
                }
                else if i[0] >= m[1] && i[0]+i[1] <= m[1]+m[2]{ // Completely within
                    new_idx.push([m[0] + i[0] - m[1], i[1]]);
                }
                else if i[0] < m[1] && i[0]+i[1] > m[1]+m[2]{ // Bothersome overlap
                    idx_nm.push([i[0], m[1]-i[0]]);
                    new_idx.push([m[0], m[2]]);
                    idx_nm.push([m[1]+m[2], (i[0]+i[1]) - (m[1]+m[2])]);

                }
                else if i[0] < m[1] && i[0]+i[1] <= m[1]+m[2]{ // Left poke out
                    idx_nm.push([i[0], m[1]-i[0]]);
                    new_idx.push([m[0], m[2]]);
                }
                else if i[0] >= m[1] && i[0]+i[1] > m[1]+m[2]{ // Right poke out
                    new_idx.push([i[0], m[2] - (i[0]-m[1])]);
                    idx_nm.push([m[1] + m[2], (i[0]+i[1]) - (m[1]+m[2])]);
                }
            }
            idx = idx_nm;
        }
        idx.append(&mut new_idx);
    }
    return idx.iter().min_by(|x, y| x[0].cmp(&y[0])).unwrap()[0];
}
//
fn main() {
    // Load data
    let test = get_input("data/test.txt");
    let data = get_input("data/data.txt");

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1: {:?} - 35. Completed in {:.2?}", get_lowest_location(&test.0, &test.1), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", get_lowest_location(&data.0, &data.1), now.elapsed());

    // Part 2
    let now = std::time::Instant::now();
    println!("Test 2: {:?} - 46. Completed in {:.2?}", get_lowest_location_from_range(&test.0, &test.1), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", get_lowest_location_from_range(&data.0, &data.1), now.elapsed());
}