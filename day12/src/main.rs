fn get_data(path: &str) -> Vec<(String, Vec<usize>)>{
    let mut output = Vec::new();
    for line in std::fs::read_to_string(path).unwrap().lines(){
        let mut split = line.split_whitespace();
        output.push((split.next().unwrap().to_owned(), split.last().unwrap().split(",").map(|c| c.parse().unwrap()).collect()));
    }
    return output
}
// Rewrite to non-recursive and add cache
fn check_valid(s: &Vec<String>, n: &[usize], cache: &mut std::collections::HashMap<(Vec<String>, Vec<usize>), u64>) -> u64{
    if let Some(m) = cache.get(&(s.clone(), n.to_vec())){
        return *m
    }
    //
    if s.len() == 0 && n.len() > 0{
        cache.insert((s.clone(), n.to_vec()), 0);
        return 0
    }
    else if n.len() == 0{
        if s.iter().filter(|s| s.contains('#')).count() == 0{
            cache.insert((s.clone(),n.to_vec()), 1);
            return 1
        }
        else{
            cache.insert((s.clone(),n.to_vec()), 0);
            return 0
        }
    }
    else if let Some(idx) = s.last().unwrap().rfind(|c| c == '?'){
        // Change to dot
        let mut s2 = s.clone();
        if idx == 0{
            if s2.last().unwrap().len() == 1{
                s2.pop();
            }
            else{
                s2[s.len()-1] = s2[s.len()-1][idx+1..].to_owned();
            }
        }
        else if idx == s2[s.len()-1].len()-1{
            s2[s.len()-1] = s2[s.len()-1][..idx].to_owned();
        }
        else{
            s2[s.len()-1] = s2[s.len()-1][..idx].to_owned();
            s2.push(s[s.len()-1][idx+1..].to_owned());
        }

        // Change to #
        let mut s3 = s.clone();
        s3[s.len()-1].replace_range(idx..=idx, "#");
        
        //
        let res = check_valid(&s2, n, cache) + check_valid(&s3, n, cache);
        cache.insert((s.clone(), n.to_vec()), res);
        return res
    }
    else{
        if s.last().unwrap().len() != *n.last().unwrap(){
            cache.insert((s.clone(), n.to_vec()), 0);
            return 0
        }
        else if s.len() == 1 && n.len() == 1{
            cache.insert((s.clone(), n.to_vec()), 1);
            return 1
        }
        //
        let res = check_valid(&s[..s.len()-1].to_vec(), &n[..n.len()-1], cache);
        cache.insert((s.clone(), n.to_vec()), res);
        return res
    }

}
//
fn split_springs(data: &String) -> Vec<String>{
    return data.split('.').filter(|s| s.len()>0).map(|s| s.to_owned()).collect()
}
//
fn arrangements_sum(data: &[(String, Vec<usize>)]) -> u64{
    let mut cache: std::collections::HashMap<(Vec<String>, Vec<usize>), u64> = std::collections::HashMap::new();

    let mut sum = 0;
    for (spr, num) in data{
        sum += check_valid(&split_springs(spr), num, &mut cache);
        cache.clear(); // Apparantly halves total time spend
    }
    return sum
}
//
fn arrangements_sum_unfolded(data: &Vec<(String, Vec<usize>)>) -> u64{
    let mut unfolded = data.clone();
    for i in 0..data.len(){
        for _j in 0..4{
            unfolded[i].0.push('?');
            unfolded[i].0 = unfolded[i].0.clone() + &mut data[i].0.clone();
            unfolded[i].1.append(&mut data[i].1.clone());
        }
    }
    return arrangements_sum(&unfolded)
}
//
fn main() {
    // Load data
    let test = get_data("data/test.txt");
    let data = get_data("data/data.txt");

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1: {:?} - 21. Completed in {:.2?}", arrangements_sum(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", arrangements_sum(&data), now.elapsed());

    // Part 2
    let now = std::time::Instant::now();
    println!("Test 2: {:?} - 525152. Completed in {:.2?}", arrangements_sum_unfolded(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", arrangements_sum_unfolded(&data), now.elapsed());
}