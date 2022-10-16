#![allow(non_snake_case)]
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() 
{
    let randomNum=rand::thread_rng().gen_range(1..=50);
    println!("COMPUTER HAS SELECTED THE RANDOM NUMBER FROM RANGE 1 to 50:-");
    loop
    {
    println!("please input your guess:");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess)
        .expect("not read from user");
let guess: u32=guess.trim().parse().expect("Please enter the number");
 match guess.cmp(&randomNum) 
    {
        Ordering::Less=>println!("too low"),
        Ordering::Greater=>println!("too high"),
        Ordering::Equal=>
        {
            println!("you win");
            break;
        }
    }
}
println!("the secret number that computer guessed was {}",randomNum);
}
