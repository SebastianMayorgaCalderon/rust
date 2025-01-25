fn main() {
    let text: String=my_function(1, "jhon");
    let text2: String=my_function2(2, "mario");
    println!("{}", text);
    println!("{}", text2);
}

fn my_function(param1:i32, param2: &str) -> String{
    format!("hello {} {}", param1, param2)
}
fn my_function2(param1:i32, param2: &str) -> String{
    return format!("hello {} {}", param1, param2);
}