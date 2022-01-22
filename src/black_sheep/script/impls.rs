use cgmath::InnerSpace;

use super::*;

#[derive(Debug, Clone)]
pub struct Warrior {
    pub position: Vector2<f32>,
    pub speed: f32,
}
impl Warrior {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
            speed: 0.0,
        }
    }
}
impl GameObject for Warrior {
    fn get_position(&self) -> Vector2<f32> {
        self.position
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
    fn print(&self) {
        println!("{:?}", self);
    }

    fn as_movable(&mut self) -> Option<&mut dyn Movable> {
        Some(self as &mut dyn Movable)
    }

    fn box_it(self) -> Box<dyn GameObject> {
        Box::new(self)
    }
}
impl Movable for Warrior {
    fn move_to(&mut self, dest: Vector2<f32>) {
        let v = dest - self.position;
        let mag = v.magnitude();

        #[cfg(not(feature = "debug_off"))]
        print!("move from {:?}", self.position);

        if mag < self.speed {
            self.position = dest
        } else {
            self.position += (v / mag) * self.speed;
        }

        #[cfg(not(feature = "debug_off"))]
        println!(" to {:?}", self.position);
    }
}

#[derive(Debug, Clone)]
pub struct Mark {
    pub position: Vector2<f32>,
}
impl Mark {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
        }
    }
}
impl GameObject for Mark {
    fn get_position(&self) -> Vector2<f32> {
        self.position
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn as_movable(&mut self) -> Option<&mut dyn Movable> {
        None
    }

    fn print(&self) {
        println!("{:?}", self);
    }

    fn box_it(self) -> Box<dyn GameObject> {
        Box::new(self)
    }
}

#[derive(Clone)]
pub struct MoveAtoB {
    a: String,
    b: String,
}
impl MoveAtoB {
    pub fn new(a: String, b: String) -> Self {
        MoveAtoB { a, b }
    }
}
impl Action for MoveAtoB {
    fn act(&self, vars: &mut HashMap<String, Box<dyn GameObject>>) -> Option<()> {
        let p = vars.get(&self.b)?.get_position();
        let a = vars.get_mut(&self.a)?;

        if let Some(mv) = a.as_movable() {
            mv.move_to(p);
        }
        None
    }

    fn into_instruction(self) -> Instruction {
        Instruction::Action(Box::new(self))
    }
}

pub struct Not {
    condition: Box<dyn Condition>,
}

pub fn not(condition: Box<dyn Condition>) -> Box<dyn Condition> {
    Not::new(condition).box_it()
}
impl Not {
    pub fn new(condition: Box<dyn Condition>) -> Self {
        Not { condition }
    }
}
impl Condition for Not {
    fn is_met(&mut self, vars: &HashMap<String, Box<dyn GameObject>>) -> Option<bool> {
        self.condition.is_met(vars).map(|b| !b)
    }

    fn box_it(self) -> Box<dyn Condition> {
        Box::new(self)
    }
}

pub struct IsCloseEnough {
    a: String,
    b: String,
    dist: f32,
}
impl IsCloseEnough {
    pub fn new(a: String, b: String, dist: f32) -> Self {
        IsCloseEnough { a, b, dist }
    }
}
impl Condition for IsCloseEnough {
    fn is_met(&mut self, vars: &HashMap<String, Box<dyn GameObject>>) -> Option<bool> {
        let a = vars.get(&self.a)?.get_position();
        let b = vars.get(&self.b)?.get_position();

        if (a - b).magnitude() < self.dist {
            Some(true)
        } else {
            Some(false)
        }
    }

    fn box_it(self) -> Box<dyn Condition> {
        Box::new(self)
    }
}

pub struct Iterations(usize);
impl Iterations {
    pub fn new(i: usize) -> Self {
        Iterations(i)
    }
}
impl Condition for Iterations {
    fn is_met(&mut self, _vars: &HashMap<String, Box<dyn GameObject>>) -> Option<bool> {
        if self.0 > 0 {
            println!("GZ {}", self.0);
            self.0 -= 1;
            Some(true)
        } else {
            Some(false)
        }
    }

    fn box_it(self) -> Box<dyn Condition> {
        Box::new(self)
    }
}
