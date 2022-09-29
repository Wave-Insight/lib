// use std::io::prelude::*;
use std::fs::File;
// use std::str;
// use std::thread;
// use tokio::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// use std::time::Duration;
// use std::fs::File;
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
    let mut line_num:usize = 0;
    let mut buf:Vec<String> = Vec::new();
    let mut constructor = Constructor::init();
    // constructor.set_file_path(file_path.);
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines{
        line_num +=1;
        if let Ok(words) = line {
            let word_list = words.split_whitespace();
            for word in word_list{
                if word == "$end"{
                    match buf[0] {
                        _ if buf[0] == "$version" => constructor.set_version("aaa"),
                        _ if buf[0] == "$date" => constructor.set_date("bbb"),
                        _ if buf[0] == "$timescale" => constructor.set_timescale("ccc"),
                        _ if buf[0] == "$scope" => {
                            if buf[1] == "module"{
                                let name = &buf[2];
                                constructor.new_module(name);
                            }
                        },
                        _ if buf[0] == "$upscope" => {
                            constructor.end_module();
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
                        _ if buf[0] == "$enddefinitions" => constructor.set_version("aaa"),
                        _ => println!("Unknown Token {}, line {}", buf[0], line_num)
                    }
                    buf.clear();
                } else {
                    buf.push(String::from(word));
                }
            }
        };
    };
    return Ok(constructor);
}

mod test {
    mod parser {
        #[test]
        fn tt(){
            let c = crate::parser::vcd("/Users/junzhuo/Developer/Wave-Insight/lib/tests/input.vcd");
            print!("{:?}",c)
        }
    }
}