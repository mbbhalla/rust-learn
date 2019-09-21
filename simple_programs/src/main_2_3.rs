
const SOMEVAR:u8 = 42;  //inlined wherever it is used
static mut globalStatic:u8 = 100;

fn main() {
    //scopes

    let mut a = 123;
    let c = 10;

    //curly braces create a scope
    //scope start
    {   
        let b = 234;
        a = 12;
        let c = 20;     //c shodows outside scope "c"
        println!("b inside = {}", b);
        println!("a inside = {}", a);
        println!("c inside = {}", c);   //value inside scope
    }
    //scope end

    let b = 456;
    println!("a outside = {}", a);
    println!("b outside = {}", b);
    println!("c outside = {}", c);  //value outside scope

    println!("SOMEVAR = {}", SOMEVAR);

    unsafe {
        globalStatic = globalStatic + 1;
        println!("globalStatic = {}", globalStatic);
    }
    
    //CANNOT USE HERE 
    //println!("globalStatic = {}", globalStatic);
}