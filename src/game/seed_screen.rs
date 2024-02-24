use super::*;

pub struct SeedScreen {
    seed: u64,
    ticks: u64,
}

impl SeedScreen {
    pub fn new() -> Self {
        Self {
            seed: rand::<u64>(),
            ticks: 0,
        }
    }

    pub fn update(&mut self, buttons: Buttons) -> Option<Game> {
        self.seed = self.seed.wrapping_add(rand::<u64>());
        self.ticks += 1;

        if buttons.contains(Buttons::K) {
            unsafe { rand::set_seed(self.seed) };
            Some(Game::Subway(SubwayLevel::new()))
        } else {
            self.seed = self
                .seed
                .wrapping_mul((buttons.bits() as u64 + 1).wrapping_mul(rand::<u64>()));
            None
        }
    }

    pub fn render<T, E>(&mut self, _: &mut Framebuffer, display: &mut T)
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        if self.ticks % 12 == 0 {
            let style = MonoTextStyle::new(&ascii::FONT_6X10, Rgb565::WHITE);

            let Ok(_) = display.clear(Rgb565::BLACK) else {
                panic!("Failed to draw.");
            };

            let Ok(_) = Text::new("Seeding RNG", Point::new(10, 15), style).draw(display) else {
                panic!("Failed to draw.");
            };
            let Ok(_) = Text::new("Press WASD/IJL", Point::new(10, 30), style).draw(display) else {
                panic!("Failed to draw.");
            };
            let Ok(_) = Text::new("Press K When Ready", Point::new(10, 45), style).draw(display)
            else {
                panic!("Failed to draw.");
            };

            let mut buf = [0u8; 50];
            let mut s = ArrayString::<50>::new();
            s.push_str(self.seed.numtoa_str(10, &mut buf));
            let Ok(_) = Text::new(&s, Point::new(10, 60), style).draw(display) else {
                panic!("Failed to draw.");
            };
        }
    }
}
