use std::fs;

fn main() {
    let file_path = "res/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    //let lines = lines.collect::<Vec<&str>>();
    let chars = lines.map(|line| line.chars().collect::<Vec<char>>());
    let chars_array = chars.collect::<Vec<Vec<char>>>();
    let mut sum = 0;
    let mut temp = 0;
    let mut next_to = false;
    for i in 0..chars_array.len() {
        for j in 0..chars_array[i].len() {
            match chars_array[i][j].to_digit(10) {
                Some(x) => {
                    if temp == 0 {


                        temp += x as i32;

                        //Chexk up and diagonal
                        let t = i as i64 - 1;
                        println!("{:?} with i worth {:?}", t as usize, i);
                        match chars_array.get(t as usize) {
                            Some(_line) => {
                                println!("{:?} is checked while on number {:?} and total {:?}", chars_array[i - 1][j], chars_array[i][j], temp);
                                if chars_array[i - 1][j] != '.' && !chars_array[i - 1][j].is_numeric() {
                                    next_to = true;
                                }
                                let t = j as i64 - 1;
                                match chars_array[i - 1].get(t as usize) {
                                    Some(_char) => {
                                        println!("{:?} is checked while on number {:?} and total {:?}", chars_array[i - 1][j - 1], chars_array[i][j], temp);
                                        if chars_array[i - 1][j - 1] != '.' && !chars_array[i - 1][j - 1].is_numeric() {
                                            next_to = true;
                                        }
                                    },
                                    None => {
                                        println!("number {:?} is on the left edge", temp);
                                    },
                                }
                            },
                            None => {
                                println!("number {:?} is on the top edge", temp);
                            },
                        }
                        //Check down and diagonal
                        match chars_array.get(i + 1) {
                            Some(_line) => {
                                println!("{:?} is checked while on number {:?} and total {:?}", chars_array[i + 1][j], chars_array[i][j], temp);
                                if chars_array[i + 1][j] != '.' && !chars_array[i + 1][j].is_numeric() {
                                    next_to = true;
                                }
                                let t = j as i64 - 1;
                                match chars_array[i + 1].get(t as usize) {
                                    Some(_char) => {
                                        println!("{:?} is checked while on number {:?} and total {:?}", chars_array[i + 1][j - 1], chars_array[i][j], temp);
                                        if chars_array[i + 1][j - 1] != '.' && !chars_array[i + 1][j - 1].is_numeric() {
                                            next_to = true;
                                        }
                                    },
                                    None => {
                                    },
                                }
                            },
                            None => {
                            },
                        }

                        //Check left (actually right)
                        let t = j as i64 - 1;
                        match chars_array[i].get(t as usize) {
                            Some(_char) => {
                                println!("{:?} is checked while on number {:?} and total {:?}", chars_array[i][j - 1], chars_array[i][j], temp);
                                if chars_array[i][j - 1] != '.' && !chars_array[i][j - 1].is_numeric() {
                                    next_to = true;
                                }
                            },
                            None => {
                            },
                        }


                    } else {


                        temp *= 10;
                        temp += x as i32;

                        //Chexk up
                        let t = i as i64 - 1;
                        match chars_array.get(t as usize) {
                            Some(_line) => {
                                println!("{:?} is checked while on number {:?} and total {:?}", chars_array[i - 1][j], chars_array[i][j], temp);
                                if chars_array[i - 1][j] != '.' && !chars_array[i - 1][j].is_numeric() {
                                    next_to = true;
                                }
                            },
                            None => {
                            },
                        }
                        //Check down
                        match chars_array.get(i + 1) {
                            Some(_line) => {
                                println!("{:?} is checked while on number {:?} and total {:?}", chars_array[i + 1][j], chars_array[i][j], temp);
                                if chars_array[i + 1][j] != '.' && !chars_array[i + 1][j].is_numeric() {
                                    next_to = true;
                                }
                            },
                            None => {
                            },
                        }


                    }
                },
                None => {
                    if temp > 0 {
                        //Chexk up
                        let t = i as i64 - 1;
                        match chars_array.get(t as usize) {
                            Some(_line) => {
                                println!("{:?} is checked while on number {:?} and total {:?}", chars_array[i - 1][j], chars_array[i][j], temp);
                                if chars_array[i - 1][j] != '.' && !chars_array[i - 1][j].is_numeric() {
                                    next_to = true;
                                }
                            },
                            None => {
                            },
                        }
                        //Check down
                        match chars_array.get(i + 1) {
                            Some(_line) => {
                                println!("{:?} is checked while on number {:?} and total {:?}", chars_array[i + 1][j], chars_array[i][j], temp);
                                if chars_array[i + 1][j] != '.' && !chars_array[i + 1][j].is_numeric() {
                                    next_to = true;
                                }
                            },
                            None => {
                            },
                        }
                        //Check middle
                        println!("{:?} is checked while on number {:?} and total {:?}", chars_array[i][j], chars_array[i][j], temp);
                        if chars_array[i][j] != '.' && !chars_array[i][j].is_numeric() {
                            next_to = true;
                        }

                        if next_to {
                            println!("{} is next to a symbol", temp);
                            sum += temp;
                            temp = 0;
                            next_to = false;
                        } else {
                            println!("{} is not next to a symbol", temp);
                            temp = 0;
                        }
                    }
                }
            }
        }
    }
    println!("Sum : {}", sum);

    
}