
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
        let secret = rand::thread_rng().gen_range(1,101);

    loop{

        println!("please enter a num!");
        let mut x =  String::new();
        io::stdin().read_line(&mut x).expect("Fuck.");
        println!("your guessed number :{}", x);
        let mut y : std::num::ParseIntError;
        
        let k: i32 = match x.trim().parse() { 
            Ok(num) =>  num,
            Err(err) => { 
                println!("{}",err); y=err;  continue;
            },
        };

        match k.cmp(&secret){
            Ordering::Less => {println!("little idiot {} < {}",k, secret)},
            Ordering::Greater => println!("big idiot {} > {}",k, secret),
            Ordering::Equal => {println!("u is right FINALLY"); break;},
        }
    }
    
}
