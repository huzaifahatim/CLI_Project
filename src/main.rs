fn main() {
    use std::io;
    use std::env;
    //use std::fs;

    println!("ENTER YOUR NAME :");
    let mut name = String::new();
    name.clear();
    io::stdin().read_line(&mut name).unwrap();
    


    println!("ENTER YOUR ROLL NO :");
    let mut rollno = String::new();
    rollno.clear();
    io::stdin().read_line(&mut rollno).unwrap();
   

    println!("ENTER YOUR AGE :");
    let mut age = String::new();
    io::stdin().read_line(&mut age).unwrap();
    

    println!("Your Name is {}", name);
    println!("Your Rollno is {}", rollno);
    println!("Your Age is {}", age);




    let args : Vec<String> = env::args().collect();
    println!("{:?}",args);

    
}