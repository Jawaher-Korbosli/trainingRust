
mod sqoin_module {
    pub mod sqoin_sub_module {

    
    pub struct Sqoiner {
        pub number: i32,
    }
    
    pub trait SqoinRules {
        fn shownumber(sqoiner: &Sqoiner);
    }
    
    
    pub fn print_sqoiner(sqoiner: &Sqoiner)-> ()
    {
        println!(" Printing Sqoin {}", sqoiner.number);
    }
}

}
mod karima {
    pub fn say_hello() {
        println!("I'am Karima");
    }
}



mod khouloud;
mod jawaher;
use std::fmt::{Display, Formatter , Result};
fn main() {



   // println!("Hello, world!");
  
  
   sqoin_module::sqoin_sub_module::print_sqoiner(&sqoin_module::sqoin_sub_module::Sqoiner{number:10});
    println!("----------- Modules -----------");

    karima::say_hello();
    khouloud::say_hello();
    jawaher::jawaherimpl::say_hello();

    println!("----------Variables ------------");

    let y:u8 = 100_000_000u64 as u8;
    let a:u32 = 1000u32;
    let x:i8 = 10;
    // let y:unsize = 10u64;

    let func=|z:u32| z*z;
    println!("valueof x is {}" , x);
    println!("valueof y is {}" , y);
    println!("valueof func (z) is {}" , func(a));


    println!("------------ Tables ----------");
    let mut table:[u32;10] = [10;10];
    for i in 0..10 {
        print!("{}",table[i]);
    }
    println!();
    
    for i in 0..10 {
        table[i] = i as u32;
    }
    for i in 0..10 {
        print!("{}",table[i]);
    }

    println!();
    println!("------------ Tuples ----------");

    let tup:(u32,[u8;4]) =(4,[1,2,3,4]);
    println!("{}",tup.0);

    let b = (10,);
    println!("{}",b.0);

    let tup1: (u32,[u8;4]) ={
        println!("assigning tuple");
        (4,[1,2,3,4])
    };


    println!("------------ Vecteur ----------");
    let mut vec1 = Vec::<u32>::new();
    vec1.push(a);
    for i in vec1 {
        print!("{}",i);
    }
    println!();
   /* println!();
      for i in vec1 {
        print!("{}",i);
    }*/
    
    println!("------------ Memory Management variable----------");
    
    let m1:u32 = 8u32;
    let m2=m1;
    println!("{} {}", m1,m2);
    
    let mut b1 = Box::new(10u32);
    let b2=b1;
    b1 = Box::new(11u32);
    b1= print_box(b1);
    print_box(b1);
    print_box(b2);
    
    println!("------------ Memory Management Reference----------");
    
    let b3 = Box::new(10u32);
    {
    let refb3  = &b3;
    println!("{}",*refb3); 
    } //si on le met dans l'acolate on peut utiliser b3 
    print_box(b3); // 8alta 5atr 9talt b3 
    
    
    let mut  v3 = vec![10u32];
    v3.push(11u32);
    {
        let refv3  = &mut v3;
        refv3.push(11u32);
        //  let refv4  = &v3; on ne peut pas re referencier  v3 car mutable
        println!("{}",refv3[0]); 
    } //si on le met dans l'acolate on peut utiliser b3 
    //   print_box(b3); // 8alta 5atr 9talt b3 


    println!("------------ Memory Management Slice----------");
    let table = [1,2,3,4,5,6];
    
    let slice = &table[1..3];
    for  i in slice {
        print!("{}",i)
    }
    println!();
    
    print_slice(slice);
    println!();
    print_slice(&v3);

    println!("------------ Memory Management Slice----------");
    
    let s = r"bacem"; // b for asci , utf8
    let s1 = "bacem".to_string();
    let mut s2 = String::new();
    s2+=("bacem");
    let a = s2.clone();
    // let b = s2;
    println!("{} {} {}", s , s1 , s2);


    println!("------------ Enum and patterns----------");
    
    let trainer: Trainee = Trainee::Bacem(10u32);
    let trainer1: Trainee = Trainee::Amal("Amal2".to_string());
    let trainer2: Trainee = Trainee::Jawaher(true);
    
    // let t1 = &trainer;
    let t1 = &trainer1;
    use Trainee::*;
    match t1 {
        Bacem(9) => println!("{}",x),
        Amal(x)=> println!("Amal {}",x ),
            _ => println!("i dont know"),
        }
        let ab = match t1 {
            Bacem(9) => 9u32,
            Bacem(x) => *x,
            Amal(x) => x.len() as u32,
            _ => {println!("i dont know");
            10u32
        }
    };
    println!("a is {}", ab);


    println!("------------ Functions ----------");

    let x1 = &10;
    let x2 = &20;
    let x3 = sum (x1,x2);
    println!("sum : {}" , x3);

    let x1= &Box::new(10u32);
    {
        let x2 =&Box::new(12u32);
        {
            let x3 = sum_box(x1,x2);
            println!("sum box : {}" , x3);
        }
    }
// fonction générique 
    println!("sum from integers : {}",sum_t(10u32 , 20u32) );
    println!("sum from integers : {}",sum_t::<u32>(10 , 20) );
    println!("sum from integers : {}",sum_t(4.25f64 , 1.3f64) );


    println!("------------ Structs ----------");

    let mut p : PortfolioAccount = PortfolioAccount{
        name: "Bacem Portfolio".to_string(),
        values: [(Currency::BITCOIN, 1f64),
        (Currency::ETHEREUM, 2f64)
        ]
    };
    let p1 : PortfolioAccount = PortfolioAccount::new("bacem portfolio" , 5.0, 10.0);
   // let p1 : PortfolioAccountGen<f64> = PortfolioAccountGen::new("bacem portfolio" , 5.0, 10.0);
    let p2: Portfolio_account<f64> = Portfolio_account::new("KHoukha", 1.4, 1.5);
    println!("protfolio de karima est :{}", p);
    println!(" portfolio de Bacem est : {}  " , p);
    println!(" portfolio de Bacem est : {}  " , p1);
    p.deposit_bitcoin(2.3);
    println!(" portfolio de Bacem after deposit  est : {}  " , p);
    println!(" portfolio de khoukha after deposit  est : {}  " , p2);

    let mut port  = Portfolio_account::new("bacem portfolio" , 5.0, 10.0);
    let p3 : &mut dyn Wallet = &mut port;
    

    p3.deposit(2.3);
    p3.deposit_multiple(&[1.0,2.0,3.0]);
    p3.withdraw(1.3);
    println!(" wallet  de Bacem after deposit  est : {}  " , p3.get_bitcoin());
    //panic!("there is a problem here we have to exit ");

    let mut a = 9 ; 
    let b = 11 ;
    if a <b {
        println!("a is less then b ");
    }else {
        println!(" a is super then b");
    }

    while a < b {
        println!("{}" , a);
        a+=1;
    }




}

