


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

/* NOTES:

Most of the stuff is an expression in Rust. I.e they evaluates to a value

* If - else blocks are expression => If else block evaluates to a value
* match is an expression
* Nested braces are expressions 
* Entire funtion body is an expression

FOR BLOCKS:

Multiple expressions in a  { } (except match) should be separated by ;. Hence the line become statement.
But line interals can be an expression. Statements return ().

            ------ expression ------
let hello = match min{X => 5,Y => 20};
------------- statement --------------

Combined, all the lines separated by ; inside {} will create an expression.

                   --- exp 5 ---          ---- exp 9 --
let a = if 1 < 2 { 1; 2; 3; 4; 5 } else { 4; 6; 7; 8; 9  };
        --------------------- exp 5 ---------------------

Expression value of lines separated by ; in {} is the value of last expression.


*/
