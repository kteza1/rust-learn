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

fn vec_add<T: Arith>(vec: &mut Vec<T>, a: T){
    /* Returns mutable reference to current element */
    for e in vec.iter_mut(){
        /* e is of type &mut i32. But you can give it to print() which
           expects i32 because rust derefs it implicitly */
        *e = e.add(a);
    }
}

fn vec_mul<T: Arith>(vec: &mut Vec<T>, a: T){
    for e in vec.iter_mut(){
        *e = e.mult(a);
    }
}

fn main(){
    let mut vec: Vec<i32> = vec![1,2,3,4,5];
    
    /* Only immutable borrows allowed further. Till 'v1' is in scope */
    //let v1 = &vec[1];
    
    /* No more borrows allowed further. Till 'v2' is in scope */
    //let mut v2 = &vec[2];
    
    /* var which is borrowing 'vec' will anway wont be in scope after the exec-
       ution of this function. So no prob */
    vec_add(&mut vec, 1);
    
    vec_mul(&mut vec, 2);
    
    for e in vec{
        println!("{}", e);
    }
}
