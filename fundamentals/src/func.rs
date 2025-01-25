pub fn tuple_function(param1:i32, param2: i32) -> (i32, i32) {
    let tuple: (i32, i32) = (param1, param2);
    return tuple
}

pub fn array_function(tuple_param: (i32, i32)) -> [i32;2] {
    [tuple_param.0, tuple_param.1]
}
