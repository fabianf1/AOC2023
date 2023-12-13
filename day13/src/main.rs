fn get_data(path: &str) -> Vec<Vec<String>>{
    let mut output = Vec::new();
    for pat in std::fs::read_to_string(path).unwrap().split("\r\n\r\n"){
        output.push(pat.lines().map(|s| s.to_owned()).collect());
    }
    return output
}
//
fn find_reflections(data: &Vec<Vec<String>>) -> usize{
    let mut sum = 0;
    for pat in data{
        // Check horizontal match
        'outer: for i in 0..pat.len()-1{
            for j in 0..=i.min(pat.len()-i-2){
                if pat[i-j] != pat[i+1+j]{
                    continue 'outer
                }
            }
            sum += 100 * (i+1);
            break
        }
        // Vertical match
        'outer: for i in 0..pat[0].len()-1{
            for j in 0..=i.min(pat[0].len()-i-2){
                for k in 0..pat.len(){
                    if pat[k][i-j..=i-j] != pat[k][i+1+j..=i+1+j]{
                        continue 'outer
                    }
                }
            }
            sum += i+1;
            break
        }
    }
    return sum
}
//
fn find_reflections_after_fix(data: &Vec<Vec<String>>) -> usize{
    let mut sum = 0;
    'pat: for pat in data{
        // Check horizontal match
        'outer: for i in 0..pat.len()-1{
            let mut smudge = false;
            for j in 0..=i.min(pat.len()-i-2){
                if pat[i-j] != pat[i+1+j]{
                    if smudge{
                        continue 'outer
                    }
                    else{
                        let mut num = 0;
                        for (a,b) in std::iter::zip(pat[i-j].chars(), pat[i+1+j].chars()){
                            if a!=b{
                                num+=1;
                            }
                        }
                        if num==1{
                            smudge = true;
                        }
                        else{
                            continue 'outer
                        }
                    }
                }
            }
            if smudge{
                sum += 100 * (i+1);
                continue 'pat;
            }
        }
        // Vertical match
        'outer: for i in 0..pat[0].len()-1{
            let mut smudge = false;
            for j in 0..=i.min(pat[0].len()-i-2){
                for k in 0..pat.len(){
                    let mut smudge_tent = false;
                    if pat[k][i-j..=i-j] != pat[k][i+1+j..=i+1+j]{
                        if smudge || smudge_tent{
                            continue 'outer
                        }
                        else{
                            smudge_tent = true;
                        }
                    }
                    if smudge_tent{
                        smudge = true;
                    }
                }
            }
            if smudge{
                sum += i+1;
                break
            }
        }
    }
    return sum
}
//
fn main() {
    // Load data
    let test = get_data("data/test.txt");
    let test2 = get_data("data/test2.txt");
    let data = get_data("data/data.txt");

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1: {:?} - 405. Completed in {:.2?}", find_reflections(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", find_reflections(&data), now.elapsed());

    // Part 2
    let now = std::time::Instant::now();
    println!("Test 2.1: {:?} - 400. Completed in {:.2?}", find_reflections_after_fix(&test), now.elapsed());
    println!("Test 2.1: {:?} - 6. Completed in {:.2?}", find_reflections_after_fix(&test2), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", find_reflections_after_fix(&data), now.elapsed());
}