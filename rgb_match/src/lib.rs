pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match (first, second) {
            (b'r', b'g') | (b'g', b'r') => {
                let temp = self.r;
                self.r = self.g;
                self.g = temp;
            }
            (b'r', b'b') | (b'b', b'r') => {
                let temp = self.r;
                self.r = self.b;
                self.b = temp;
            }
            (b'r', b'a') | (b'a', b'r') => {
                let temp = self.r;
                self.r = self.a;
                self.a = temp;
            }
            (b'g', b'b') | (b'b', b'g') => {
                let temp = self.g;
                self.g = self.b;
                self.b = temp;
            }
            (b'g', b'a') | (b'a', b'g') => {
                let temp = self.g;
                self.g = self.a;
                self.a = temp;
            }
            (b'b', b'a') | (b'a', b'b') => {
                let temp = self.b;
                self.b = self.a;
                self.a = temp;
            }
            _ => {} // Si les composantes ne correspondent pas, ne rien faire.
        }
        self // Renvoyer la structure Color mise à jour.
    }
}