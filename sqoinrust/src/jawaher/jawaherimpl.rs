
use crate::karima;

pub fn say_hello(){
    println!("hello from jawaher");
    println!("jawaher calling karima");
    karima::say_hello();
    println!("jawaher calling halima");
    halima::say_hello();
    sqoinlib::halima::say_hello();
}