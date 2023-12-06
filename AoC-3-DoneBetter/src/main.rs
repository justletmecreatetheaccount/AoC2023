use std::fs;
use std::collections::HashMap;

struct machine_component {
    component_type: char,
    serials: Vec<u32>,
}

fn main() {
    let file_path = "res/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let chars = lines.map(|line| line.chars().collect::<Vec<char>>());
    let chars_array = chars.collect::<Vec<Vec<char>>>();
    //println!("{:?}", chars_array);

    let mut all_components:HashMap<(usize, usize), machine_component> = HashMap::new();

    let mut sum = 0;
    let mut temp = 0;
    let mut hits:Vec<(usize, usize)> = Vec::new();

    for i in 0..chars_array.len() {
        for j in 0..chars_array[i].len() {
            //check if it is a number 
            match chars_array[i][j].to_digit(10) {
                Some(x) => {
                    //construct the number
                    temp = temp * 10 + x;

                    //check above
                    let t: i64 = i as i64;
                    match chars_array.get((t - 1) as usize) {
                        Some(line) => {
                            //check if it is a machine component
                            if line[j] != '.' && !line[j].is_numeric() {
                                //check if component is known about / is in the HashMap
                                if !all_components.contains_key(&(i - 1, j)) {
                                    all_components.insert((i - 1, j), machine_component { component_type: line[j], serials: vec![]});
                                }
                                //adds coorordinates to the list of machine components that are in the vicinity of the number
                                hits.push((i - 1, j));
                            }
                        },
                        None => { } 
                    }

                    //check underneath
                    match chars_array.get(i + 1) {
                        Some(line) => {
                            //check if it is a machine component
                            if line[j] != '.' && !line[j].is_numeric() {
                                //check if component is known about / is in the HashMap
                                if !all_components.contains_key(&(i + 1, j)) {
                                    all_components.insert((i + 1, j), machine_component { component_type: line[j], serials: vec![]});
                                }
                                //adds coorordinates to the list of machine components that are in the vicinity of the number
                                hits.push((i + 1, j));
                            }
                        },
                        None => { } 
                    }
                },
                None => {
                    //check if a number was in construction
                    if temp > 0 {
                        //check current coordinates
                        if chars_array[i][j] != '.' && !chars_array[i][j].is_numeric() {
                            //check if component is known about / is in the HashMap
                            if !all_components.contains_key(&(i, j)) {
                                all_components.insert((i, j), machine_component { component_type: chars_array[i][j], serials: vec![]});
                            }
                            //adds coorordinates to the list of machine components that are in the vicinity of the number
                            hits.push((i, j));
                        }

                        //check above
                        let t: i64 = i as i64;
                        match chars_array.get((t - 1) as usize) {
                            Some(line) => {
                                //check if it is a machine component
                                if line[j] != '.' && !line[j].is_numeric() {
                                    //check if component is known about / is in the HashMap
                                    if !all_components.contains_key(&(i - 1, j)) {
                                        all_components.insert((i - 1, j), machine_component { component_type: line[j], serials: vec![]});
                                    }
                                    //adds coorordinates to the list of machine components that are in the vicinity of the number
                                    hits.push((i - 1, j));
                                }
                            },
                            None => { } 
                        }

                        //check underneath
                        match chars_array.get(i + 1) {
                            Some(line) => {
                                //check if it is a machine component
                                if line[j] != '.' && !line[j].is_numeric() {
                                    //check if component is known about / is in the HashMap
                                    if !all_components.contains_key(&(i + 1, j)) {
                                        all_components.insert((i + 1, j), machine_component { component_type: line[j], serials: vec![]});
                                    }
                                    //adds coorordinates to the list of machine components that are in the vicinity of the number
                                    hits.push((i + 1, j));
                                }
                            },
                            None => { } 
                        }

                        //for part 1 only check if number was close to any char
                        if !hits.is_empty() {
                            sum += temp;
                        }

                        //add number to all machine components
                        for hit in &hits {
                            match all_components.get_mut(hit) {
                                Some(x) => {
                                    x.serials.push(temp);
                                },
                                None => { print!("Error at number {:?}, char at position {:?} was not added", temp, hit)}
                            }
                        }
                        temp = 0;
                        hits.clear();

                        //chexk again for the next number might need it
                        //check current coordinates
                        if chars_array[i][j] != '.' && !chars_array[i][j].is_numeric() {
                            //check if component is known about / is in the HashMap
                            if !all_components.contains_key(&(i, j)) {
                                all_components.insert((i, j), machine_component { component_type: chars_array[i][j], serials: vec![]});
                            }
                            //adds coorordinates to the list of machine components that are in the vicinity of the number
                            hits.push((i, j));
                        }

                        //check above
                        let t: i64 = i as i64;
                        match chars_array.get((t - 1) as usize) {
                            Some(line) => {
                                //check if it is a machine component
                                if line[j] != '.' && !line[j].is_numeric() {
                                    //check if component is known about / is in the HashMap
                                    if !all_components.contains_key(&(i - 1, j)) {
                                        all_components.insert((i - 1, j), machine_component { component_type: line[j], serials: vec![]});
                                    }
                                    //adds coorordinates to the list of machine components that are in the vicinity of the number
                                    hits.push((i - 1, j));
                                }
                            },
                            None => { } 
                        }

                        //check underneath
                        match chars_array.get(i + 1) {
                            Some(line) => {
                                //check if it is a machine component
                                if line[j] != '.' && !line[j].is_numeric() {
                                    //check if component is known about / is in the HashMap
                                    if !all_components.contains_key(&(i + 1, j)) {
                                        all_components.insert((i + 1, j), machine_component { component_type: line[j], serials: vec![]});
                                    }
                                    //adds coorordinates to the list of machine components that are in the vicinity of the number
                                    hits.push((i + 1, j));
                                }
                            },
                            None => { } 
                        }

                    } else {
                        hits.clear();
                        //check current coordinates
                        if chars_array[i][j] != '.' && !chars_array[i][j].is_numeric() {
                            //check if component is known about / is in the HashMap
                            if !all_components.contains_key(&(i, j)) {
                                all_components.insert((i, j), machine_component { component_type: chars_array[i][j], serials: vec![]});
                            }
                            //adds coorordinates to the list of machine components that are in the vicinity of the number
                            hits.push((i, j));
                        }

                        //check above
                        let t: i64 = i as i64;
                        match chars_array.get( (t - 1) as usize ) {
                            Some(line) => {
                                //check if it is a machine component
                                if line[j] != '.' && !line[j].is_numeric() {
                                    //check if component is known about / is in the HashMap
                                    if !all_components.contains_key(&(i - 1, j)) {
                                        all_components.insert((i - 1, j), machine_component { component_type: line[j], serials: vec![]});
                                    }
                                    //adds coorordinates to the list of machine components that are in the vicinity of the number
                                    hits.push((i - 1, j));
                                }
                            },
                            None => { } 
                        }

                        //check underneath
                        match chars_array.get(i + 1) {
                            Some(line) => {
                                //check if it is a machine component
                                if line[j] != '.' && !line[j].is_numeric() {
                                    //check if component is known about / is in the HashMap
                                    if !all_components.contains_key(&(i + 1, j)) {
                                        all_components.insert((i + 1, j), machine_component { component_type: line[j], serials: vec![]});
                                    }
                                    //adds coorordinates to the list of machine components that are in the vicinity of the number
                                    hits.push((i + 1, j));
                                }
                            },
                            None => { } 
                        }
                    }
                 }
            }
        }
        //for part 1 only check if number was close to any char
        if !hits.is_empty() {
            sum += temp;
        }

        //add number to all machine components
        for hit in &hits {
            match all_components.get_mut(hit) {
                Some(x) => {
                    x.serials.push(temp);
                },
                None => { print!("Error at number {:?}, char at position {:?} was not added", temp, hit)}
            }
        }
        temp = 0;
        hits.clear();
    }

    println!("Sum {}", sum);
    sum = 0;

    for component in all_components {
        if component.1.serials.len() == 2 && component.1.component_type == '*' {
            sum += component.1.serials[0] * component.1.serials[1];
        }
    }

    println!("Sum {}", sum);
}
