use std::fs;

fn main() {
    let file_path = "res/input";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let init_lines = contents.split("\n");
    let lines = init_lines.map(|line| line.replace("Game", ""))
                                                        .map(|line| line.replace(":", ""))
                                                        .map(|line| line.replace(",", ""))
                                                        .map(|line| line.replace("\r", ""));
    //let lines:Vec<Vec<&str>> = lines.map(|line| line.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let lines = lines.collect::<Vec<String>>();
    let lines = lines.iter().map(|line| line.split(" ").collect::<Vec<&str>>());
    let lines:Vec<Vec<&str>> = lines.collect::<Vec<Vec<&str>>>();
    let lines2 = lines.clone();

    const RED:i32 = 12;
    const GREEN:i32 = 13;
    const BLUE:i32 = 14;
    let mut sum = 0;
    
    'games:for line in lines {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for i in 3..line.len() {
            match (line[i]) {
                "red" => r += line[i-1].parse::<i32>().unwrap(),
                "green" => g += line[i-1].parse::<i32>().unwrap(),
                "blue" => b += line[i-1].parse::<i32>().unwrap(),
                "red;" => {
                    r += line[i-1].parse::<i32>().unwrap();
                    if (r <= RED && g <= GREEN && b <= BLUE) {
                        println!("Game {} is valid", line[1]);
                    } else {
                        println!("Game {} is invalid", line[1]);
                        continue 'games;
                    }
                    r = 0;
                    g = 0;
                    b = 0
                },
                "green;" => {
                    g += line[i-1].parse::<i32>().unwrap();
                    if (r <= RED && g <= GREEN && b <= BLUE) {
                        println!("Game {} is valid", line[1]);
                    } else {
                        println!("Game {} is invalid", line[1]);
                        continue 'games;
                    }
                    r = 0;
                    g = 0;
                    b = 0
                },
                    "blue;" => {
                        b += line[i-1].parse::<i32>().unwrap();
                    if (r <= RED && g <= GREEN && b <= BLUE) {
                        println!("Game {} is valid", line[1]);
                    } else {
                        println!("Game {} is invalid", line[1]);
                        continue 'games;
                   }
                    r = 0;
                    g = 0;
                    b = 0
                },
                _ => continue,
            }
        }
        if (r <= RED && g <= GREEN && b <= BLUE) {
            println!("Game {} is valid", line[1]);
           sum += line[1].parse::<i32>().unwrap();
        } else {
            println!("Game {} is invalid", line[1]);
       }
        r = 0;
        g = 0;
        b = 0;
        println!("SUM : {}", sum)
    }
    println!("Sum of valid games: {}", sum);

    let mut sum = 0;
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for line in lines2 {
        println!("{:?}", line);
        for i in 3..line.len() {
            match (line[i]) {
                "red" => if (line[i-1].parse::<i32>().unwrap() > r) { r = line[i-1].parse::<i32>().unwrap() },
                "green" => if (line[i-1].parse::<i32>().unwrap() > g) { g = line[i-1].parse::<i32>().unwrap() },
                "blue" => if (line[i-1].parse::<i32>().unwrap() > b) { b = line[i-1].parse::<i32>().unwrap() },
                "red;" => if (line[i-1].parse::<i32>().unwrap() > r) { r = line[i-1].parse::<i32>().unwrap() },
                "green;" => if (line[i-1].parse::<i32>().unwrap() > g) { g = line[i-1].parse::<i32>().unwrap() },
                "blue;" => if (line[i-1].parse::<i32>().unwrap() > b) { b = line[i-1].parse::<i32>().unwrap() },
                _ => continue,
            }
        println!("{} {} {}", r, g, b);
        }
        sum += r * g * b;
        r = 0;
        g = 0;
        b = 0;
    }
    println!("Sum of power of games: {}", sum);
}