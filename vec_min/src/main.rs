


enum Number{
    Value(i32),
    Nothing
}

/* NOTE:

Q: Why to return 'Number' enum instead of an 32 ?
Ans: How would you return failure cases and integers at the same time ?
     Let's say the vector is empty, How would return the failure using an integer?
     Return -ve values ? Not an option.

     So we use enums.Enum specifies set of types (- optional values) it supports.
     Enums in rust are like unions (With just one extra field to specify correct
     current type during runtime [and optionally extract the value]). So memory is not wasted
     because of multiple types.
*/
fn vec_min(vec: Vec<i32>) -> Number{
    let mut min = Number::Nothing;

    for num in vec{
        match min{
            Number::Nothing => { min = Number::Value(num); }
            Number::Value(val) => {
                let new_min = min_num(val, num);
                min = Number::Value(new_min);
            }
        }
    }

    min
}

fn min_num(n1: i32, n2: i32) -> i32{
    if n1 < n2{
        n1
    }
    else{
        n2
    }
}


fn main() {
    println!("Hello, world!");

    let numbers = vec![4,5,6,1,3,6,3,7,-1];
    let min = vec_min(numbers);

    match min{
        Number::Nothing => println!("Error printing minimum value"),
        Number::Value(val) => println!("Minimum value = {}", val)
    }
}