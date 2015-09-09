use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;

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

    let path = Path::new("output.txt");

    /* Cannot do 'create', 'write' .. directly with new()
       due to problems with temporary variable lifetime.
       https://github.com/rust-lang/rfcs/blob/master/text/0066-better-temporary-lifetimes.md
       RFC might fix this

       let range = [1,2,3,4,5,6,7,8,9].iter(); Similar to this
       [1,2....] is dropped after the statement making range invalid*/
    let mut options = OpenOptions::new();

    /* We can use same 'options' to open multiple files */
    options.create(true).write(true).append(true);

    let write_file = match options.open(path){
        Ok(file) => file,
        Err(..) => panic!("Error opening file"),
    };

    let mut writer = BufWriter::new(&write_file);

    println!("Hello, world!");

    let file = match File::open("numbers.txt"){
        Ok(file) => file,
        Err(err) => panic!("Unable to open file"),
    };

    let file = BufReader::new(&file);
    for line in file.lines(){
        let line = line.unwrap();
        let num = line.parse::<i32>().unwrap();
        let factorial = fact(num);
        writeln!(& mut writer, "Factorial of {} = {}", num, factorial);
    }
}
