use super::*;

impl SubwayLevel {
    pub fn render_ui<T, E>(&mut self, display: &mut T)
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        if self.initial_ui {
            let Ok(_) = display.clear(Rgb565::BLACK) else {
                panic!("Failed to draw");
            };
            self.initial_ui = false;
        }

        if self.ticks % 30 == 0 {
            let Ok(_) = display.fill_solid(&Rectangle {
                top_left: Point { x: 10, y: 10 },
                size: Size {
                    width: 120,
                    height: 25,
                },
            }, Rgb565::BLACK) else {
                panic!("Failed to draw");
            };

            let style = MonoTextStyle::new(&ascii::FONT_8X13, Rgb565::WHITE);

            let mut buf = [0u8; 50];
            let mut s = ArrayString::<50>::from("Speed: ").unwrap();
            let speed = ((self.game_speed_scalar - consts::STARTER_GAME_SPEED_SCALAR) * 100000.0) as u64;
            s.push_str(speed.numtoa_str(10, &mut buf));
            let Ok(_) = Text::new(&s, Point::new(12, 20), style).draw(display) else {
                panic!("Failed to draw.");
            };
            let Ok(_) = Text::new("Coins:", Point::new(12, 32), style).draw(display) else {
                panic!("Failed to draw.");
            };
        }
    }
}
