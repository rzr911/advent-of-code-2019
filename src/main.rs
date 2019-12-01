use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    // first_problem();
    second_problem();
}


fn first_problem() -> io::Result<()> {
    let mut sum_of_fuel_requirements = 0;

    let file = File::open("./data/1.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
       
        let i = line?;
        let mass: u32 = i.parse::<u32>().unwrap();
        let individual_fuel_required = mass/3 -2;
        sum_of_fuel_requirements += individual_fuel_required;

    }
    print!("{}", sum_of_fuel_requirements);
    Ok(())
}

fn second_problem() -> io::Result<()> {
    let mut sum_of_fuel_requirements = 0;

    let file = File::open("./data/1.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
       
        let i = line?;
        let mass: i32 = i.parse::<i32>().unwrap();
        let mut individual_fuel_required = mass/3 - 2;
        // print!("{}", individual_fuel_required);
        sum_of_fuel_requirements += individual_fuel_required;
        

        // let mut individual_fuel_required_with_fuel_mass_included = individual_fuel_required;
        let mut done = false;
        while !done {
            print!("{}", "init");
            println!("{}", individual_fuel_required);
            if individual_fuel_required < 3  {
                done = true;
                println!("{}","return")
            }
            individual_fuel_required = individual_fuel_required/3-2;
            
            if individual_fuel_required > 0 {
            sum_of_fuel_requirements += individual_fuel_required;
            }
        }
        

    }
    print!("{}", sum_of_fuel_requirements);
    Ok(())
}
