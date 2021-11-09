fn main(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };
    println!(" The Result is {}", result);

    let mut number = 5;
    while number !=0 {
        println!("{}!",number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("element {}",element);
    }


    for number in (1..4).rev(){
        println!("{}!",number);
    }
    println!("LIFTOFF!!!");

    {
        let condition = true;
        let number = if condition {5} else {6};
        println!("number is {}" , number);
    }

}