fn repeat(f:|x:int| -> int,n:int){
    for  num in range(0,n){
        print!("{}\t",f(num));
    }
    print!("\n");
}

fn main(){
    let c:int = 3;
    let n:int = 10;

    print!("Addition:\t");
    repeat(|x:int| x + c, n);

    print!("Subtraction:\t");
    repeat(|x:int| x - c, n);

    print!("Multiplication:\t");
    repeat(|x:int| x * c, n);

    print!("Division:\t");
    repeat(|x:int| x / c, n);

    print!("Remainder:\t");
    repeat(|x:int| x % c, n);

    print!("Bitwise And:\t");
    repeat(|x:int| x & c, n);

    print!("Bitwise Xor:\t");
    repeat(|x:int| x ^ c, n);

    print!("Bitwise Or:\t");
    repeat(|x:int| x | c, n);

    print!("Bitwise Not:\t");
    repeat(|x:int| !x , n);

    print!("Bitshift Left:\t");
    repeat(|x:int| x << c, n);

    print!("Bitshift Right:\t");
    repeat(|x:int| x >> c, n);
}