pub enum Trainee{
    Bacem(u32) ,
    Amal(String) , 
    Jawaher(bool), 
}

fn sum<'a> (x1: &'a u32 ,x2: &'a u32) ->  u32 {
    let sum = x1+x2;
    sum
}

use std::ops::Add;
fn sum_t<T: Add<Output=T>> (x1: T,x2:T) ->  T {
    let sum = x1+x2;
    sum
}

fn sum_box<'a> (x1: &'a Box<u32> ,x2: &'a Box<u32>) ->  u32 {
    let sum = x1.as_ref()+x2.as_ref();
    sum
}

fn print_box(b:Box<u32>) -> Box<u32> {
    println!("{}", b);
    b
}


fn print_slice(v: &[u32]){
    for  i in v {
        print!("{}",i)
    }
    println!();
}

enum Currency {
    BITCOIN ,
    ETHEREUM
}

struct PortfolioAccount {
    name: String,
    values: [(Currency , f64);2]
}



impl Display for PortfolioAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
        writeln!(f,"Portfolio : name = {} , BITCOIN ={} , ETH = {}",self.name , self.values[0].1, self.values[1].1);
        Result::Ok(())
    }

}


impl PortfolioAccount {
    fn new (name: &str , bitcoin:f64, eth : f64) -> PortfolioAccount {
        PortfolioAccount{ name: name.to_string(),
        values: [(Currency::BITCOIN, bitcoin),
        (Currency::ETHEREUM, eth)
        ]
    }
    }
    pub fn deposit_bitcoin (&mut self , d:f64) {
        let mut old= self.values[0].1;
        old += d;
        self.values[0]= (Currency::BITCOIN, old);



    }

}
pub struct Portfolio_account<T: Add<Output = T>> {
    name: String,
    values: [(Currency, T); 2],
}

impl Display for Portfolio_account<f64> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(
            f,
            " Portfolio : name {} , BITCOIN ={} , ETH={}",
            self.name, self.values[0].1, self.values[1].1
        );
        Result::Ok(())
    }
}
impl Portfolio_account<f64> {
    fn new(name: &str, bitcoin: f64, eth: f64) -> Portfolio_account<f64> {
        Portfolio_account::<f64> {
            name: name.to_string(),
            values: [(Currency::BITCOIN, bitcoin), (Currency::ETHEREUM, eth)],
        }
    }
    /*pub fn deposit_bitcoin(&mut self, d: f64) {
        let mut old = self.values[0].1;
        old += d;
        self.values[0] = (Currency::BITCOIN, old);
    }*/
}

pub trait Wallet  {
    fn deposit(&mut self , value: f64);

    fn deposit_multiple(&mut self , values: &[f64]) {
        for v in values {
            self.deposit(*v);
        }
    }
    fn withdraw(&mut self , value: f64) ->();

    fn get_bitcoin(&self) -> f64;
    
}

impl Wallet for Portfolio_account <f64>{
    fn deposit(&mut self ,value: f64) {
        let mut old = self.values[0].1;
        old += value;
        self.values[0] = (Currency::BITCOIN, old);

    }
    fn withdraw(&mut self,value: f64){
        let mut old = self.values[0].1;
        old -= value;
        self.values[0] = (Currency::BITCOIN, old);
    }

    fn get_bitcoin(&self) ->f64 {
        return self.values[0].1;
    }
}