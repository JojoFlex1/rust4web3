//adding simple numbers
fn main() {
    let x=5;
    let y=7;
    let sum = x+y;
    println! ( " sum of {} and {} is {}" ,x,y,sum);

}

struct user{ 
 name:string,
    email:string,
    age:u32,
    status:bool,
    }
    impl user{

       pub fn new( name:string, email:string, age:u32 ,status:bool)->user
        user {
            user{
            name:string,
            email:string,
            age:u32,
            status:bool,
            }

        }
    }

        fn main() {
       let mut user1 = user::new( 
           "Mary_jane".to_string(),
           "mary@gmail.com".to_string(),
           21,
           active,
       );
        
           println!( " {:?}", user1);
        }
        


// match

enum Trafficlight{
    Red ,
    Green,
    Amber,
    Purple,
    }
fn main () {
    let number =7;

    let light= Trafficlight::Green;
    match number{
        1=> println!("not number wanted"),
        2=> println!("not number wanted"),
        3=> println!("not number wanted"),
        4=> println!("not number wanted"),
        5=> println!("wanted"),
        _=> println!("really"),
            }

match light{
Trafficlight:: Red=> println!(" stop!"),
Trafficlight::Green=> println!(" go!"),
Trafficlight::Amber=> println!("get ready!"),
Trafficlight::Purple=> println!(" go/stop!"),
}
}
*/

//loops
//loop , creates an infinte loop, and is broken by break
// while , continues to to loop as long as the condition is true
//for , just iterate ( don't quite understand it)*

fn main(){


    let mut count =0;
    loop {
        println!( " count {}",count);
        count +=1;
        if count ==10{
            break;
        }
    }
}
    

// Define a function that takes two parameters, `a` and `b`

fn main() {
let result = add ( 5,3);
println!( " result is {}", result);

}
fn add ( num1:i32 , num2:i32)-> i32{
num1+num2 
}



