/*
    concu.rs
    execute functions concurrently
    standard rust API
    todo: add lambda support
*/

fn secondTry(f:||,n:int){
    f();
    for num in range(0,n){
        spawn(proc(){
            let localnum = num;
            println!("λ{:i}",localnum);
        });
    }
    f();f();
}

fn main() {
    secondTry(|| println!("λ"),15); // how to give this the send property?
}
