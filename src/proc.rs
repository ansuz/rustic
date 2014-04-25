/*
    closu2.rs
    demonstrating closures
    vanilla
*/

fn main(){
    fn call_twice(f: ||) { 
        f(); 
        f(); 
    }
    
    let closure = || { "I'm a closure, and it doesn't matter what type I am"; };

    fn function() { 
        "I'm a normal function"; 
    }

    call_twice(|| println!("{:?}",function()));

    call_twice(|| println!("{:?}","pewpew"));

    call_twice(proc(){println!("pew")});

    call_twice(closure);
    call_twice(function);
}
