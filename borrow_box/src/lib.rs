#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        })
    }

    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 > self.p2.1 {
            return self.p1.clone();
        } else if self.p2.1 > self.p1.1{
            return self.p2.clone();
        } else {
            return ("Same score! tied".to_string(), self.p1.1);
        }        
    }

    pub fn update_score(&mut self, user_name: String) {
        if self.p1.0 == user_name && self.p1.1 < 3 && self.nb_games != 0 {
            self.p1.1 += 1;
            return;
        } else if self.p2.0 == user_name && self.p2.1 < 3 && self.nb_games != 0 {
            self.p2.1 += 1;
            return;
        }
        self.nb_games -= 1;
    }

    pub fn delete(self) -> String {
        let id = self.id;
        format!("game deleted: id -> {}", id)
    }
}
