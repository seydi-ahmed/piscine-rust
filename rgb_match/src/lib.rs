#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        // Vérifiez d'abord si les valeurs fournies sont valides
        if first > 3 || second > 3 {
            panic!("Invalid color component index");
        }

        // Utilisez un modèle de correspondance pour échanger les valeurs
        match (first, second) {
            (0, 1) => { let temp = self.r; self.r = self.g; self.g = temp; },
            (0, 2) => { let temp = self.r; self.r = self.b; self.b = temp; },
            (0, 3) => { let temp = self.r; self.r = self.a; self.a = temp; },
            (1, 2) => { let temp = self.g; self.g = self.b; self.b = temp; },
            (1, 3) => { let temp = self.g; self.g = self.a; self.a = temp; },
            (2, 3) => { let temp = self.b; self.b = self.a; self.a = temp; },
            // Si les indices sont les mêmes, il n'y a pas besoin d'échanger, donc retournez simplement la couleur inchangée
            _ => {},
        }
        
        self // Retourne la couleur modifiée
    }
}
