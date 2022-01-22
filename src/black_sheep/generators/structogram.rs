use cgmath::{Vector2, Vector3};

use crate::black_sheep::{script::Script, rendering::geometry::{self, MeshToken}};

use super::boxes::{Square, SquareComposition};

pub struct Structogram {
    //square_composition: SquareComposition,
    script: Script,
    pub position: Vector2<f32>,
    pub dimension: Vector2<f32>,
    borderthickness: f32,
    padding: f32,

    pub mesh_token: MeshToken,
    
}

impl Structogram {
    pub fn new(script: Script, position: Vector2<f32>) -> Self {
        let mut structogram = Structogram {
            //square_composition: SquareComposition::new(),
            script,
            position,
            dimension: Vector2::new(0.0, 0.0),
            borderthickness: 5.0,
            padding: 2.0,
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
    pub fn compose_squares(&mut self) -> SquareComposition {
        let mut sc = SquareComposition::new();
        let mut cursor = Vector2::new(
            self.borderthickness + self.padding,
            self.borderthickness + self.padding,
        );

        //cursor += self.position;

        let block_height = 50.0;
        let block_width = 400.0;

        let mut debth_stack = Vec::new();

        for instr in self.script.instructions.iter() {
            debth_stack = debth_stack
                .drain(0..)
                .filter_map(|i: usize| {
                    if i == 0 {
                        cursor.x -= block_height + self.padding;
                        None
                    } else {
                        Some(i - 1)
                    }
                })
                .collect();

            match instr {
                crate::black_sheep::script::Instruction::WhileLoop(wl) => {
                    let square = Square::new(
                        cursor,
                        Vector2::new(block_width - cursor.x, block_height),
                        Vector3::new(1.0, 0.0, 0.0),
                    );
                    sc.add_square(square);
                    cursor.y += block_height;

                    let square = Square::new(
                        cursor,
                        Vector2::new(block_height, (block_height + self.padding) * wl.len as f32),
                        Vector3::new(1.0, 0.0, 0.0),
                    );
                    sc.add_square(square);
                    cursor.x += block_height + self.padding;

                    debth_stack.push(wl.len);
                }
                crate::black_sheep::script::Instruction::IfCFlow(_) => todo!(),
                crate::black_sheep::script::Instruction::Action(_) => {
                    let square = Square::new(
                        cursor,
                        Vector2::new(block_width - cursor.x, block_height),
                        Vector3::new(0.0, 1.0, 0.0),
                    );
                    sc.add_square(square);
                    cursor.y += block_height;
                }
            }

            cursor.y += self.padding;
        }

        sc
    }
}
