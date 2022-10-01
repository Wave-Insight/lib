// use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::structure::Constructor;
// fn lut(d:u8)->[&u8;4]{
//     // d >15, d is invalid
//     let table = [
//         ['0','0','0','0'],
//         ['0','0','0','1'],
//         ['0','0','1','0'],
//         ['0','0','1','1'],

//         ['0','1','0','0'],
//         ['0','1','0','1'],
//         ['0','1','1','0'],
//         ['0','1','1','1'],

//         ['1','0','0','0'],
//         ['1','0','0','1'],
//         ['1','0','1','0'],
//         ['1','0','1','1'],

//         ['1','1','0','0'],
//         ['1','1','0','1'],
//         ['1','1','1','0'],
//         ['1','1','1','1'],


//     ]
// }

// fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(file_path)?;
//     Ok(io::BufReader::new(file).lines())
// }

/// Hello world example for Rust.
pub fn vcd<P>(file_path: P) -> io::Result<Constructor>
where P: AsRef<Path>, {
    fn buf_to_string(buf: &[String]) -> String{
        let mut out = String::new();
        for word in buf{
            if out.is_empty(){
                out = format!("{}",word);
            } else {
                out = format!("{} {}",out, word);
            }
        };
        out
    }
    let mut structure_done = false;
    let mut buf:Vec<String> = Vec::new();
    let mut line_num:usize = 0;
    let mut x_time:usize = 0;
    let mut is_vector = false;
    let mut vector = String::new();
    let mut constructor = Constructor::init();
    if let Some(file_path) = file_path.as_ref().to_str(){
        constructor.set_file_path(file_path);
    };
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines{
        line_num +=1;
        if let Ok(words) = line {
            let word_list = words.split_whitespace();
            for word in word_list{
                if !structure_done {
                    if word == "$end"{
                        match buf[0] {
                            _ if buf[0] == "$version" => {
                                constructor.set_version(&buf_to_string(&buf[1..]));
                            },
                            _ if buf[0] == "$date" => {
                                constructor.set_date(&buf_to_string(&buf[1..]));
                            },
                            _ if buf[0] == "$timescale" => {
                                constructor.set_timescale(&buf_to_string(&buf[1..]))
                            },
                            _ if buf[0] == "$scope" => {
                                if buf[1] == "module"{
                                    let name = &buf[2];
                                    constructor.new_module(name);
                                }
                            },
                            _ if buf[0] == "$upscope" => {
                                constructor.end_module()?;
                            },
                            _ if buf[0] == "$var" => {
                                if buf[1] == "wire"{
                                    let str_id = &buf[3];
                                    let name = &buf[4];
                                    if let Ok(size) = buf[2].parse::<usize>(){
                                        constructor.new_wire(str_id, name, size, [0,0]);
                                    }else{
                                        println!("can not parser {} into uszie, line {}", buf[2], line_num);
                                    };
                                };
                            },
                            _ if buf[0] == "$enddefinitions" => {
                                structure_done = true
                            },
                            _ => println!("Unknown Token {}, line {}", buf[0], line_num)
                        }
                        buf.clear();
                    } else {
                        buf.push(String::from(word));
                    }
                } else {
                    // let  x = word.chars().nth(0)
                    match word.chars().nth(0) {
                        Some('#') => {
                            if let Ok(t) = &word[1..].parse::<usize>(){
                                x_time = t.clone();
                            }else{
                                println!("{} Can NOT parse {}, line {}", "error: ", word, line_num)
                            };
                        },
                        Some('b')|Some('B') => {
                            is_vector = true;
                            vector = word[1..].to_owned();
                        },
                        Some('r')|Some('R') => {
                            println!("{} NOT srupport REAL number {}, line {}", "error: ", word, line_num)
                        },
                        Some(_) => {
                            if is_vector {
                                is_vector = false;
                                let wire_str_id = word.to_owned();
                                constructor.add_wave(&wire_str_id, x_time, &vector)?;
                            } else {
                                let wire_str_id = word[1..].to_owned();
                                if let Some(vector) = word.to_owned().get(0..1){
                                    constructor.add_wave(&wire_str_id, x_time, &vector)?;
                                };
                            }
                        },
                        None => {
                            println!("Unknown Token {}, line {}", word, line_num)
                        }
                    }
                }
            }
        }
    }
    return Ok(constructor);
}


mod test {
    mod parser {
        #[test]
        fn tt(){
            let c = crate::parser::vcd("tests/input.vcd");
            print!("{:?}",c)
        }
    }
}