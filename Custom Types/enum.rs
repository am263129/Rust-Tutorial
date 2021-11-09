static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i32, y: i32 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page Unloaded"),
        WebEvent::KeyPress(c) => println!("pressed \"{}\"", c),
        WebEvent::Paste(s) => println!("Pasted {}", s),
        WebEvent::Click { x, y } => {
            println!("click at x={}, y={}", x, y);
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('X');
    let pasted = WebEvent::Paste(LANGUAGE.to_owned());
    let click = WebEvent::Click {
        x: THRESHOLD,
        y: THRESHOLD,
    };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
