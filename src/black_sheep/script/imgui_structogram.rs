use imgui::{ImColor32, Ui};

use super::*;

pub struct Structogram {
    script: Script,
}

impl Structogram {
    pub fn new(script: Script) -> Self {
        Structogram{
            script
        }
    }
    pub fn build(&self, ui: &Ui) {
        let mut cursor = Vector2::from(ui.window_content_region_min());
        cursor += Vector2::from( ui.window_pos());

        let right_border = ui.window_content_region_max()[0];

        let block_thickness_with_spacing = ui.text_line_height_with_spacing();
        let block_thickness = ui.text_line_height();
        let spacing = block_thickness_with_spacing - block_thickness;

        let mut debth_stack = Vec::new();

        let draw_list = ui.get_window_draw_list();

        for instr in self.script.instructions.iter() {
            match instr {
                Instruction::WhileLoop(wl) => {
                    // let square = Square::new(
                    //     cursor,
                    //     Vector2::new(self.block_width - cursor.x, block_thickness),
                    //     Vector3::new(1.0, 0.0, 0.0),
                    // );
                    // sc.add_square(square);

                    draw_list
                        .add_rect(
                            cursor.into(),
                            [right_border, cursor.y + block_thickness],
                            ImColor32::from_rgba(255, 0, 255, 255),
                        )
                        .build();

                    println!("add rect \n {:?}",cursor);
                    cursor.y += block_thickness;

                    // let square = Square::new(
                    //     cursor,
                    //     Vector2::new(block_thickness, (ph) * wl.len as f32),
                    //     Vector3::new(1.0, 0.0, 0.0),
                    // );
                    // sc.add_square(square);
                    cursor.x += block_thickness_with_spacing;

                    debth_stack.push(wl.len);
                }
                Instruction::IfCFlow(_) => todo!(),
                Instruction::Action(_) => {
                    // let square = Square::new(
                    //     cursor,
                    //     Vector2::new(self.block_width - cursor.x, block_thickness),
                    //     Vector3::new(0.0, 1.0, 0.0),
                    // );
                    // sc.add_square(square);
                    cursor.y += block_thickness;
                }
                Instruction::Placeholder => {
                    // let square = Square::new(
                    //     cursor,
                    //     Vector2::new(self.block_width - cursor.x, block_thickness),
                    //     Vector3::new(0.0, 0.0, 0.0),
                    // );
                    // sc.add_square(square);
                    cursor.y += block_thickness;
                }
            }
            cursor.y += spacing;

            debth_stack = debth_stack
                .drain(0..)
                .filter_map(|i: usize| {
                    if i == 0 {
                        cursor.x -= block_thickness_with_spacing;
                        None
                    } else {
                        Some(i - 1)
                    }
                })
                .collect();
        }
    }
}
