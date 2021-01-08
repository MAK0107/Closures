use std::time::Duration;
use std::thread;
use std::io;
use rand::Rng;
#[warn(unused_variables)]
fn main(){ 
//defining closures
let closure_variable = | |{
    println!(" I am an anonymous function");
};

closure_variable();

//passing arguments in closures

let var = | x |{   // x is a parameter in the vertical pipes. 
    println!("{}",x);
};
var(1.5);

//workout complex Algorithm 

    let random_number = rand::thread_rng().gen_range(2..5);
    println!("The RN is {}", random_number);

    println!("Enter your SUSV: ");   
    let mut simulated_user_specified_value = String::new();
    io::stdin()
        .read_line(& mut simulated_user_specified_value)
        .expect("Failed to read line");


    generate_workout(simulated_user_specified_value, random_number);


}

//workout complex algorithm functions
fn simulated_expensive_calculation(intensity: String) -> String{
    println!("Calculating Slowly");
    thread::sleep(Duration::from_secs(2));
    intensity

}

fn generate_workout (intensity:String  , _random_number: u32){

    //let expensive_result = simulated_expensive_calculation(intensity); //we've called the SEC fn here  
                                                            // passed intensity through it and stored it
                                                            //in int1. this reduces fn calling twice
    let int2 = intensity.clone();
    let int1 = intensity.clone();
    let expensive_closure = |num|{
        println!("Calculating Slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < "25".to_string(){
    println!("today, do {} push-ups", expensive_closure(int2));

    println!("Also do {} sit-ups", expensive_closure(int1));

}
    else{

        if _random_number == 3{
            println!("Take a break today");
        }
        else{
            println!("Today run for {} minutes",expensive_closure(intensity));
        }
}


}    