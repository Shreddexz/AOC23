use std::fs::File;
use std::error::Error;
use std::io::{self,prelude::*, BufReader};

fn main() {
    let something = parse_string();
}

fn parse_string() -> Result<i32, Box<dyn Error>>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut line_in: String = String::new();
    let mut numbers: Vec<i32> = Vec::new();
    let line_number: i32 = 0;
    let mut num: i32 = 0;

    for line in  reader.lines(){
        let mut num_string = String::new();
        let mut out_num: i32 = 0;
        match line {
            Ok(line) => {
                line_in = line;

                for ch in line_in.chars() {
                    match ch.is_numeric(){
                        true => {
                            num_string.push(ch);
                        },
                        false => {

                    },
                    }
                }
                let mut conv = num_string.trim().parse::<i32>().unwrap(); 
                match conv{

                }
                numbers.push(out_num);
                println!("{out_num}");
            },
            Err(_) => {

            },
        }
    }

    print!("Succes!!!");
    Ok(num)
}
