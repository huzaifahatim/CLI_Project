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





    /*if args[1].as_str() == "--name" {
        let name = &args[2];
    }
    else if args[3].as_str() ==  "--age" {
        let age = &args[4];
        
    }
    else if args[5].as_str() == "--rollno" {
        let rollno = &args[6];

    }*/
    
     





       

    

    /*let name = &args[2];
    let age = &args[4];
    let rollno = &args[6];*/

    println!("NAME: {} age: {} rollno: {}",name , rollno, age);






    


/*    println!("ENTER YOUR NAME :");
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
    println!("Your Age is {}", age);*/




    
    
}
