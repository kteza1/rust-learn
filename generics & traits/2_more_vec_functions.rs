enum Number{
    Nothing,
    Number(i32)
}

/* Capable of handling empty vectors as well */
fn vec_min(vec: Vec<i32>) -> Number{
    
    let mut min = Number::Nothing;
    
    for i in vec{
        match min{
            Number::Nothing => min = Number::Number(i),
            Number::Number(num) => if i < num{
                                        min = Number::Number(i)
                                    }
        }
    }
    min
}

fn vec_sum(vec: Vec<i32>) -> Number{

    let mut sum = Number::Nothing;
    
    for i in vec{
        match sum{
            Number::Nothing => sum = Number::Number(i),
            Number::Number(num) => sum = Number::Number(num + i)
        }
    }
    sum
}

impl Number{
    fn print(self){
        match self{
            Number::Nothing => println!("Nothing to print"),
            Number::Number(num) => println!("Number = {}", num)
        }
    }
}

fn vec_print(vec: Vec<i32>) -> (){
    for i in vec{
        println!("Num = {}", i);
    }
}

fn main() {
    println!("Hello, world!");
    
    let vec = vec![1,4,6,3,6,2,-1,0];
    //let vec = vec![];
    let min = vec_min(vec);
    
    min.print();
    
/*  Leads to an error. 'vec' is already moved into vec_min(). Now vec_min() will
    own it. 'vec' will be destroyed at the end of vec_min()
    
    let sum = vec_sum(vec);
*/

    let vec2 = vec![1,4,6,3,6,2,-1,0];
    let sum = vec_sum(vec2);
    sum.print();
    
    let vec3 = vec![1,4,6,3,6,2,-1,0];
    vec_print(vec3);
}
