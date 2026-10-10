fn main(){
    // loop over a range
    for i in 1..4{
        println!("{}", i);
    }

    // loop over an array
    let scores = [90,85,100];
    for score in scores{
        println!("score is: {}", score)
    }
}




// fn main(){
//     // while
//     let mut n = 3;
//     while n!= 0 {
//         println!("{}", n);
//         n-=1;
//     }
// }




// fn main(){
//     //loop
//     let mut counter = 0;

//     let result = loop{
//         counter +=1;
//         if counter == 5 {
//             break counter * 2;
//         }
//     };

//     println!("loop result: {}", result); // return 10

// }




// fn main(){
//     //loop
//     let mut counter = 0;

//     let result = loop{
//         counter +=1;
//         if counter == 5 {
//             break counter;
//         }
//     };

//     println!("loop result: {}", result); //return 5

// }



// fn main(){
//     let number = 6;

//     let lable = if number % 2 == 0 {"even"} else {"odd"};
//     println!("{}", lable);
// }







// fn main(){
//     let number = 6;
    
//     if number % 2 == 0 {// devide and return remainder
//         println!("The number is even")
//     }else {
//         println!("The number is odd!")
//     }
// }







// fn main(){
//     greet("pasan");
//     println!("{}",add(10, 20));

// }

// fn greet(name:&str){
//     println!("Hello {}", name)
// }


// fn add(a: i32, b: i32) -> i32{ // need to define return type ass well
//     a+b // is an expression, it value automatically becomes the return value. (if you add ; this is no longer an expression)

// }











// fn main(){
//     //scalers
//     let age:u32 = 18; // non negative whole number
//     let num: i32 = -100; // default for numbers if you don't specify type
//     let _n1 = -100;

//     let temparature : f64 = 36.6; // 64 bit floating point number for decimals
//     let is_active: bool = true;
//     let grade: char = 'A'; // single character

//     println!(
//         "age:{}, temp{}, active{}, grade:{}, num{}",
//         age, temparature, is_active, grade, num
//     );

//     // compound types. lets to mix different types

//     //tuple
//     let person:(&str, u32, u32) = ("maya",29, 50000);
//     println!("{} is {} years old, has the salary {}", person.0, person.1, person.2);


//     // array
//     let scores: [i32;3] = [90,85,100];
//     println!("first score: {}", scores[0])
// }





// const MAX_POINTS:u32 = 100_000; // always immutable - type is required for constants

// fn main() {
//     let y = 5;
//     println!("y is {}",y);

//     let y = y+1; // shadowing - y is now 6
//     println!("y is {}",y);

//     let y = y*2;
//     println!("y is {}",y);

// }





// fn main() {
//     let mut x = 5; // mutable variable can be changed after set
//     println!("x is {}", x); // {} curly brace is a placeholder
//     x = 6; // this is allowed because x is mutable
//     println!("x is {}",x);
// }





// fn main() {
//     let x = 5; // immutable variable cannot be changed after set
//     print!("x is {}", x); // {} curly brace is a placeholder
//     x = 6; // give an error because x is immutable
//     print!("x is {}",x);
// }





// fn main() {
//     let x = 5; // immutable variable cannot be changed after set
//     print!("x is {}", x); // {} curly brace is a placeholder
// }