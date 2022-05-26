use self::problems::p01::p01;
use self::problems::p02::p02;
use self::problems::p03::p03;
use self::problems::p04::p04;
use self::problems::p05::p05;
use self::problems::p06::p06;
use self::problems::p07::p07;
use self::problems::p08::p08;
use self::problems::p09::p09;
use self::problems::p10::p10;
use self::problems::p25::p25;

pub mod primes;
pub mod problems;

fn main() {
    println!("{}", p01());
    println!("{}", p02());
    println!("{}", p03());
    println!("{}", p04());
    println!("{}", p05());
    println!("{}", p06());
    println!("{}", p07());
    println!("{}", p08());
    println!("{}", p09());
    println!("{}", p10());
    println!("{}", p25());
}
