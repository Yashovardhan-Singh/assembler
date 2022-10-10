use std::fs;
use std::io::prelude::*;
use std::collections::HashMap;
use std::env;

fn main() {

    let args : Vec<String> = env::args().collect();
    
    if (args[1].len() < 4) || !(args[1][args[1].len()-4..] == String::from(".eba")) {
        panic!("Not an .eba file!");
    }

    let mut x = fs::File::open(&args[1]).unwrap();
    let mut c = String::new();
    x.read_to_string(& mut c).unwrap();
    let mut out_file = fs::File::create("prg.bin").unwrap();

    let mut bytes = 0;

    let mut labels : HashMap<String, i32> = HashMap::new();

    for i in c.split_terminator("\r\n") {
        parse(&String::from(i), &mut out_file, &mut labels, &mut bytes);
    }
}

fn parse(
    line: &String,
    file: &mut fs::File,
    labels : &mut HashMap<String, i32>,
    bytes : &mut i32
) {

    if line.contains(';') {
        return;
    } 

    if line.contains('$') {
        labels.insert(String::from(line.trim().replace("$","")), *bytes);
        return;
    }

    let col : Vec<&str> = line.trim().split(' ').collect();
    match col[0] {
        "STR" => store(bytes, line, file) ,
        "LDR" => load(bytes, line, file) ,
        "ADD" | "SUB" => add_sub(bytes, line, file) ,
        "JEQ" | "JLT" | "JGT" | "JNE" => jump(bytes, line, file, labels) ,
        _ => (),
    }

}

fn store(
    bytes: &mut i32,
    line: &String,
    file: &mut fs::File
) {
    
    let split_line = line.trim().split(' ').collect::<Vec<&str>>();

    let mut s = String::from("000");


    if ! split_line[1][..].chars().all(char::is_numeric) {
        s += "1";
        s += parse_register(split_line[2]);
        s += "0000";
        s += parse_register(split_line[1]);
    } else {
        s += "0";
        s += parse_register(split_line[2]);
        let temp_var = format!("{:b}", split_line[1].parse::<i32>().unwrap());
        s += &temp_var[..];
    }
    
    *bytes += 16;

    let final_string = &s[..];
    
    write(final_string, file);

}

fn load(
    bytes: &mut i32,
    line: &String,
    file: &mut fs::File
) {
    
    let split_line = line.trim().split(' ').collect::<Vec<&str>>();

    let mut s = String::from("001");    

    if ! split_line[1][..].chars().all(char::is_numeric) {
        s += "1";
        s += "00000000";
        s += parse_register(split_line[1]);
    } else {
        s += "00000";
        let temp_var = format!("{:b}", split_line[1].parse::<i32>().unwrap());
        s += &temp_var[..];
    }
    
    *bytes += 16;

    let final_string = &s[..];
    
    write(final_string, file);

}

fn add_sub(bytes: &mut i32, line: &String, file: &mut fs::File) {
    
    let split_line = line.trim().split(' ').collect::<Vec<&str>>();

    let mut s = String::from("");

    match split_line[0] {
        "ADD" => s += "010",
        "SUB" => s += "011",
        _ => (),
    }
   
    if ! split_line[1][..].chars().all(char::is_numeric) {
        s += "1";
        s += parse_register(split_line[2]);
        s += "0000";
        s += parse_register(split_line[1]);
    } else {
        s += "0";
        s += parse_register(split_line[2]);
        let temp_var = format!("{:01$b}", split_line[1].parse::<i32>().unwrap(), 8);
        s += &temp_var[..];
    }
    
    *bytes += 16;

    let final_string = &s[..];
    
    write(final_string, file);

}

fn jump(
    bytes: &mut i32,
    line : &String, 
    file: &mut fs::File, 
    labels : &mut HashMap<String, i32>
) {

    let mut s = String::from("");
    let split_line = line.trim().split(' ').collect::<Vec<&str>>();

    match split_line[0]
    {
        "JEQ" => s += "100",
        "JLT" => s += "101",
        "JGT" => s += "110",
        "JNE" => s += "111",
        _ => (),
    }
    
    s += &get_label(split_line[1], labels)[..];
    let final_string = &s[..];
    
    *bytes += 16;
    
    write(final_string, file);
    
}

fn get_label(
    x : &str,
    labels : &mut HashMap<String, i32>
) -> String {

    let jh : i32 = *labels.get(x).unwrap_or(&0);
    format!("{:0>13}", get_bin(jh).to_string())

}

fn parse_register(reg : &str) -> &str {

    match reg {
        "AR" => return "0000",
        "C1" => return "0001",
        "C2" => return "0010",
        "A"  => return "0011",
        "B"  => return "0100",
        "C"  => return "0101",
        "D"  => return "0110",
        "E"  => return "0111",
        "F"  => return "1000",
        "G"  => return "1001",
        "H"  => return "1010",
        "I"  => return "1011",
        "J"  => return "1100",
        "K"  => return "1101",
        "L"  => return "1110",
        "M"  => return "1111",
        _    => return "",
    }

}

fn get_bin(num : i32) -> i32 {

    if num == 0 { return num; }
    num % 2 + 10 * get_bin(num / 2)

}

fn write(
    final_string : &str,
    file : &mut fs::File
) {

    let mut buf = [0u8; 2];
    let result = format!("{:01$x}", u32::from_str_radix(final_string, 2).unwrap(), 4);
    hex::decode_to_slice(result, &mut buf).unwrap();
    file.write(&buf).unwrap();

}