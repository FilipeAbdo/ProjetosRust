use sdl2::keyboard::Keycode;

extern crate sdl2;
fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Prototype", 900, 700)
        .resizable()
        .build()
        .unwrap();

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("cars/car1.png")?;

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    break 'main
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::A), ..} => {
                    if matches!(detect_pressing(Keycode::A, KeyStatus::DOWN),KeyEvent::Pressed) {
                        println!("Pressed A");
                    }
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::A), ..} => {
                    if matches!(detect_pressing(Keycode::A, KeyStatus::UP),KeyEvent::Pressed) {
                        println!("Pressed A");
                    }
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::B), ..} => println!("Keycode B"),
                _ => {},
            }
            // match event {
            //    sdl2::event::Event::KeyUp { keycode, ..} => {
            //        println!("Keycode {:?}", keycode);
            //        keycodeValue = keycode;
            //     },
            //     _ => {},
            // }
        }
        // println!("keycode value = {:?}", keycodeValue);
        // render window contents here
    }
}

struct KeyInfo{
    key:Option<Keycode>,
    status:KeyStatus,
}

#[derive(PartialEq)]
enum KeyStatus{
    DOWN,
    UP,
}

#[derive(PartialEq)]
enum KeyEvent{
    Pressed,
    Released,
    NotPressed,
}

impl Default for KeyEvent {
    fn default() -> Self {
        Self::Pressed
    }
}

fn detect_pressing(key:Keycode, up_down:KeyStatus) -> KeyEvent{
    static mut lastKey:KeyInfo = KeyInfo{ key:None, status:KeyStatus::UP};
    let mut return_value:KeyEvent = KeyEvent::NotPressed;
    unsafe{
        if lastKey.key == Some(key) && matches!(up_down, KeyStatus::UP) && matches!(lastKey.status, KeyStatus::DOWN) {
            return_value = KeyEvent::Pressed;
        }else{
            return_value = KeyEvent::NotPressed;
        }
        lastKey.key = Some(key);
        lastKey.status = up_down;
    }
    return  return_value;
}