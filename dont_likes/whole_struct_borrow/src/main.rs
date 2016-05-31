use std::collections::LinkedList;
use std::sync::{Arc, Mutex};

struct Client {
    queue: Arc<Mutex<LinkedList<i32>>>,
    a: i32,
}

impl Client {
    fn print_a(&mut self) {
        println!("{}", self.a);
    }
}

fn main() {
   let list: LinkedList<i32> = LinkedList::new();
   let mut client = Client {a: 10, queue: Arc::new(Mutex::new(list))};
   //borrow to client member
   let queue = client.queue.lock().unwrap();
   // cannot use client any more even though you aren't messing
   // queue with below method
   client.print_a();
}
