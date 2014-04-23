/*
    concu.rs
    execute functions concurrently
    standard rust API
    todo: add lambda support
*/

fn threadThis(n:int){
    for num in range(0,n){
        spawn(proc(){
            println!("task {:i}.",num);
            }
        );
    }
}

fn main() {
    threadThis(10);
}
