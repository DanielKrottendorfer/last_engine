use cgmath::Vector2;
use std::{any::Any, collections::HashMap};
pub mod impls;

pub trait Movable {
    fn move_to(&mut self, dir: Vector2<f32>);
}
pub trait GameObject {
    fn get_position(&self) -> Vector2<f32>;
    fn as_any(&mut self) -> &mut dyn Any;
    fn as_movable(&mut self) -> Option<&mut dyn Movable>;
    fn box_it(self) -> Box<dyn GameObject>;
    fn print(&self);
}
pub trait Action {
    fn act(&self, vars: &mut HashMap<String, Box<dyn GameObject>>) -> Option<()>;
    fn into_instruction(self) -> Instruction;
}
pub trait Condition {
    fn is_met(&mut self, vars: &HashMap<String, Box<dyn GameObject>>) -> Option<bool>;
    fn box_it(self) -> Box<dyn Condition>;
}
pub struct WhileLoop {
    pub len: usize,
    pub condition: Box<dyn Condition>,
}

impl WhileLoop {
    pub fn new(len: usize, condition: Box<dyn Condition>) -> Self {
        WhileLoop { len, condition }
    }
    pub fn into_instruction(self) -> Instruction {
        Instruction::WhileLoop(self)
    }
}
pub struct IfCFlow {
    pub len: usize,
    pub condition: Box<dyn Condition>,
}

impl IfCFlow {
    pub fn new(len: usize, condition: Box<dyn Condition>) -> Self {
        Self { len, condition }
    }
    pub fn into_instruction(self) -> Instruction {
        Instruction::IfCFlow(self)
    }
}
pub enum Instruction {
    WhileLoop(WhileLoop),
    IfCFlow(IfCFlow),
    Action(Box<dyn Action>),
}

#[derive(Default)]
pub struct Script {
    pub variables: HashMap<String, Box<dyn GameObject>>,
    pub instructions: Vec<Instruction>,
}
impl Script {
    pub fn new() -> Self {
        Script::default()
    }
    pub fn push_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }
    pub fn add_game_object(&mut self, name: String, game_object: Box<dyn GameObject>) {
        self.variables.insert(name, game_object);
    }
}

pub fn run_script(script: &mut Script) {
    let len = script.instructions.len();
    // sub_run(script, 0, len);

    let mut scope_stack = vec![(0 as usize, len)];

    while let Some(scope) = scope_stack.pop() {
        let (start, mut ende) = scope;

        let mut i = start;

        while i < ende {
            match &mut script.instructions[i] {
                Instruction::WhileLoop(wl) => {
                    if let Some(b) = wl.condition.is_met(&mut script.variables) {
                        if b {
                            scope_stack.push((i, ende));
                            ende = i + wl.len + 1;
                            i += 1;
                        } else {
                            i += 1 + wl.len;
                        }
                    } else {
                        print!("oh nooo");
                        return;
                    }
                }
                Instruction::IfCFlow(cf) => {
                    if let Some(b) = cf.condition.is_met(&mut script.variables) {
                        if b {
                            i += 1;
                        } else {
                            i += 1 + cf.len;
                        }
                    } else {
                        print!("oh nooo");
                        return;
                    }
                }
                Instruction::Action(a) => {
                    a.act(&mut script.variables);
                    i += 1;
                }
            }
        }
    }
}

use impls::*;

pub fn init_script() -> Script {
    let mut warrior = Warrior::new();
    warrior.position = Vector2::new(0.0, -1.0);
    warrior.speed = 1.0;

    let mut mark1 = Mark::new();
    mark1.position = Vector2::new(1.0, 1.0);

    let mut mark2 = Mark::new();
    mark2.position = Vector2::new(-1.0, 1.0);

    let mut script = Script::new();
    script.add_game_object("warrior".to_string(), warrior.box_it());
    script.add_game_object("mark1".to_string(), mark1.box_it());
    script.add_game_object("mark2".to_string(), mark2.box_it());

    script.push_instruction(WhileLoop::new(4, Iterations::new(5).box_it()).into_instruction());
    script.push_instruction(
        WhileLoop::new(
            1,
            not(IsCloseEnough::new("warrior".to_string(), "mark1".to_string(), 0.1).box_it()),
        )
        .into_instruction(),
    );
    script.push_instruction(
        MoveAtoB::new("warrior".to_string(), "mark1".to_string()).into_instruction(),
    );

    script.push_instruction(
        WhileLoop::new(
            1,
            not(IsCloseEnough::new("warrior".to_string(), "mark2".to_string(), 0.1).box_it()),
        )
        .into_instruction(),
    );
    script.push_instruction(
        MoveAtoB::new("warrior".to_string(), "mark2".to_string()).into_instruction(),
    );
    script.push_instruction(
        MoveAtoB::new("warrior".to_string(), "mark2".to_string()).into_instruction(),
    );

    script
}

