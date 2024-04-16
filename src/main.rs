//converting celcius to fahrenheit and vice versa
//we create the fahrenheit function

// use std::io;
// use colored::Colorize;

// fn main(){
//     println!("Temperature conversions");
// println!("type 'exit' to break out of the loop");
//     loop{
//     println!("{}", "Please input the number".red());
    
// let mut temperature = String::new();
// // we import the std::io library
// io::stdin().read_line(&mut temperature).expect("Failed to read line");
// if temperature.trim() == "exit"{
//     break;
// }
// //we parse this temperature as the floating type
// let temperature = temperature.trim().parse::<f64>().unwrap();
// let conv_temp:f64 = (temperature - 32.0) * 5.0/9.0;

//     println!("{} degrees Fahrenheit is {} degrees in celcius",temperature, conv_temp);
// }
// }

////////////////////////////////////////////////////////////////////
fn conversion(celcius:f64) -> f64{
    (celcius * 9.0/5.0) + 32.0
}

use std::io;
use colored::*;
fn main(){

    println!("Note, type 'exit' to break out of the loop");

    println!("{}","Temperature converter: Celcius to Fahrenheit".green());
    loop{
    //prompt user for input
    println!("{}","Please enter the temperature in degree celcius:".blue());
    //read input from user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    //convert input to f64
    let celcius:f64 = input.trim().parse::<f64>().expect("Please enter a valid number");
    //convert celcius to fahrenheit
    let fahrenheit = conversion(celcius);
    //display the output
    println!("{} degrees celcius is equal to {} degrees fahrenheit", celcius, conversion(celcius))

}
}

//in this programme, we have successfully written temperature conversions from fahrenheit to celcius