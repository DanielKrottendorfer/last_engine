use cgmath::Vector2;

use std::{any::Any, collections::HashMap};
pub mod debth_stack;
pub mod imgui_structogram;
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

pub enum Instruction {
    WhileLoop { condition: Box<dyn Condition> },
    EndWhileLoop,
    IfCFlow { condition: Box<dyn Condition> },
    EndIfCFlow,
    Action { action: Box<dyn Action> },
}

impl Instruction {
    pub fn to_str(&self) -> &'static str {
        match self {
            Instruction::WhileLoop { .. } => "WhileLoop",
            Instruction::IfCFlow { .. } => "IfCFlow",
            Instruction::Action { .. } => "Action",
            Instruction::EndWhileLoop => "EndWhileLoop",
            Instruction::EndIfCFlow => "EndIfCFlow",
        }
    }
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

    fn push_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    pub fn add_game_object(&mut self, name: String, game_object: Box<dyn GameObject>) {
        self.variables.insert(name, game_object);
    }

    pub fn print(&self) {
        for s in self.instructions.iter() {
            print!("{}, ", s.to_str());
        }
        println!();
    }
}

use impls::*;

pub fn init_script2() -> Script {
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

    script.push_instruction(
        MoveAtoB::new("warrior".to_string(), "mark1".to_string()).into_instruction(),
    );
    script.push_instruction(Instruction::WhileLoop {
        condition: Iterations::new(5).box_it(),
    });

    script.push_instruction(
        MoveAtoB::new("warrior".to_string(), "mark1".to_string()).into_instruction(),
    );
    script.push_instruction(
        MoveAtoB::new("warrior".to_string(), "mark1".to_string()).into_instruction(),
    );
    script.push_instruction(Instruction::WhileLoop {
        condition: Iterations::new(5).box_it(),
    });

    script.push_instruction(
        MoveAtoB::new("warrior".to_string(), "mark1".to_string()).into_instruction(),
    );
    script.push_instruction(
        MoveAtoB::new("warrior".to_string(), "mark1".to_string()).into_instruction(),
    );
    script.push_instruction(Instruction::EndWhileLoop);
    script.push_instruction(Instruction::EndWhileLoop);

    script.push_instruction(Instruction::WhileLoop {
        condition: Iterations::new(5).box_it(),
    });
    script.push_instruction(Instruction::EndWhileLoop);

    script.push_instruction(
        MoveAtoB::new("warrior".to_string(), "mark1".to_string()).into_instruction(),
    );

    script.push_instruction(
        MoveAtoB::new("warrior".to_string(), "mark1".to_string()).into_instruction(),
    );
    script
}
