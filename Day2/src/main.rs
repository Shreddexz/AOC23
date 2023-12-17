use std::io::BufRead;
use std::{fs::File, io::BufReader};
use std::error::Error;

pub const MAX_CUBES_RED: u32 = 12;
pub const MAX_CUBES_GREEN: u32 = 13;
pub const MAX_CUBES_BLUE: u32 = 14;

fn main() {
    let file_lines: Vec<String>;
    match get_file_lines() {
        Ok(lines) => {
            file_lines = lines;
            parse_lines(file_lines);
        },
        Err(err) => println!("{err}"),
    }
}

fn get_file_lines() -> Result<Vec<String>, Box<dyn Error>>{
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines(){
        match line {
            Ok(line) => {
                lines.push(line);
            },
            Err(err) => println!("{err}"),
        }
    }
    Ok(lines)
}

fn parse_lines(lines: Vec<String>){
    let mut sum: u32 = 0;
    let mut line_idx: u32 = 0;
    for line in lines{
        line_idx += 1;
        let mut num_out: u32 = 0;
        if let Some((game_string, cube_string)) = line.split_once(':'){
            let playable = calculate_cubes(cube_string.to_string());
            if playable{
                sum += line_idx;
                println!("Game {line_idx} is playable");
            }
        }
    }
    println!("{sum}");
}

fn calculate_cubes(input: String) -> bool{
    for part in input.split([',', ';']){
        let mut cubecount_red: u32 = 0;
        let mut cubecount_green: u32 = 0;
        let mut cubecount_blue: u32 = 0;
        let mut count: u32 = 0;
        for char in part.chars(){
            if char.is_numeric(){
                count = (count * 10) + (char.to_digit(10).unwrap());
                println!("{count}");
            }
        }

        match part{
            x if x.contains("red") => {cubecount_red += count},
            x if x.contains("green") => {cubecount_green += count},
            x if x.contains("blue") => {cubecount_blue += count},
            &_ => {println!("other????????")},
        }

        if cubecount_red > MAX_CUBES_RED ||
        cubecount_green > MAX_CUBES_GREEN || 
        cubecount_blue > MAX_CUBES_BLUE{
            println!("false");
            println!("{part}");
            println!("{cubecount_red} | {cubecount_green} | {cubecount_blue}");
            return false;
        }
    }

    return true;
}

fn init_colors() -> Vec<String>{
    let mut colors = Vec::new();
    colors.push(String::from("red"));
    colors.push(String::from("green"));
    colors.push(String::from("blue"));
    return colors;
}
