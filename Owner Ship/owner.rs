#[derive(Debug)]
struct Point (i32, i32);
struct Person (String, i32);

fn main(){
    let s1 = String::from("Mufasa");
    let s2 = s1;
    // println!("string content: {}", s1);// Error. s1 is not longer available since it's owner is gaven to s2.
    println!("string content: {}", s2);
    let site = Point(32, 34);
    let prison = Person(String::from("あけちさまのすけ　ひでみつ"), 24);
    let site_copy = site;
    let twin = Person;
    // twin = prison;

    println!("sites = {:?}, {:?} , twin = ",site, site_copy);
}