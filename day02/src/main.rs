fn get_input(path: &str) -> Vec<Vec<[usize;3]>>{
    // Data: Game -> Set -> [r,g,b];
    let mut data:Vec<Vec<[usize;3]>> = Vec::new();
    for line in std::fs::read_to_string(path).unwrap().lines(){
        let mut game:Vec<[usize;3]> = Vec::new();
        let sets =  line.split(": ").skip(1).next().unwrap().split("; ");
        for set in sets{
            let mut balls:[usize;3] = [0;3];
            for ball in set.split(", "){
                let mut str_split = ball.split(" ");
                let num = str_split.next().unwrap().parse().unwrap();
                match str_split.next().unwrap(){
                    "red" => balls[0] = num,
                    "green" => balls[1] = num,
                    "blue" => balls[2] = num,
                    _ => panic!(),
                }
            }
            game.push(balls);
        }
        data.push(game);
    }
    return data
}
//
fn possible_game_sum(data: &Vec<Vec<[usize;3]>>, bag: [usize;3]) -> usize{
    let mut sum = 0;
    'outer: for (i,game) in data.iter().enumerate(){
        for set in game{
            if set[0] > bag[0] || set[1] > bag[1] || set[2] > bag[2]{
                continue 'outer;
            };
        }
        sum += i+1;
    }
    return sum
}
//
fn get_power_sum(data: &Vec<Vec<[usize;3]>>) -> usize{
    let mut sum = 0;
    for game in data{
        let mut balls = [0;3];
        for set in game{
            balls[0] = std::cmp::max(balls[0], set[0]);
            balls[1] = std::cmp::max(balls[1], set[1]);
            balls[2] = std::cmp::max(balls[2], set[2]);
        }
        sum += balls[0]*balls[1]*balls[2];
    }
    return sum
}
//
fn main() {
    // Load data
    let test = get_input("data/test.txt");
    let data = get_input("data/data.txt");

    // Part 1
    let now = std::time::Instant::now();
    println!("Test 1: {:?} - 8. Completed in {:.2?}", possible_game_sum(&test, [12,13,14]), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 1: {:?}. Completed in {:.2?}", possible_game_sum(&data, [12,13,14]), now.elapsed());

    // // Part 2
    let now = std::time::Instant::now();
    println!("Test 2: {:?} - 2286. Completed in {:.2?}", get_power_sum(&test), now.elapsed());
    let now = std::time::Instant::now();
    println!("Part 2: {:?}. Completed in {:.2?}", get_power_sum(&data), now.elapsed());
}