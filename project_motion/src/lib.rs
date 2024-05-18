
const GRAVITY: f32 = 9.8;

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }

    fn get_position(&mut self) -> Object {
        let new_x = self.init_position.x + self.init_velocity.x * self.time;
        let new_y = self.init_position.y + self.init_velocity.y * self.time
            - 0.5 * GRAVITY * self.time.powi(2);

        let new_x = (new_x * 10.0).round() / 10.0;
        let new_y = (new_y * 10.0).round() / 10.0;

        Object { x: new_x, y: new_y }
    }
    fn velocity(&mut self) -> Object {
        let new_vx = self.init_velocity.x;
        let new_vy = self.init_velocity.y - GRAVITY * self.time;

        let new_vx = (new_vx * 10.0).round() / 10.0;
        let new_vy = (new_vy * 10.0).round() / 10.0;

        Object {
            x: new_vx,
            y: new_vy,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        // Update time
        self.time += 1.0; // Assuming 1 second time step

        self.actual_position = self.get_position();
        self.actual_velocity = self.velocity();

        if self.actual_position.y <= 0.0 {
            return None;
        }

        Some(self.clone())
    }
}