use crate::color::{Color};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color> // is a Vec<Vec<Color>> more SIMD-able?
}

impl Canvas {
    pub fn new(width: usize, height: usize, pixels: Vec<Color>) -> Canvas {
        Canvas { width, height, pixels }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let offset = (self.width * y ) + x;

        self.pixels[offset] = color
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        let offset = (self.width * y ) + x;

        self.pixels[offset]
    }

    pub fn to_ppm(&self) -> String {
        let mut body = String::new();
        let header = &format!("P3\n{} {}\n255\n", self.width, self.height);
        body.push_str(header);

        for x in 0..self.height {
            let offset = x * self.width;
            let n = self.width;

            let mut row = String::new();
            for p in &self.pixels[offset..offset + n] {
                row.push_str(&p.ppm_str());
                row.push_str(" ");
            }
            row.push_str("\n");

            let boundary = 70;
            if row.len() > boundary {
                let sub = &row[0..70];

                if let Some(idx) = sub.rfind(" ") {
                    row.insert_str(idx, "\n");
                } else {
                    panic!("That's some weird-ass data");
                }
            }

            body.push_str(&row);
        }

        let trimmed: Vec<&str> = body
            .lines()
            .map( |l| l.trim() )
            .collect();

        let mut result = trimmed.join("\n");
        result.push_str("\n");

        return result
    }
}

