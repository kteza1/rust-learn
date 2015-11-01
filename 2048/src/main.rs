extern crate rand;

use rand::{Rng};
use std::io::{self, BufRead};

fn print_board(board: & mut [[u8; 4]; 4]){
	for row in board.into_iter(){
		println!("{:?}", row);
	}
}

//TODO: Doesn't fill the board with new random number if the slot is already filled
fn generate_random(board: &mut [[u8; 4]; 4]){
	let mut range = rand::thread_rng();
	
	let i = range.gen_range(0, 4);
	let j = range.gen_range(0, 4);

	let num = range.gen_range(2, 3);

	if(board[i][j] == 0){
		board[i][j] = num;
	}

	//println!("{:?}, {:?}", i, j);

}

fn move_left(board: &mut [[u8; 4]; 4]){

	for row in 0..4{
		for column in 1..4{
			//Going from (0..column) will result in arith overflow runtime error
			//One advantage is that even though this is a runtime error, rust didn't
			//allow access and hence retreive wrong value/ segfault.
			for iterate in (1..column).rev(){
				if board[row][iterate - 1] == 0{
					board[row][iterate - 1] = board[row][iterate];
					board[row][iterate]= 0;
				}
			}
		}
	}
}

fn main() {
	/* Array of 4 elements where each element is an array of 4 u8s */
	let mut board = [[0u8; 4]; 4];
	let mut b2 = [0; 6];

	/* Initializing the board */
	generate_random(&mut board);
	generate_random(&mut board);
	//println!("{:?}", board[1][2]);

	print_board(&mut board);

	let stdin = io::stdin();
	
	for line in stdin.lock().lines(){
		println!("{:?}", line);

		/* I had to use match. There is no way I could forget handling the error condition.
		 API writer has enforced this on me. This is fantastic */
		match line {
		    Ok(key) => if key == "\u{1b}[A"{            //up
		    				println!("{:?}", "up");
		    			}
		    			else if key == "\u{1b}[D"{      //left
		    			    println!("{:?}", "left");
		    			    move_left(&mut board);
		    			    //generate_random(&mut board);
		    			}
		    			else if key == "\u{1b}[B"{		//down
		    			    println!("{:?}", "down");
		    			}
		    			else if key == "\u{1b}[C"{		//right
		    			    println!("{:?}", "right");
		    			}
		    			else {
		    			    println!("{:?}", "Invalid key pressed");
		    			},
		    Err(err) => panic!("{:?}", "error")
		}
		print_board(&mut board);	
	}
}
