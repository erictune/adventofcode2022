use grid::*;


#[derive(Debug)]
pub struct Crt {
    pixels: Grid<bool>,
    cycle: i64,
    col: u16, // u16 can eb cast up to usize for index or i32 for difference in positions.
    row: u16,
    sprite_col: i16,  // -1 is valid sprite pos.  
}

impl Crt {
    pub fn new() -> Self {
        Crt {
            pixels: Grid::new(6, 40),
            cycle: 1,
            row: 0,
            col: 0,
            sprite_col: 0,  // I guess?
        }
    }

    pub fn set_sprite(&mut self, mut h: i64) {
        if h < -2 { h = -1; }
        if h > 41 { h = 41; }
        self.sprite_col = h as i16;
    }

    pub fn tick(&mut self) {
        assert!(self.cycle <= 240);

        let diff = self.col as i32 - self.sprite_col as i32;
        let pixel_val = diff <= 1 && diff >= -1;
        self.pixels[self.row as usize][self.col as usize] = pixel_val;
        self.cycle += 1;
        (self.row, self.col) = match (self.row, self.col) {
            // Abuse of match syntax?  But fun.
            (5, 39) => (0,0),
            (r @ 0..=5, c @ 0..=38) => (r, c+1),
            (r @ 0..=5, 39) => (r+1, 0),
            _ => panic!("Invalid row/column state"),
        };
    }

    pub fn as_multiline_string(&self) -> String {
        let mut s = String::new();
        for r in 0..6 {
            for c in 0..40 {
                let ch = match self.pixels[r][c] {
                    true => '#',
                    false => '.'
                };
                s.push(ch)
            }
            s.push('\n');
        }
        s
    }
}


#[test]
fn test_crt() {
    let mut crt = Crt::new();
    for hz in 0..6 {
        crt.set_sprite(hz*hz); // h = 0, 1, 4, 9, 16, 25
        for _ in 0..40 {
            crt.tick();
        }
    }
    // Expect 3 # centered around cols 0, 1, 4, 9, 16, 25;
    let expected = String::from("\
##......................................
###.....................................
...###..................................
........###.............................
...............###......................
........................###.............
");
    let actual = crt.as_multiline_string();
    println!("actual:\n{}", actual);
    println!("expected:\n{}", expected);

    assert_eq!(expected, actual);
}
