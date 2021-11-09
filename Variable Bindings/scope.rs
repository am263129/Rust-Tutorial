fn main(){
    let mut long_lived_binding = 1;
    let shadowed_binding = 1;
    {
        let short_live_binding = 2;
        println!("inner short: {}", short_live_binding);
        long_lived_binding +=1;
        let shadowed_binding = "ABC";
        println!("shadowed binding {}", shadowed_binding);
    }

    // println!("outer short: {}", short_live_binding); // error scope {}
    println!("outer long: {}", long_lived_binding);
    println!("ouside block {}", shadowed_binding);
    let shadowed_binding = 2;
    println!("shadow in outside {}", shadowed_binding);
}