use std::fs;

fn main() {
    let file_path = "res/input";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut sum = 0;
    let mut first_digit = 0;
    let mut second_digit = 0;
    for line in lines {
        for c in line.chars() {
            if let Option::Some(result) = c.to_digit(10) {
                first_digit = result;
                break;
            }
        }
        for c in line.chars().rev() {
            if let Option::Some(result) = c.to_digit(10) {
                second_digit = result;
                break;
            }
        }
    sum += first_digit * 10 + second_digit;
    }
    println!("Sum: {}", sum);

    let lines = contents.split("\n");
    let mut sum = 0;
    let mut first_digit = 0;
    let mut second_digit = 0;
    let mut line_number = 0;
    for line in lines {
        let line_array:Vec<char> = line.chars().collect();
        //let reverse_line_array:Vec<char> = line.chars().rev().collect();
        let reverse_line_array:Vec<char> = line_array.clone().into_iter().rev().collect();
        'left_to_right:
        for i in 0..line.len() { 
            if let Option::Some(result) = line_array[i].to_digit(10) {
                first_digit = result;
                break;
            } else {
                match line_array[i] {
                    'o' => if line.len() - i > 3 && String::from_iter(&line_array[i+1..i+3]) == "ne" {
                        first_digit = 1;
                        break 'left_to_right;
                    },
                    't' => if line.len() - i > 3 && String::from_iter(&line_array[i+1..i+3]) == "wo" {
                        first_digit = 2;
                        break 'left_to_right;
                    } else if line.len() - i > 5 && String::from_iter(&line_array[i+1..i+5]) == "hree" {
                        first_digit = 3;
                        break 'left_to_right;
                    },
                    'f' => if line.len() - i > 4 && String::from_iter(&line_array[i+1..i+4]) == "our" {
                        first_digit = 4;
                        break 'left_to_right;
                    } else if line.len() - i > 4 && String::from_iter(&line_array[i+1..i+4]) == "ive" {
                        first_digit = 5;
                        break 'left_to_right;
                    },
                    's' => if line.len() - i > 3 && String::from_iter(&line_array[i+1..i+3]) == "ix" {
                        first_digit = 6;
                        break 'left_to_right;
                    } else if line.len() - i > 5 && String::from_iter(&line_array[i+1..i+5]) == "even" {
                        first_digit = 7;
                        break 'left_to_right;
                    },
                    'e' => if line.len() - i > 5 && String::from_iter(&line_array[i+1..i+5]) == "ight" {
                        first_digit = 8;
                        break 'left_to_right;
                    },
                    'n' => if line.len() - i > 4 && String::from_iter(&line_array[i+1..i+4]) == "ine" {
                        first_digit = 9;
                        break 'left_to_right;
                    },
                    _ => continue,
                }
                    
                }
            }
        'right_to_left:
        for i in 0..line.len() { 
            if let Option::Some(result) = reverse_line_array[i].to_digit(10) {
                second_digit = result;
                break;
            } else {

                match reverse_line_array[i] {
                    'e' => if line.len() - i > 3 && String::from_iter(&reverse_line_array[i+1..i+3]) == "no" {
                        second_digit = 1;
                        break 'right_to_left;
                    } else if line.len() - i > 5 && String::from_iter(&reverse_line_array[i+1..i+5]) == "erht" {
                        second_digit = 3;
                        break 'right_to_left;
                    } else if line.len() - i > 4 && String::from_iter(&reverse_line_array[i+1..i+4]) == "vif" {
                        second_digit = 5;
                        break 'right_to_left;
                    } else if line.len() - i > 4 && String::from_iter(&reverse_line_array[i+1..i+4]) == "nin" {
                        second_digit = 9;
                        break 'right_to_left;
                    },
                    'o' => if line.len() - i > 3 && String::from_iter(&reverse_line_array[i+1..i+3]) == "wt" {
                        second_digit = 2;
                        break 'right_to_left;
                    },
                    'r' => if line.len() - i > 4 && String::from_iter(&reverse_line_array[i+1..i+4]) == "uof" {
                        second_digit = 4;
                        break 'right_to_left;
                    },
                    'x' => if line.len() - i > 3 && String::from_iter(&reverse_line_array[i+1..i+3]) == "is" {
                        second_digit = 6;
                        break 'right_to_left;
                    }, 
                    'n' => if line.len() - i > 5 && String::from_iter(&reverse_line_array[i+1..i+5]) == "eves" {
                        second_digit = 7;
                        break 'right_to_left;
                    },
                    't' => if line.len() - i > 5 && String::from_iter(&reverse_line_array[i+1..i+5]) == "hgie" {
                        second_digit = 8;
                        break 'right_to_left;
                    },
                    _ => continue,
                }
            }
        }
    line_number += 1;
    println!("{} -> {}", line_number, first_digit * 10 + second_digit);
    sum += first_digit * 10 + second_digit;
    }
    println!("Sum: {}", sum);
}