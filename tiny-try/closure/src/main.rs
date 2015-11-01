

fn main() {
    
    let mut i: i32 = 1;

    let mut cl_inc_1 = || { i = i + 1; i} ;
    //let mut cl_inc_2 = || { i = i + 2; i} ;

    println!("{:?},", cl_inc_1());

    //println!("{:?}, {:?}", cl_inc_1(), cl_inc_1());
}



/* 

Closure captures its enclosing environment and holds on to it even when 
enclosing environment is gone while calling the closure.


RUST CLOSURE RULES
------------------

1. While defining a closure, both input and return can be inferred most of the times obserbing the type of captures.

let i = 100 ;
			
let cl_inc_1 = || { i = i + 1; i} ;

/* Or you could write it as */
let cl_inc_1 = || -> i32 { i = i + 1; i} ;







2. Closure will capture the variable in least restricting way. 
	1. Reference (&T)  2. Mutable Reference (&mut T)  3. By Value (T)

THINK OF CAPTURE AS PARAMETER BUT IN SAME SCOPE AS CAPTURING ENVIRONMENT

Capture type = &T
----------------

let i = 100;
let cl_inc_1 = ||{  let j: i32 = i + 1 + i + 2; 
					println!("{}", i);           
					j}; // 'i' is captured by the closure as &T

let cl_inc_2 = ||{let j: i32 = i + 2 + i + 3; j};

println!("{}", i); // 'i' borrowed as &T

CLOSURE TYPE = &T

// Can call same closure multiple times
cl_inc_1();
cl_inc_1(); 

// At the same time you can call other closure multiple times which is also capturing the same variable 'i' as &T
cl_inc_2();
cl_inc_2();

This all possible because of immutable references

Capture type = &mut T
---------------------
let mut i = 100;
let k = 1000;

let mut cl_inc_1 = ||{ 	i = i + k + 1; 					  
						let j: i32 = i + 1 + i + 2; 
						println!("{}", i);
						j}; //'i' is captured by closure as '&mut T'.
							//'k' is captured by closure as '&T'

//CLOSURE TYPE = &mut T

//Cannot use 'i' as long as 'cl_inc_1' is in scope
//let cl_inc_2 = ||{ i = i + 2; let j: i32 = i + 2 + i + 3; j};
//println!("{}", i);




Capture type = T
----------------
let movable = Box::new(3);

// `drop` requires `T` so this must take by value. A copy type
// would copy into the closure leaving the original untouched.
// A non-copy must move and so `movable` immediately moves into
// the closure.


//*** NOTE: Equivalant to closure having a implicit parameter of movable: Box<i32> = movable (T) but in same scope of enclosing env.
let consume = || {
    println!("`movable`: {:?}", movable); //closure captures 'movable' as &T
    drop(movable);    // closure captures 'movable' as 'T'
	//drop(movable);  // can't use movable ever again. It is moved
};

/* You cannot define another closure which is using 'movable'
let consume2 = || {
    println!("`movable`: {:?}", movable);
    drop(movable);    
	//drop(movable);
}; // closure captures 'movable' as 'T'

*/

//CLOSURE TYPE = T

//Can't use 'movable' here as well. 'movable' is already moved. We don't know when the closure gets called and use invalidated box
//drop(movable)


//Cannot call consume twice as closure 'consume' is moving a non-copy type
//and that non-copy type should not me moved again while calling the consume() again
consume();
//consume();

    

'consume' closure is of type 'T' now. Which means it moved one of the environment
variable into it and we call that closure again because it will again try
to move the variable which is 'dead'. 'movable' in this case.






3. While passing the closure to a function, we need to specify its type.

Closure type 'T' = FnOnce  ==>  Which means function accepts any closure which might have possibly captured the
                                environment variable in all possible ways


								But you cannot call this closure twice because it might have moved an environment variable


Closure type '&mut T' = FnMut  ==> Function accepts a closure which might've captured a variable as '&mut T' or '&T' or both


Closure type '&T' = Fn  ==> Function accepts a closure which only captures a variable as &T
*/