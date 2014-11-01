/* 
    Programming would be really difficult lacking variable assignment
    So let's look at how Rust handles the task
*/

/*  Global variables */

//  Global variables should be static!
//  There may be exceptions, I will investigate.
//  You need to declare a type, as well. Rust has a lot of types.

/*  rustc will throw a warning
    if you try to give global vars a lowercase identifier */

static X : int=5;

fn main(){
    // show off your shiny new variable!
    println!("{}",X); 
}
