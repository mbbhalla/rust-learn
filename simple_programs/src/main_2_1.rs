fn main() {

    //primitive data types 
    
    let a:u8 = 125;
    println!("a = {} size = {} bytes", a, std::mem::size_of_val(&a));

    let c = 2234;   //i32
    println!("c = {} size = {} bytes", c, std::mem::size_of_val(&c));

    let z:isize = 123;
    let size_of_z = std::mem::size_of_val(&z);
    println!("z = {}, size_of_z = {}, {} bit OS", 
        z, size_of_z, size_of_z * 8);

    let ch:char = '∑';
    println!("ch = {}, size_of_ch = {} bytes", ch, std::mem::size_of_val(&c));

    let e:f32 = 2.5;    //default f64
    println!("e = {}, size_of_e = {} bytes", e, std::mem::size_of_val(&e));

    let bool = 2.5 == 5.0;
    println!("bool = {}, size_of_bool = {} bytes", bool, std::mem::size_of_val(&bool));
}

