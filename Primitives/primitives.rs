fn main (){
    let _logical:bool = true;
    let _a_float:f64 = 1.0;
    let _an_integer = 5i32;
    let _default_float = 1.0; //f64
    let _default_integer = 8; //i32
    let mut _inferred_type = 12;
    _inferred_type = 65536;
    let mut _mutable = 12;
    _mutable = 21;
    // mutable = true; //Error! the type of a vartible can't be changed.
    let _mutable = true;
    println!("result: {}{}{}{}{}{}{}", _logical, _a_float, _an_integer, _default_float, _default_integer, _inferred_type, _mutable);
}