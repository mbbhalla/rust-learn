fn main() {
    //operators

    let a = 3;
    let a_cubed = i32::pow(a, 3);
    println!("a_cubed = {}", a_cubed);

    let b = 0.5;
    let b_cubed = f64::powi(b, 3);
    let b_pi = f64::powf(b, std::f64::consts::PI);
    println!("b_cubed = {}", b_cubed);
    println!("b_pi = {}", b_pi);

    //bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
    println!("c is {}", c);

    //shift
    let d = 10;
    println!("{} shifted >> 2 = {}", d, d >> 2);
}

