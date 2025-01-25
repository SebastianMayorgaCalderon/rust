mod func;
use crate::func::{tuple_function, array_function};

fn main() {
   let tuple_result: (i32, i32) = tuple_function(33,44);
   println!("tuple result: 0-{} 1-{}", tuple_result.0, tuple_result.1);
   let array_result = array_function(tuple_result);
   println!("array result: [{},{}]", array_result[0], array_result[1]);
}
