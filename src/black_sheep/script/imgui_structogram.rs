use imgui::{ImColor32, Ui};

use super::*;

pub struct Structogram {
    pub script: Script,
    placeholder: Option<usize>,
    block_size: f32,
    spacing: f32,
    panel_position: Vector2<f32>,
    dimension: Vector2<f32>,
}

impl Structogram {
    pub fn new(script: Script) -> Self {
        Structogram {
            script,
            placeholder: None,
            block_size: -1.0,
            spacing: -1.0,
            panel_position: Vector2::new(-1.0, -1.0),
            dimension: Vector2::new(-1.0, -1.0),
        }
    }

    pub fn update(&mut self, mouse_pos: Vector2<f32>) {
        let rel_mouse_pos = mouse_pos - self.panel_position;
        let temp = self.dimension - rel_mouse_pos;

        if rel_mouse_pos.x > 0.0 && rel_mouse_pos.y > 0.0 && temp.x > 0.0 && temp.y > 0.0 {
            self.insert_placeholder(rel_mouse_pos);
        } else {
            self.placeholder = None;
        }
    }

    pub fn insert_placeholder(&mut self, mouse_pos: Vector2<f32>) {
        let block_size = self.block_size;
        let spacing = self.spacing;
        let block_size_and_spacing = block_size + spacing;

        let mut cursor = Vector2::new(spacing, spacing);

        let mouse_pos = mouse_pos - Vector2::new(0.0, block_size_and_spacing / 2.0);

        for (instr, i) in self.script.instructions.iter().zip(0..) {
            // Check if the cursor has cought up to the mouse_positon
            // Breaks if the mouse position is inside the box of the curser
            // We dont break if the mouse is on the vertical bar of a loop

            let temp = mouse_pos - cursor;
            if temp.y < 0.0 && temp.x > 0.0 {
                println!("i: {} {} ", i, instr.to_str());
                self.placeholder = Some(i);
                return;
            }

            match instr {
                Instruction::WhileLoop { .. } => {
                    cursor.x += block_size_and_spacing;
                    cursor.y += block_size_and_spacing;
                }
                Instruction::IfCFlow { .. } => {
                    cursor.x += block_size_and_spacing;
                    cursor.y += block_size_and_spacing;
                }
                Instruction::Action { .. } => {
                    cursor.y += block_size_and_spacing;
                }
                Instruction::EndWhileLoop => {
                    cursor.x -= block_size_and_spacing;
                }
                Instruction::EndIfCFlow => {
                    cursor.x -= block_size_and_spacing;
                }
            }
        }
        self.placeholder = Some(self.script.instructions.len())
    }

    pub fn build(&mut self, ui: &Ui) {
        let draw_list = ui.get_window_draw_list();

        let window_pos = Vector2::from(ui.window_pos());
        let top_left = window_pos + Vector2::from(ui.window_content_region_min());
        let bottom_right_border = window_pos + Vector2::from(ui.window_content_region_max());
        let mut cursor = top_left;
        self.panel_position = top_left;
        self.dimension = bottom_right_border - top_left;

        let block_size_and_spacing = ui.text_line_height_with_spacing();
        let block_size = ui.text_line_height();
        self.block_size = block_size;

        let spacing = block_size_and_spacing - block_size;
        self.spacing = spacing;

        let mut debth_stack = debth_stack::DebthStack2::new();

        for (instr, i) in self.script.instructions.iter().zip(0..) {
            if let Some(ph) = self.placeholder {
                if i == ph {
                    draw_list
                        .add_rect(
                            cursor.into(),
                            [bottom_right_border.x, cursor.y + block_size],
                            ImColor32::from_rgba(25, 255, 25, 255),
                        )
                        .filled(true)
                        .build();
                    cursor.y += block_size_and_spacing;
                    debth_stack.advance();
                }
            }

            match instr {
                Instruction::WhileLoop { .. } => {
                    draw_list
                        .add_rect(
                            cursor.into(),
                            [bottom_right_border.x, cursor.y + block_size],
                            ImColor32::from_rgba(255, 0, 255, 255),
                        )
                        .filled(true)
                        .build();

                    ui.set_cursor_pos((cursor - window_pos).into());
                    ui.text("while");

                    cursor.x += block_size_and_spacing;
                    cursor.y += block_size_and_spacing;

                    debth_stack.push();
                    debth_stack.advance();
                }
                Instruction::IfCFlow { .. } => todo!(),
                Instruction::Action { .. } => {
                    draw_list
                        .add_rect(
                            cursor.into(),
                            [bottom_right_border.x, cursor.y + block_size],
                            ImColor32::from_rgba(255, 255, 0, 255),
                        )
                        .filled(true)
                        .build();
                    cursor.y += block_size_and_spacing;
                    debth_stack.advance();
                }
                Instruction::EndWhileLoop => {
                    cursor.x -= block_size_and_spacing;

                    if let Some(h) = debth_stack.pop() {
                        let h = (h - 1) as f32;

                        let mut top_left = cursor;
                        top_left.y -= (h * block_size_and_spacing) + spacing;

                        draw_list
                            .add_rect(
                                top_left.into(),
                                [cursor.x + block_size, cursor.y - spacing],
                                ImColor32::from_rgba(255, 255, 255, 255),
                            )
                            .filled(true)
                            .build();
                    }
                }
                Instruction::EndIfCFlow => {
                    cursor.x -= block_size_and_spacing;
                }
            }
        }
    }
}
