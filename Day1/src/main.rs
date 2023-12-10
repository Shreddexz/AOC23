use std::fs::File;
use std::error::Error;
use std::io::{self,prelude::*, BufReader};

fn main() {
    let mut SLICES: Vec<String> = Vec::new();
    let temp_slices = init_slices();
    match temp_slices {
        Ok(slices) => SLICES = slices,
        Err(_) => todo!(),
    }
    let output = calculate_sum(&SLICES);
    match output {
        Ok(sum) => println!("The sum of all the numbers is {:?}", sum),
        Err(e) => println!("{:?}", e),
    }
    // println!("The sum of all the numbers is {:?}", output);
}

fn calculate_sum(SLICES: &Vec<String>) -> Result<u32, Box<dyn Error>>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut line_in: String = String::new();
    let mut numbers: Vec<u32> = Vec::new();
    let mut num: u32 = 0;
    let mut sum: u32 = 0;

    for line in  reader.lines(){
        let mut num_string = String::new();
        let mut out_num: u32 = 0;
        let mut line_num: u32 = 0;
        let mut line_string = String::new();

        // match line {
        //     Ok(line) => {
        //         line_in = line;
        //         get_line_value(&line_in, &SLICES);
        //
        //         for ch in line_in.chars() {
        //             match ch.is_numeric(){
        //                 true => {
        //                     num_string.push(ch);
        //                 },
        //                 false => {
        //
        //                 },
        //             }
        //         }
        //         let conv = num_string.trim().parse::<u32>().unwrap(); 
        //         let first_num = num_string.trim().chars().nth(0);
        //         let last_num = conv % 10;
        //
        //         match first_num {
        //             Some(value) => out_num = value.to_digit(10).unwrap(),
        //             None => todo!(),
        //         }
        //         out_num = out_num * 10 + last_num;
        //
        //         sum += out_num;
        //         println!("{out_num}");
        //     },
        //     Err(e) => {
        //         println!("{:?}", e);
        //     },

        match line {
            Ok(_line) => line_string = _line,
            Err(_) => todo!(),
        }

        match get_line_value(&line_string, SLICES){
            Ok(value) => line_num = value,
            Err(_) => todo!(),
        }

        let mut first_num: u32 = 0;
        match line_num.to_string().chars().nth(0){
            Some(value) => num_string.push(value),
            None => println!("Error: nothing found"),
        };

        match char::from_u32(line_num % 10){
            Some(value) => num_string.push(value),
            None => println!("Error: Cannot convert"),
        }

        out_num = num_string.trim().parse::<u32>().unwrap();
        sum += out_num;
    }

    Ok(sum)
}

fn get_line_value(line_in: &String, slices: &Vec<String>) -> Result<u32, Box<dyn Error>>{
    let mut n = 0;
    let value:u32 = 0;
    let mut numbers: Vec<u32> = Vec::new();
    let mut string_to_parse = String::new();

    for char in line_in.chars() {
        string_to_parse.push(char); 

        if(char.is_numeric()){
            numbers.push(char.to_digit(10).unwrap());
            string_to_parse.clear();
        }

        for slice in slices{
            if string_to_parse.as_str().contains(slice){
                println!("{slice}");
                match slice.as_str() {
                    x if x.contains("one") =>{numbers.push(1)}, 
                    x if x.contains("two") =>{numbers.push(2)}, 
                    x if x.contains("three") =>{numbers.push(3)}, 
                    x if x.contains("four") =>{numbers.push(4)}, 
                    x if x.contains("five") =>{numbers.push(5)}, 
                    x if x.contains("six") =>{numbers.push(6)}, 
                    x if x.contains("seven") =>{numbers.push(7)}, 
                    x if x.contains("eight") =>{numbers.push(8)}, 
                    x if x.contains("nine") =>{numbers.push(9)}, 
                    &_ => {},
                }
                string_to_parse.clear();
            }
        }
    }

    // for number in numbers  {
    //     println!("{number} | {n}");
    //     n+= 1;
    // }
    Ok(value)
}

fn init_slices() -> Result<Vec<String>, Box<dyn Error>>{
    let mut slices : Vec<String> = Vec::new();
    slices.push(String::from("one"));
    slices.push(String::from("two"));
    slices.push(String::from("three"));
    slices.push(String::from("four"));
    slices.push(String::from("five"));
    slices.push(String::from("six"));
    slices.push(String::from("seven"));
    slices.push(String::from("eight"));
    slices.push(String::from("nine"));
    Ok(slices)
}
