extern crate rand;

use rand::{task_rng,Rng};

fn main(){
    let mut rng = task_rng();
    println!("{:b}", rng.gen_weighted_bool(3));
    println!("{}", task_rng().gen_ascii_str(10));

    let n: uint = rng.gen_range(0u, 10);
    println!("{}", n);
    let m: f64 = rng.gen_range(-40.0, 1.3e5);
    println!("{}", m);
}
