#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let color = self.clone();

        let (r, g, b, a) = (color.r, color.g, color.b, color.a);
        let swap_result = match (first, second) {
            (r1, r2) if (r, g) == (r1, r2) || (g, r) == (r1, r2) => Color {r: g, g: r, b, a},
            (r1, r2) if (r, b) == (r1, r2) || (b, r) == (r1, r2) => Color {r: b, g: b, r, a},
            (r1, r2) if (r, a) == (r1, r2) || (a, r) == (r1, r2) => Color {r: a, g: b, a, r},
            (g1, g2) if (g, b) == (g1, g2) || (b, g) == (g1, g2) => Color {r: g, b: b, g, a},
            (g1, g2) if (g, a) == (g1, g2) || (a, g) == (g1, g2) => Color {r: g, a: b, a, g},
            (b1, b2) if (b, a) == (b1, b2) || (a, b) == (b1, b2) => Color {r: g, b: a, a, b},
            _ => self,
        };

        self = swap_result;
        self
    }
}



// Color { r: 10, g: 200, b: 255, a: 30 }
// Color { r: 200, g: 255, b: 10, a: 30 }
// Color { r: 30, g: 200, b: 10, a: 255 }

// Color { r: 200, g: 255, b: 10, a: 30 }
// Color { r: 255, g: 10, b: 200, a: 30 }
// Color { r: 255, g: 30, b: 10, a: 200 }

// Color { r: 10, g: 200, b: 255, a: 30 }
// Color { r: 255, g: 10, b: 200, a: 30 }
// Color { r: 255, g: 200, b: 30, a: 10 }

// Color { r: 30, g: 200, b: 10, a: 255 }
// Color { r: 255, g: 200, b: 30, a: 10 }
// Color { r: 255, g: 30, b: 10, a: 200 }