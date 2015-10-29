use std::io::{self, BufRead};

fn main(){
    
    let stdin = io::stdin();
    for line in stdin.lock().lines(){
        println!("{:?}", line);
    }
}


/*

pub fn stdin() -> Stdin

[−]

Constructs a new handle to the standard input of the current process.

Each handle returned is a reference to a shared global buffer whose access is synchronized via a mutex. 
If you need more explicit control over locking, see the lock() method.

*/

/*

Q. Why doesn't this work ?

for line in io::stdin().lock().lines(){
        println!("{:?}", line);
}

stdin() returns handle of stdin buffer and lock() returns a reference to the handle. Since hadle isn't binded by any variable, it
gets destroyed at the end of statement and reference returned by lock() would have been pointing to destroyed handle.

http://stackoverflow.com/questions/27468558/rust-lifetime-chaining-function-calls-vs-using-intermediate-variables

http://stackoverflow.com/questions/23440793/why-do-i-get-borrowed-value-does-not-live-long-enough-in-this-example
*/
