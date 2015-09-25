/* Writing a function which adds 2 to every element of vector and a function to multiply 2
   to every element of the vector */
   
/* NOTES: OWNERSHIP AND BORROW RULES
    1. Only one owner at a time
    2. Only 1 active mutable borrow at a time
    3. Every other borrow after a shared borrow should be a shared borrow
*/

trait Arith:Copy{
    fn add(self, b: Self) -> Self;
    fn mult(self, b: Self) -> Self;
    fn print(self);
}

impl Arith for i32{
    fn add(self, b: i32) -> i32{
        self + b
    }
    
    fn mult(self, b: Self) -> Self{
        self * b
    }
    
    fn print(self) {
        println!("Val = {}", self);
    }
}

fn vec_add<T: Arith>(vec: &mut Vec<T>){
    for e in vec.iter_mut(){
        /* e is of type &mut i32. But you can give it to print() which
           expects i32 because rust derefs it implicitly */
        e.print();
        e.add(5);
    }
}

fn main(){
    println!("Hello World");
    let mut vec: Vec<i32> = vec![1,2,3,4,5];
    vec_add(&mut vec);
}


/*

What's the mistake with e.add(5) which is throwing below error. Isn't 'b:Self' of type i32 for the current example

<anon>:35:15: 35:16 error: mismatched types:
 expected `T`,
    found `_`
(expected type parameter,
    found integral variable) [E0308]
<anon>:35         e.add(5);

ANS: Rust won't know the type of 'e' during compile time. It just knows that 'e' is of type 'T' and type 'T'
     is implementing trait 'Arithmatic'.
     
     It'll just compare : T.add(5)  <--> fn add(val: T, b: T)   because Self is implementors type
     so you are comparing 5 and T here which is wrong
     
     Practically, lets say you have implemented 'Arith' for 'f32'. e.add(5) cannot be correct for both i32 and f32
     at the same time because 'f32' expects 'f32' as argument.
*/
