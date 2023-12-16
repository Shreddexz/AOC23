use std::fs::{File, self};
use std::error::Error;
use std::io::{self,prelude::*, BufReader};
use std::ops::Add;

fn main() {
    let mut Slices: Vec<String> = Vec::new();
    let temp_slices = init_slices();
    match temp_slices {
        Ok(slices) => {
            Slices = slices;
            let output = calculate_sum(&Slices);
            let len = &Slices.len();
            match output {
                Ok(sum) => println!("The sum of all the numbers is {sum}"),
                Err(e) => println!("{:?}", e),
            }
        },
        Err(_) => (),
    }
}

fn calculate_sum(SLICES: &Vec<String>) -> Result<u32, Box<dyn Error>>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;

    for line in  reader.lines(){
        let mut out_num: u32 = 0;
        let mut first_num:u32 = 0;


        match line {
            Ok(_line) => {
                let parsed_num = get_line_value(&_line, SLICES);
                first_num = parsed_num;
                while first_num >= 10 {
                    first_num /= 10;
                }
                out_num = first_num * 10 + parsed_num % 10;
                println!("combined {out_num}");
                sum += out_num;
            },
            Err(_) => (),
        }
    }

    Ok(sum)
}

fn get_line_value(line_in: &String, slices: &Vec<String>) -> u32{
    let mut value:u32 = 0;
    let mut numbers: Vec<u32> = Vec::new();
    let mut string_to_parse = String::new();

    for char in line_in.chars() {
        string_to_parse.push(char); 

        if char.is_numeric(){
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
                let last_char = string_to_parse.chars().last();
                string_to_parse.clear();
                match last_char{
                    Some(char) => string_to_parse = char.to_string(),
                    None => todo!(),
                }
            }
        }
    }

    for number in numbers{
        value = value * 10 + number;
    }
        println!("parsed {value}");

    return value
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
