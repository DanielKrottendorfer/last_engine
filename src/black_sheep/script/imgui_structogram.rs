use imgui::{ImColor32, Ui};

use super::*;

pub struct Structogram {
    script: Script,
    block_size: f32,
    spacing: f32,
    panel_position: Vector2<f32>,
    dimension: Vector2<f32>,
}

impl Structogram {
    pub fn new(script: Script) -> Self {
        Structogram {
            script,
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
            if self.try_insert_placeholder(rel_mouse_pos) {
                self.script.print();
            }
        }
    }

    pub fn try_insert_placeholder(&mut self, mouse_pos: Vector2<f32>) -> bool {
        let block_size = self.block_size;
        let spacing = self.spacing;

        let mut cursor = self.panel_position + Vector2::new(spacing, spacing);

        let mut debth_stack = vec![self.script.instructions.len()];

        let init_placeholder_index = self
            .script
            .instructions
            .iter()
            .position(|x| x.is_placeholder());

        for instr in self.script.instructions.iter() {
            match instr {
                Instruction::WhileLoop(wl) => {
                    cursor.y += block_size + spacing;
                    cursor.x += block_size + spacing;
                    debth_stack.push(wl.len);
                }
                Instruction::IfCFlow(_) => todo!(),
                Instruction::Action(_) => {
                    cursor.y += block_size + spacing;
                }
                Instruction::Placeholder => {
                    cursor.y += block_size + spacing;
                }
            }

            // Check if the cursor has cought up to the mouse_positon
            // Breaks if the mouse position is inside the box of the curser
            // We dont break if the mouse is on the vertical bar of a loop

            let temp = mouse_pos - cursor;
            if temp.y < 0.0 && temp.x > 0.0 {
                break;
            }

            debth_stack = debth_stack
                .drain(0..)
                .filter_map(|i: usize| {
                    if i == 0 {
                        cursor.x -= block_size;
                        None
                    } else {
                        Some(i - 1)
                    }
                })
                .collect();
        }

        let ii = self.script.instructions.len() - debth_stack.first().unwrap();

        if ii < self.script.instructions.len() {
            if let Some(i) = init_placeholder_index {
                if i == ii {
                    false
                } else {
                    self.script.insert_placeholder(ii);
                    true
                }
            } else {
                self.script.insert_placeholder(ii);
                true
            }
        } else {
            false
        }
    }

    pub fn build(&mut self, ui: &Ui) {
        let draw_list = ui.get_window_draw_list();

        let window_pos = Vector2::from(ui.window_pos());
        let top_left = window_pos + Vector2::from(ui.window_content_region_min());
        let bottom_right = window_pos + Vector2::from(ui.window_content_region_max());
        let mut cursor = top_left;
        self.panel_position = top_left;
        self.dimension = bottom_right - top_left;

        let block_size_with_spacing = ui.text_line_height_with_spacing();
        let block_size = ui.text_line_height();
        self.block_size = block_size;

        let spacing = block_size_with_spacing - block_size;
        self.spacing = spacing;

        let mut debth_stack = Vec::new();

        for instr in self.script.instructions.iter() {
            match instr {
                Instruction::WhileLoop(wl) => {
                    draw_list
                        .add_rect(
                            cursor.into(),
                            [bottom_right.x, cursor.y + block_size],
                            ImColor32::from_rgba(255, 0, 255, 255),
                        )
                        .filled(true)
                        .build();

                    ui.set_cursor_pos((cursor - window_pos).into());
                    ui.text("while");

                    cursor.y += block_size;

                    let loop_heith = (block_size_with_spacing * wl.len as f32);
                    draw_list
                        .add_rect(
                            cursor.into(),
                            [cursor.x + block_size, cursor.y + loop_heith],
                            ImColor32::from_rgba(255, 0, 255, 255),
                        )
                        .filled(true)
                        .build();

                    cursor.x += block_size_with_spacing;

                    debth_stack.push(wl.len);
                }
                Instruction::IfCFlow(_) => todo!(),
                Instruction::Action(_) => {
                    draw_list
                        .add_rect(
                            cursor.into(),
                            [bottom_right.x, cursor.y + block_size],
                            ImColor32::from_rgba(255, 255, 0, 255),
                        )
                        .filled(true)
                        .build();
                    cursor.y += block_size;
                }
                Instruction::Placeholder => {
                    draw_list
                        .add_rect(
                            cursor.into(),
                            [bottom_right.x, cursor.y + block_size],
                            ImColor32::from_rgba(1, 2, 1, 255),
                        )
                        .filled(true)
                        .build();
                    cursor.y += block_size;
                }
            }
            cursor.y += spacing;

            debth_stack = debth_stack
                .drain(0..)
                .filter_map(|i: usize| {
                    if i == 0 {
                        cursor.x -= block_size_with_spacing;
                        None
                    } else {
                        Some(i - 1)
                    }
                })
                .collect();
        }
    }
}
