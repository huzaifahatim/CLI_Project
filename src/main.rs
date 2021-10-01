use std::env::args;

fn main() {
    //use std::io;
    use std::env;

    let args : Vec<String> = env::args().collect();
    println!("{:?}",args);
    

    let mut name = String::new();
    let mut age = String::new();
    let mut rollno = String::new();

    for (index,arg) in args.iter().enumerate() {
        if arg.as_str() == "--name" {
            name.push_str(args[index + 1].as_str());
            
            
        }
        else if arg.as_str() ==  "--age" {
            age.push_str(args[index + 1].as_str());

            
        }
        else if arg.as_str() == "--rollno" {
            rollno.push_str(args[index + 1].as_str());
    
        }


    } 

println!("NAME: {} age: {} rollno: {}",name , rollno, age);
   
    
}