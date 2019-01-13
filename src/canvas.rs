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
        assert!(
            x <= self.width && y <= self.height,
            "Attempt to write out of bounds: {} {}",
            x, y
        );
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

#[cfg(test)]
mod tests {
    use crate::{ color, canvas };

    #[test]
    fn creating_a_canvas() {
        let c = canvas(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert!(c.pixels.iter().all(|&pixel| pixel == color(0.0, 0.0, 0.0) ));
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = canvas(10, 20);
        let red = color(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);

        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn construct_ppm_header() {
        let c = canvas(5, 3);
        let ppm = c.to_ppm();
        let header: Vec<&str> = ppm.lines().take(3).collect();

        assert_eq!("P3", header[0]);
        assert_eq!("5 3", header[1]);
        assert_eq!("255", header[2]);
    }

    #[test]
    fn construct_ppm_pixel_data() {
        let mut c = canvas(5, 3);
        let c1 = color(1.5, 0.0, 0.0);
        let c2 = color(0.0, 0.5, 0.0);
        let c3 = color(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = c.to_ppm();
        let body: Vec<&str> = ppm.lines().skip(3).take(3).collect();

        assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", body[0]);
        assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", body[1]);
        assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", body[2]);
    }

    #[test]
    fn split_long_lines_in_ppm_files() {
        let mut c = canvas(10, 2);
        let color = color(1.0, 0.8, 0.6);
        for y in 0..c.height {
            for x in 0..c.width {
                c.write_pixel(x, y, color)
            }
        }
        let ppm = c.to_ppm();
        let body: Vec<&str> = ppm.lines().skip(3).take(4).collect();

        assert_eq!("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204", body[0]);
        assert_eq!("153 255 204 153 255 204 153 255 204 153 255 204 153", body[1]);
        assert_eq!("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204", body[2]);
        assert_eq!("153 255 204 153 255 204 153 255 204 153 255 204 153", body[3]);
    }

    #[test]
    fn ppm_files_terminate_with_newline() {
        let c = canvas(5, 3);
        let ppm = c.to_ppm();

        assert!(ppm.ends_with("\n"));
    }
}
