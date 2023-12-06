use std::{fs, num::ParseIntError};

enum phase {
    checking,
    parsing,
}

fn main() {
    let file_path = "res/input";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let words:Vec<Vec<&str>> = lines.map(|line| line.split(' ').collect()).collect();
    let mut words2 = words.clone();

    let mut total_points = 0;
    let mut points_per_card = 0;
    let mut phase:phase = phase::parsing;
    let mut winning_numbers:Vec<i32> = vec![];
    for line in words {
        for word in line {
            match word.parse::<i32>() {
                Ok(x) => {
                    match phase {
                        phase::checking => {
                            if winning_numbers.contains(&x) {
                                if points_per_card == 0 {
                                    points_per_card = 1;
                                } else {
                                    points_per_card *= 2
                                }
                            }
                        },
                        phase::parsing => {
                            winning_numbers.push(x);
                        }                       
                    }
                },
                Err(x) => {
                    if word == "|" {
                        phase = phase::checking;
                    }
                }
            }
        }
        total_points += points_per_card;
        points_per_card = 0;
        winning_numbers.clear();
        phase = phase::parsing;
    }

    println!("{:?}",total_points);

    let mut points_per_card = 0;
    let mut phase:phase = phase::parsing;
    let mut winning_numbers:Vec<i32> = vec![];
    let mut i = 0;
    while (i < words2.len()) {
        for word in words2[i].clone() {
            match word.parse::<i32>() {
                Ok(x) => {
                    match phase {
                        phase::checking => {
                            if winning_numbers.contains(&x) {
                                points_per_card += 1;
                                match words2[i][1].replace(":", "").parse::<i32>() {
                                    Ok(x) => {
                                        words2.push(words2[(x + points_per_card - 1) as usize].clone());
                                    },
                                    Err(x) => { 
                                        match words2[i][2].replace(":", "").parse::<i32>() {
                                            Ok(x) => {
                                                words2.push(words2[(x + points_per_card - 1) as usize].clone());
                                            },
                                            Err(x) => {
                                                let index = words2[i][3].replace(":", "").parse::<i32>().unwrap();
                                                words2.push(words2[(index + points_per_card - 1) as usize].clone());
                                            }
                                        }
                                    }
                                }
                                
                            }
                        },
                        phase::parsing => {
                            winning_numbers.push(x);
                        }                       
                    }
                },
                Err(x) => {
                    if word == "|" {
                        phase = phase::checking;
                    }
                }
            }
        }
        total_points += points_per_card;
        points_per_card = 0;
        winning_numbers.clear();
        phase = phase::parsing;
        i += 1;
    }
    //println!("{:?}", words2);
    println!("{:?}", words2.len())
}
