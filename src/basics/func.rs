/*  function definition in rust is pretty simple
    below is a null-ary, void function
    it takes no arguments, and returns nothing */    

fn pew(){
  println!("pewpew");
}

/*  If you want to pass an argument, provide a type annotation
    the function below takes an integer, and returns nothing  */

fn sqr(n:int){
  println!("{}",n*n);
}

/*  If you want your function to return a value
    provide a return value annotation   */

fn cube(n:int)->int{
  return n*n*n;
}

fn main(){
  pew();
  sqr(5);
  println!("{}",cube(2));
}
