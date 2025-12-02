use crate::day01::rotation::Rotation;

pub struct Lock {
    position: i32,
    pub num_final_clicks: u32,
    pub num_any_clicks: u32,
}

impl Default for Lock {
    fn default() -> Self {
        Self {
            position: 50,
            num_final_clicks: 0,
            num_any_clicks: 0,
        }
    }
}

impl Lock {
    pub fn rotate(&mut self, rotation: Rotation) {
        let (sense, clicks) = match rotation {
            Rotation::Left(n) => (-1, n),
            Rotation::Right(n) => (1, n),
        };

        for _ in 0..clicks {
            self.position += sense;

            if self.position == -1 {
                self.position = 99;
            } else if self.position == 100 {
                self.position = 0;
            }

            if self.position == 0 {
                self.num_any_clicks += 1;
            }
        }

        if self.position == 0 {
            self.num_final_clicks += 1;
        }
    }
}

