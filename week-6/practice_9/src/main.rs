fn main() {
    let na:i32 = 10;
    let nb:i32 = 20;

    println!("Value of A: {}",na);
    println!("Value of B: {}",nb);

    let mut res = na>nb;
    println!("A greater than B: {}",res);

    res = na<nb;
    println!("A lesser than B: {}",res);

    res = na>=nb;
    println!("A greater than or equal to B: {}",res);

    res = na<=nb;
    println!("A lesser than or equal to B: {}",res);

    res = na==nb;
    println!("A equal to B: {}",res);

    res = na!=nb;
    println!("A not equal to B: {}",res);
}
