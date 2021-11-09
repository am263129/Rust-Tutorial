fn main() {
    let a_binding;
    let mut _mutable_integer = 13i32;
    {
        let x =2;
        a_binding = x * x;
        // let _mutable_integer = _mutable_integer;
        _mutable_integer = 50;// Error forzon in this scope ^
    }


    println!("a binding {}", a_binding);

    let another_binding;
    
    // println!("another binding {}", another_binding) // error 

    another_binding = 1;
    println!("another binding {}", another_binding);

    _mutable_integer = 3; //OK no problem
    println!("free integer {}", _mutable_integer);
}