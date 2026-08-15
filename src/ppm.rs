use std::io::Write;

#[derive(Clone, Copy, Debug)]
pub struct Rgb8(u8, u8, u8);

impl Rgb8 {
    pub fn emit(self) -> String {
        format!("{} {} {}", self.0, self.1, self.2)
    }
}

impl Rgb8 {
    const BLACK: Self = Rgb8(0, 0, 0);
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }
}

pub struct Ppm {
    width: usize,
    height: usize,
    pub buf: Vec<Rgb8>,
}

impl Ppm {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buf: vec![Rgb8::BLACK; width * height],
        }
    }

    pub fn write(&mut self, x: usize, y: usize, color: Rgb8) {
        self.buf[y * self.width + x] = color;
    }

    pub fn write_to(self, mut w: impl Write) {
        write!(w, "P3\n{} {}\n255\n", self.width, self.height).unwrap();
        for col in self.buf {
            write!(w, "{} {} {} ", col.0, col.1, col.2).unwrap();
        }
    }

    pub fn emit(self) -> String {
        let mut s = format!("P3\n{} {}\n255\n", self.width, self.height);
        let c = self
            .buf
            .into_iter()
            .map(|col| col.emit())
            .collect::<Vec<String>>()
            .join(" ");
        s.push_str(&c);
        s
    }
}
