use dust::loader;
use std::io::{BufRead, Lines};
use std::str;

pub fn load<F, T>(name: &str, mut on_load: F) where F: FnMut(Vec<T>), T: str::FromStr
{
    let on_l = |text: Box<BufRead>|
    {
        println!("");
        println!("Loading: {}", name);
        let mut meta_data = MetaData {format: FORMAT::NONE, file_type: FILETYPE::NONE};
        let mut lines_iter = text.lines();
        loop
        {
            let line = lines_iter.next().unwrap().unwrap();
            let mut words: Vec<&str> = line.trim().split(' ').map(|s| s.trim()).collect();
            words.retain(|&i| i != "");

            if words.len() > 0
            {
                if *words.first().unwrap() == "//"
                {
                    break;
                }
                read_meta_data_into(&words, &mut meta_data);
            }
        }
        let data = read_data::<T>(&mut lines_iter);
        on_load(data);
        println!("Format: {:?}", meta_data.format);
    };
    loader::load(name, on_l);
}

fn read_data<T>(lines_iter: &mut Lines<Box<BufRead>>) -> Vec<T> where T: str::FromStr
{
    let mut data = Vec::new();
    let mut no_attributes = -1;
    loop
    {
        let line = lines_iter.next().unwrap().unwrap();
        let mut words: Vec<&str> = line.trim().split(|x| (x == ' ') || (x == '(') || (x == ')') ).map(|s| s.trim()).collect();
        words.retain(|&i| i != "" && i != ")" && i != "(");

        if words.len() > 0
        {
            if *words.first().unwrap() == "//"
            {
                break;
            }

            for word in words {
                if no_attributes == -1
                {
                    match word.parse::<i32>() {
                        Ok  (i) => { no_attributes = i },
                        Err(..) => {},
                    }
                }
                else {
                    match word.parse::<T>() {
                        Ok  (i) => {data.push(i)},
                        Err(..) => {},
                    }
                }
            }
        }
    }
    data
}

#[derive(Debug)]
enum FORMAT {ASCII, BINARY, NONE}

#[derive(Debug)]
enum FILETYPE {POINTS, OWNER, NONE}

struct MetaData {
    format: FORMAT,
    file_type: FILETYPE
}

fn read_meta_data_into(words: &Vec<&str>, meta_data: &mut MetaData)
{
    if words.len() > 1
    {
        match *words.first().unwrap() {
            "format" => { meta_data.format = match words[1] {
                "ascii;" => FORMAT::ASCII,
                "binary;" => FORMAT::BINARY,
                _ => FORMAT::NONE
            }},
            "object" => { meta_data.file_type = match words[1] {
                "owner;" => FILETYPE::OWNER,
                "points;" => FILETYPE::POINTS,
                _ => FILETYPE::NONE
            }},
            &_ => {}
        }
    }
}