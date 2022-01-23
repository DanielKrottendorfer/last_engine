use cgmath::{Vector2, Vector3};

use crate::black_sheep::{
    generators::squares::{Square, SquareComposition},
    rendering::geometry::{self, MeshToken},
    script::{Instruction, Script},
};

pub struct Structogram {
    //square_composition: SquareComposition,
    script: Script,
    pub position: Vector2<f32>,
    pub dimension: Vector2<f32>,
    border: f32,
    padding: f32,

    block_height: f32,
    block_width: f32,

    pub mesh_token: MeshToken,
}

impl Structogram {
    pub fn new(script: Script, position: Vector2<f32>) -> Self {
        let mut structogram = Structogram {
            script,
            position,
            dimension: Vector2::new(0.0, 0.0),
            border: 5.0,
            padding: 5.0,
            block_height: 50.0,
            block_width: 400.0,
            mesh_token: MeshToken::default(),
        };

        structogram.mesh_token = geometry::get_mesh_repo(|mesh_repo| {
            let ss = structogram.compose_squares();
            let structo = mesh_repo.add_mesh("structogram", |mesh| {
                let vc = ss.generate_colored_triangles();
                mesh.add_floatbuffer(&vc.0, 0, 2);
                mesh.add_floatbuffer(&vc.1, 1, 3);
                mesh.add_elementarraybuffer(&vc.2);
            });
            structo
        });

        structogram
    }

    pub fn update_mesh(&mut self) {
        self.mesh_token = geometry::get_mesh_repo(|mesh_repo| {
            mesh_repo.remove_mesh("structogram");
            let ss = self.compose_squares();
            let structo = mesh_repo.add_mesh("structogram", |mesh| {
                let vc = ss.generate_colored_triangles();
                mesh.add_floatbuffer(&vc.0, 0, 2);
                mesh.add_floatbuffer(&vc.1, 1, 3);
                mesh.add_elementarraybuffer(&vc.2);
            });
            structo
        });
    }

    pub fn update(&mut self, mouse_pos: Vector2<f32>) {
        let rel_mouse_pos = mouse_pos - self.position;
        let t = self.dimension - rel_mouse_pos;

        let ph = self.padding + self.block_height;

        if rel_mouse_pos.x < 0.0 || rel_mouse_pos.y < 0.0 || t.x < 0.0 || t.y < 0.0 {
            if let Some(p) = self
                .script
                .instructions
                .iter()
                .position(|x| x.is_placeholder())
            {
                self.script.instructions.remove(p);
                self.update_mesh();
            }
        } else {
            let mut cursor = self.position
                + Vector2::new(self.border + self.padding, self.border + self.padding);

            let mut debth_stack = Vec::new();

            debth_stack.push(self.script.instructions.len());
            for instr in self.script.instructions.iter() {
                match instr {
                    Instruction::WhileLoop(wl) => {
                        cursor.y += self.block_height;
                        cursor.x += ph;
                        debth_stack.push(wl.len);
                    }
                    Instruction::IfCFlow(_) => todo!(),
                    Instruction::Action(_) => {
                        cursor.y += self.block_height;
                    }
                    Instruction::Placeholder => {
                        cursor.y += self.block_height;
                    }
                }

                cursor.y += self.padding;

                debth_stack = debth_stack
                    .drain(0..)
                    .filter_map(|i: usize| {
                        if i == 0 {
                            cursor.x -= ph;
                            None
                        } else {
                            Some(i - 1)
                        }
                    })
                    .collect();

                if rel_mouse_pos.y - cursor.y < self.block_height {
                    let debth = (rel_mouse_pos.x / ph) as usize + 2;
                    if debth > debth_stack.len() {
                        break;
                    }
                }
            }

            let ii = self.script.instructions.len() - debth_stack.first().unwrap();
            self.script.insert_instruction(ii, Instruction::Placeholder);
            self.update_mesh();
        }
    }

    pub fn compose_squares(&mut self) -> SquareComposition {
        let mut sc = SquareComposition::new();
        let mut cursor =
            self.position + Vector2::new(self.border + self.padding, self.border + self.padding);

        let mut debth_stack = Vec::new();
        let ph = self.padding + self.block_height;

        for instr in self.script.instructions.iter() {
            debth_stack = debth_stack
                .drain(0..)
                .filter_map(|i: usize| {
                    if i == 0 {
                        cursor.x -= ph;
                        None
                    } else {
                        Some(i - 1)
                    }
                })
                .collect();

            match instr {
                Instruction::WhileLoop(wl) => {
                    let square = Square::new(
                        cursor,
                        Vector2::new(self.block_width - cursor.x, self.block_height),
                        Vector3::new(1.0, 0.0, 0.0),
                    );
                    sc.add_square(square);
                    cursor.y += self.block_height;

                    let square = Square::new(
                        cursor,
                        Vector2::new(self.block_height, (ph) * wl.len as f32),
                        Vector3::new(1.0, 0.0, 0.0),
                    );
                    sc.add_square(square);
                    cursor.x += ph;

                    debth_stack.push(wl.len);
                }
                Instruction::IfCFlow(_) => todo!(),
                Instruction::Action(_) => {
                    let square = Square::new(
                        cursor,
                        Vector2::new(self.block_width - cursor.x, self.block_height),
                        Vector3::new(0.0, 1.0, 0.0),
                    );
                    sc.add_square(square);
                    cursor.y += self.block_height;
                }
                Instruction::Placeholder => {
                    let square = Square::new(
                        cursor,
                        Vector2::new(self.block_width - cursor.x, self.block_height),
                        Vector3::new(0.0, 0.0, 0.0),
                    );
                    sc.add_square(square);
                    cursor.y += self.block_height;
                }
            }

            cursor.y += self.padding;
        }

        self.dimension = Vector2::new(self.block_width, cursor.y);

        sc
    }
}
