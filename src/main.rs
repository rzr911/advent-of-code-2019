use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    first_problem_first_part();
    first_problem_second_part();
    second_problem_first_part();
    second_problem_second_part();
}


fn first_problem_first_part() -> io::Result<()> {
    let mut sum_of_fuel_requirements = 0;

    let file = File::open("./data/1.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
       
        let i = line?;
        let mass: u32 = i.parse::<u32>().unwrap();
        let individual_fuel_required = mass/3 -2;
        sum_of_fuel_requirements += individual_fuel_required;

    }
    println!("Question 1a {}", sum_of_fuel_requirements);
    Ok(())
}

fn first_problem_second_part() -> io::Result<()> {
    let mut sum_of_fuel_requirements = 0;

    let file = File::open("./data/1.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
       
        let i = line?;
        let mass: i32 = i.parse::<i32>().unwrap();
        let mut individual_fuel_required = mass/3 - 2;
        sum_of_fuel_requirements += individual_fuel_required;
        
        let mut done = false;

        while !done {

            if individual_fuel_required < 3  {
                done = true;
            }
            individual_fuel_required = individual_fuel_required/3-2;
            
            if individual_fuel_required > 0 {
            sum_of_fuel_requirements += individual_fuel_required;
            }
        }
        

    }
    println!("Question 1b {}", sum_of_fuel_requirements);
    Ok(())
}


fn second_problem_first_part() -> io::Result<()> {
    let file = File::open("./data/2.txt")?;
    let reader = BufReader::new(file);
    let mut file_string = "".to_string();
    for line in reader.lines() {
       
        let i = line?;
        file_string.push_str(&i);

    }

    let mut int_code_string : Vec<i32> = file_string.split(",").map(|s| s.parse().unwrap()).collect();
    int_code_string[1] = 12;
    int_code_string[2] = 2;
    
    
    for i in (0..int_code_string.len()).step_by(4) {
        
        let mut number = int_code_string[i];
        
        println!("{:?}",int_code_string);
        // println!("{}",input_1);
        // println!("{}",input_2);
        
        if number == 1 as i32 {
            let input_1 = int_code_string[int_code_string[i + 1] as usize];
            let input_2 = int_code_string[int_code_string[i + 2] as usize];
            let mut output_position = int_code_string[i + 3];
            let mut final_value : i32;
            final_value = input_1 + input_2;

            // println!("value {} output position {}",final_value, output_position);
        
            int_code_string[output_position as usize] = final_value;
        } else if number == 2 as i32 {
            let input_1 = int_code_string[int_code_string[i + 1] as usize];
            let input_2 = int_code_string[int_code_string[i + 2] as usize];
            let mut output_position = int_code_string[i + 3];
            let mut final_value : i32;
            final_value = input_1 * input_2;
            // println!("value {} output position {}",final_value, output_position);
            int_code_string[output_position as usize] = final_value;

        } else {
            break;
        }
    }

    println!("{:?}",int_code_string);
    println!("{}FINAL POSITION",int_code_string[0]);
    Ok(())
}


fn second_problem_second_part() -> io::Result<()> {
    let file = File::open("./data/2.txt")?;
    let reader = BufReader::new(file);
    let mut file_string = "".to_string();
    for line in reader.lines() {
       
        let i = line?;
        file_string.push_str(&i);

    }

    let mut int_code_string : Vec<i32> = file_string.split(",").map(|s| s.parse().unwrap()).collect();
    // int_code_string[1] = 12;
    // int_code_string[2] = 2;
    
    for i in 0..99 {
        for j in 0..99 {
            let (status, result) = run_opcode_program(int_code_string.clone(), i as i32, j as i32);
            if status {
                println!("{}", result);
                break;
            }
        }
    }
    
    // run_opcode_program(int_code_string.clone(), 12 as i32, 2 as i32);
    

    // println!("{:?}",int_code_string);
    // println!("{}FINAL POSITION",int_code_string[0]);
    Ok(())
}

fn run_opcode_program(unmut_int_code_string : Vec<i32>, param_1:i32, param_2:i32) -> (bool,i32) {
    let mut code_string = unmut_int_code_string;
    code_string[1] = param_1;
    code_string[2] = param_2;
    // println!("{}",code_string[1]);

    // println!("{}",code_string[2]);
    for i in (0..code_string.len()).step_by(4) {

        let mut number = code_string[i];
        
        if number == 1 as i32 {
            let input_1 = code_string[code_string[i + 1] as usize];
            let input_2 = code_string[code_string[i + 2] as usize];
            let mut output_position = code_string[i + 3];
            let mut final_value : i32;
            final_value = input_1 + input_2;

            code_string[output_position as usize] = final_value;
        } else if number == 2 as i32 {

            let input_1 = code_string[code_string[i + 1] as usize];
            let input_2 = code_string[code_string[i + 2] as usize];
            let mut output_position = code_string[i + 3];
            let mut final_value : i32;
            final_value = input_1 * input_2;
            code_string[output_position as usize] = final_value;

        } else {
            break;
        }
    }
    if code_string[0] == 19690720 {
        let solution = param_1 * 100 + param_2;
        return (true, solution);
    }
    // if code_string[0]
    return (false, 0);
}
