use sdl2::EventPump;


pub struct Events {
    pump: EventPump,

    pub quit: bool,
    pub key_escape: bool,
}

impl Events {
    pub fn new(pump: EventPump) -> Self {
        Events {
            pump: pump,

            quit: false,
            key_escape: false,
        }
    }

    /// Update the events record
    pub fn pump(&mut self) {
        // If the SDL context is dropped, then poll_iter() will simply stop
        // yieldung any input.
        for event in self.pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                Quit { .. } => self.quit = true,

                KeyDown { keycode, .. } => match keycode {
                    Some(Escape) => self.key_escape = true,
                    _ => {}
                },

                KeyUp { keycode, .. } => match keycode {
                    Some(Escape) => self.key_escape = false,
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
