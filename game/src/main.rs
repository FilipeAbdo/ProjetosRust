use sdl2::keyboard::Keycode;

extern crate sdl2;
fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Game", 900, 700)
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    let mut keycodeValue:Option<Keycode> = Keycode::from_i32(0);
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    println!("Fecha esse trem aí!");
                    break 'main
                },
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::A), ..} => println!("Keycode A"),
                sdl2::event::Event::KeyUp { keycode: Some(Keycode::B), ..} => println!("Keycode B"),
                _ => {},
            }
            match event {
               sdl2::event::Event::KeyUp { keycode, ..} => {
                   println!("Keycode {:?}", keycode);
                   keycodeValue = keycode;
                },
                _ => {},
            }
        }
        println!("keycode value = {:?}", keycodeValue);
        // render window contents here
    }
    println!("Estou fechando seu chato!")
}
