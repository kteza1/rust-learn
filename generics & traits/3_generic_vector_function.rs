/* Writing a function which calculates min value of any type of vector

Method 1: Inside function, find out the value of type during runtime and perform operation based on its type.
          But user might have provided a type which you didn't handle in the function. Which is a runtime logic error
          
Method 2: During compilation, function only accepts types which has a implemented methods that it requires. In this case 'min'.
          How do we make sure that 'type' implemented a function? -> Trait
          Let's say we want to make the 'vec_min' funtion take ints, floats and strings.
          Create trait bound on 'vec_min' with 'Minimum' trait. Minimum trait lists 'min' method. Now 'vec_min' will only accept types that implemented 'min' method
          Since we want 'vec_min' to take int, float and string, implement 'Minimum' train on these.
          If user passes any type that doesn't implement minimum method, it will fail at compile time instead of runtime.
          
NOTE: Flow has completely changed with method 2. Instead of 'vec_min' finding out the 'type' of argument and chaning the implementaion based on type,
      type itself is providing stuff that 'vec_min' requires. This way 'vec_min' will remain clean and in future if you want to extend it with new types,
      just make that type implement the 'min' method (Minimum trait). This is extremely useful if 'vec_min' is inside a library whose sources you dont want to change.
*/
 
/*

pub trait Minimum : Copy {
    fn min(self, b: Self) -> Self;
}


fn vec_min<T: Minimum>(vec: Vec<T>) -> Option<T>{
    println!("finding minimum");
}
*/

fn main() {
    println!("Hello World");
}
