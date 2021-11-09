#[derive(Debug)]
struct Person{
    name:String,
    age:u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point{
    x:f32,
    y:f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right:Point,
}

fn main(){
    let name = String::from("Mufasa");
    let age = 32;
    let peter = Person{name, age};
    println!("{:?}",peter);

    let point:Point = Point {x:10.3, y:0.4};
    println!("Point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point{x:5.2, ..point};
    println!("second point: ({}, {})",bottom_right.x, bottom_right.y);

    let Point {x:left_edge, y:top_edge} = point;

    let _rectangle = Rectangle{
        top_left: Point{x:left_edge, y:top_edge},
        bottom_right: bottom_right
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("Pair Contains {:?} and {:?}", pair.0, pair.1);
    let Pair(integer, decimal) = pair;

    println!("Pair COntains {:?} and {:?}", integer, decimal);

}