use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn fact(num: i32) -> i32{
    if num <= 0{
        println!("Invalid number");
        return -1;
    }
    if num ==  1{
        return 1;
    }
    num * fact(num - 1)
}

fn main() {
    println!("Hello, world!");
    let file = match File::open("numbers.txt"){
        Ok(file) => file,
        Err(err) => panic!("Unable to open file"),
    };

    let file = BufReader::new(&file);
    for line in file.lines(){
        let line = line.unwrap();
        let num = line.parse::<i32>().unwrap();
        println!("Factorial of {} = {}", num, fact(num));
    }
}
