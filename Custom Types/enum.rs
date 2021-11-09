enum WebEvent {
    PageLoad,
    PageUnload, 
    KeyPress(char),
    Paste(String),
    Click {x:i64, y:i64}
}

fn inspect(event:WebEvent){
    match event{
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page Unloaded"),
        WebEvent::KeyPress(c) => println!("pressed \"{}\"",c),
        WebEvent::Paste(s) =>println!("Pasted {}", s),
        WebEvent::Click {x,y} => {println!("click at x={}, y={}" , x, y);},
    }
}

fn main(){
    let pressed = WebEvent::KeyPress('X');
    let pasted = WebEvent::Paste("My text".to_owned());
    let click = WebEvent::Click{x:35, y:43};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;


    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}