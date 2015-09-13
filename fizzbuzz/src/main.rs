use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;


fn main() {
    let mut read_options = OpenOptions::new();
    read_options.read(true);

    let mut write_options = OpenOptions::new();
    write_options.write(true).create(true);

    /* We may use Path to read/write existing file or create
       new files. Hence, no error handling for Path. Erros happen
       during 'open' based on path options. */
    let read_file = read_options.open(Path::new("numbers.txt")).unwrap();
    let file_reader = BufReader::new(&read_file);

    let write_file = write_options.open(Path::new("output.txt")).unwrap();
    let mut writer = BufWriter::new(&write_file);

    for line in file_reader.lines(){
        let line = line.unwrap();
        let num = line.parse::<i32>().unwrap();

        match (num % 5, num % 3){
            (0, 0) => writeln!(& mut writer, "Num = {}", "FizzBuzz"),
            (0, _) => writeln!(& mut writer, "Num = {}", "Fizz"),
            (_, 0) => writeln!(& mut writer, "Num = {}", "Buzz"),
            _      => writeln!(& mut writer, "Num = {}", num)
        };
    }


}
