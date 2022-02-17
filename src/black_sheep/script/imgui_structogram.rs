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

        let window_pos = Vector2::from(ui.window_pos());
        let top_left = window_pos + Vector2::from(ui.window_content_region_min());
        let bottom_right = window_pos + Vector2::from(ui.window_content_region_max());
        let mut cursor = top_left;

        let content_region_max = ui.window_content_region_max();

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
                            [bottom_right.x, cursor.y + block_thickness],
                            ImColor32::from_rgba(255, 0, 255, 255),
                        )
                        .build();
                    
                    ui.set_cursor_pos((cursor - window_pos).into());

                    ui.text("while");

                    cursor.y += block_thickness;

                    draw_list
                        .add_rect(
                            cursor.into(),
                            [cursor.x + block_thickness, cursor.y + (block_thickness_with_spacing * wl.len as f32)],
                            ImColor32::from_rgba(255, 0, 255, 255),
                        )
                        .build();
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
