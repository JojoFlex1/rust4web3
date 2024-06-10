// trait  area {
//     fn area(x:i32,y:i32)->i32
// }
// pub struct rect {
//     width : i32,
//      height:i32
// }
// impl rect for area { 
//     fn area ( &self )->i32{
//         self.width*self.height
//     }

// }
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
// use std::fmt::Display;
// use std::ops::Div;
// use num::Zero;

// fn divide<T>(a: T, b: T) -> Result<T, String>
// where
// T: Zero + Div<Output = T> + Display + Copy + PartialOrd,
// {
// if b == T::zero() {
// Err(format!("Cannot divide by zero"))

// } else {

// Ok(a / b)
// }
// }


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
    // fn test_rect (){
    //     let rect_1 = rect {width:2, height:3};
    //     assert!(rect_1.area==6);
    // }
    // assignment :closure that adds two number and test
    // fn test_closure(){
    //     let add = |x,y| {x+y};
    //     assert!(add(2,4) == 6);
    // }
    // #[test]
    // fn test_one_match(){
    //     let tu = ('c', 2);
    //     assert!(one_vec(tu) == String::from("Consonant"));
    // }
    // #[test]
    // fn test_one_comp(){
        
    //     assert!(compare(1,2)== 1);
    // }

    // #[test]
    // fn test_one_comp_2(){
        
    //     assert!(compare('a', 'b')== 'a' );
    // }
    #[test]
    fn test_one_comp_bool(){
        let bo = vec![true, true];
        assert!(comp_bool(&bo)== "true".to_string() );
    }
// fn test_divide(){
    //assert!(divide(4.0,2.0)==Ok(2.0));
    // assert!(divide<f64>(4.0,2.0)==Ok(2.0));
// }
}
//Write a Rust function that takes a tuple (char, i32) and 
//returns "Vowel" if the first element is a vowel ('a', 'e', 'i', 'o', 'u') and 
//"Consonant" otherwise.

fn one (tup:(char,i32))->String{
    if tup.0=='a' || tup.0=='e' ||tup.0=='i' ||tup.0=='o' ||tup.0=='u' 
    {
        return " vowel".to_string();
    }
    else
    {
        "consonant".to_string()
    }
}
// fn one_match (tup:(char,i32))->String{
//    match tup.0{
//     ('a' ,'e' ,'i' ,'o' ,'u') =>"Vowel".to_string(),
//     _ => "Consonant".to_string()
//    }
// }
fn one_vec( tup:(char,i32))->String{
    let vowel=  vec!['a','e','i','o','u'];
    if vowel.contains(&tup.0)
    {
        return " vowel".to_string();
    }
    else
    {
        "Consonant".to_string()
    }
        
}

fn compare<T>(a: T, b: T) -> T
 where
 T: std::fmt::Display + Copy + PartialOrd,
 {
    if b < a {
    b
    }
    else{
        a
    }
}
 fn comp_bool ( vec_1: &Vec<bool>)->String{
if vec_1.iter().all_equal_to(&true){
    "true".to_string()
}
else if vec_1.iter().all_equal_to(&false){
            "false".to_string()
        } 
    else {
            "mixed".to_string()
        }
 }
 
    



