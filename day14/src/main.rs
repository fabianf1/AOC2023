fn get_data(path: &str) -> Vec<Vec<char>>{
    std::fs::read_to_string(path).unwrap().lines().map(|l| l.chars().collect()).collect()
}
//
fn calc_load(d: &Vec<Vec<char>>) -> usize{
    let mut load = 0;
    for i in 0..d.len(){
        for j in 0..d.len(){
            if d[i][j] == 'O'{
                load += d.len()-i
            }
        }
    }
    return load
}
//
fn north_load(data: &Vec<Vec<char>>) -> usize{
    // Shift north
    let mut d = data.clone();
    for i in 1..data.len(){
        for j in 0..data.len(){
            if d[i][j] == 'O'{
                let mut i_put = i;
                while i_put > 0 && d[i_put - 1][j] == '.'{
                    i_put-=1;
                }
                d[i][j] = '.';
                d[i_put][j] = 'O';
            }
        }
    }
    // Calculate load
    return calc_load(&d)
}
//
fn spin_cycle(data: &Vec<Vec<char>>) -> usize{
    let mut f_m: i32 = 0;
    let mut c_l = 0;
    let mut d = data.clone();
    let mut m = std::collections::HashMap::new();
    for n in 0..1000000000{
        m.insert(d.clone(), n);

        //let d_old = d.clone();
        // Move north
        for i in 1..data.len(){
            for j in 0..data.len(){
                if d[i][j] == 'O'{
                    let mut i_put = i;
                    while i_put > 0 && d[i_put - 1][j] == '.'{
                        i_put-=1;
                    }
                    d[i][j] = '.';
                    d[i_put][j] = 'O';
                }
            }
        }
        // Move west
        for i in 0..data.len(){
            for j in 1..data[0].len(){
                if d[i][j] == 'O'{
                    let mut j_put = j;
                    while j_put > 0 && d[i][j_put-1] == '.'{
                        j_put-=1;
                    }
                    d[i][j] = '.';
                    d[i][j_put] = 'O';
                }
            }
        }
        // Move south
        for i in (0..data.len()-1).rev(){
            for j in 0..data[0].len(){
                if d[i][j] == 'O'{
                    let mut i_put = i;
                    while i_put+1 < d.len() && d[i_put + 1][j] == '.'{
                        i_put+=1;
                    }
                    d[i][j] = '.';
                    d[i_put][j] = 'O';
                }
            }
        }
        // Move east
        for i in 0..data.len(){
            for j in (0..data[0].len()-1).rev(){
                if d[i][j] == 'O'{
                    let mut j_put = j;
                    while j_put+1 < d[0].len() && d[i][j_put+1] == '.'{
                        j_put+=1;
                    }
                    d[i][j] = '.';
                    d[i][j_put] = 'O';
                }
            }
        }
        // Check
        if m.get(&d.clone()).is_some(){
            // Set first match
            if f_m == 0{
                f_m = n;
                c_l = 1 + n - m.get(&d.clone()).unwrap();
                break;
            }
        }
    }
    // Calc load after 1000000000
    let rem = (1000000000 - f_m) % c_l;
    for (k,v) in m.iter(){
        if (*v) == rem + (f_m - c_l){
            return calc_load(k);
        }
    }
    panic!();
}
//
fn main() {
    // Load data
    let test = get_data("data/test.txt");
    //println!("{:?}", test);
    let data = get_data("data/data.txt");

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1: {:?} - 136. Completed in {:.2?}", north_load(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", north_load(&data), now.elapsed());

    // Part 2
    let now = std::time::Instant::now();
    println!("Test 2: {:?} - 64. Completed in {:.2?}", spin_cycle(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", spin_cycle(&data), now.elapsed()); // 199972522864447 too low
}