fn main() {
    //inmutable variable
    let  bunnies = 2;
    println!("bunnies inmutable {bunnies} ");

    //mutable variable
    let mut bunnies_mutable = bunnies;
    println!("bunnies mutables {bunnies_mutable} ");
    bunnies_mutable = 3;
    println!("bunnies mutables {bunnies_mutable} ");

    //constant variable
    const BUNNIES_CONST: i32 = 2;
    println!("BUNNIES_CONST {BUNNIES_CONST} ");

    {
        const BUNNIES_CONST: i32 = 99;
        println!("BNNIES_CONST SHADOWED By scope {BUNNIES_CONST} ");
    }
    println!("Hello, world! I have {bunnies} bunnies");
}
