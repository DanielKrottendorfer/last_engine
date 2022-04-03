use imgui::{ImColor32, Ui};

use super::*;

pub struct Structogram {
    pub script: Script,
    placeholder: Option<debth_stack::DebthStack>,
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
            //println!("insert placeholder");
            self.insert_placeholder(rel_mouse_pos);
            if let Some(ph) = &self.placeholder {
                self.script.push_by_debth_stack(ph);
            }
        } else {
            self.placeholder = None;
        }
    }

    pub fn insert_placeholder(&mut self, mouse_pos: Vector2<f32>) {
        let block_size = self.block_size;
        let spacing = self.spacing;
        let block_size_and_spacing = block_size + spacing;

        let mut cursor = Vector2::new(spacing, spacing);
        let len = self.script.instructions.len();

        let mut ds = debth_stack::DebthStack::new();
        ds.push(len);

        for i in 0..self.script.instructions.len() {
            let inst = &self.script.instructions[i];

        }

        for (instr, i) in self.script.instructions.iter().zip(0..) {
            match instr {
                Instruction::WhileLoop(wl) => {
                    let len = wl.length;

                    if len > 0 {
                        cursor.x += block_size_and_spacing;
                        cursor.y += block_size_and_spacing;
                    } else {
                        cursor.x += block_size_and_spacing;
                        cursor.y += block_size_and_spacing * 2.0;
                    }

                    // if i + len + 1 > target_line && target_line < i {
                    //     ds.push(len + 1);
                    // }else
                    {
                        ds.push(len);
                    }

                }
                Instruction::IfCFlow(_) => todo!(),
                Instruction::Action(_) => {
                    cursor.y += block_size_and_spacing;
                }
            }

            // Check if the cursor has cought up to the mouse_positon
            // Breaks if the mouse position is inside the box of the curser
            // We dont break if the mouse is on the vertical bar of a loop

            let temp = mouse_pos - cursor;
            if temp.y < 0.0 && temp.x > 0.0 {
                print!("i: {} ",i);
                break;
            }

            cursor.x -= block_size_and_spacing * ds.advance() as f32;
        }
        println!("{:?}", ds);

        self.placeholder = Some(ds);
    }

    pub fn build(&mut self, ui: &Ui) {
        let draw_list = ui.get_window_draw_list();

        let window_pos = Vector2::from(ui.window_pos());
        let top_left = window_pos + Vector2::from(ui.window_content_region_min());
        let bottom_right = window_pos + Vector2::from(ui.window_content_region_max());
        let mut cursor = top_left;
        self.panel_position = top_left;
        self.dimension = bottom_right - top_left;

        let block_size_and_spacing = ui.text_line_height_with_spacing();
        let block_size = ui.text_line_height();
        self.block_size = block_size;

        let spacing = block_size_and_spacing - block_size;
        self.spacing = spacing;

        let mut debth_stack = debth_stack::DebthStack::new();
        debth_stack.push(self.script.instructions.len());

        let ph_i = if let Some(ds) = &self.placeholder {
            Some(ds.iter().next().unwrap())
        } else {
            None
        };

        for (instr, i) in self.script.instructions.iter().zip(0..) {

            if let Some(ph_i) = ph_i {
                if let Some(first) = debth_stack.iter().next() {
                    if first.0 == ph_i.0 {
                        draw_list
                            .add_rect(
                                cursor.into(),
                                [bottom_right.x, cursor.y + block_size],
                                ImColor32::from_rgba(25, 255, 25, 255),
                            )
                            .filled(true)
                            .build();
                        cursor.y += block_size_and_spacing;
                    }
                }
            }
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

                    let len = wl.length;
                    if len > 0 {
                        debth_stack.push(len);

                        let loop_height = block_size_and_spacing * len as f32;

                        draw_list
                            .add_rect(
                                cursor.into(),
                                [cursor.x + block_size, cursor.y + loop_height],
                                ImColor32::from_rgba(255, 0, 255, 255),
                            )
                            .filled(true)
                            .build();
                        cursor.x += block_size_and_spacing;
                        cursor.y += spacing;
                    } else {
                        draw_list
                            .add_rect(
                                cursor.into(),
                                [cursor.x + block_size, cursor.y + block_size_and_spacing],
                                ImColor32::from_rgba(255, 0, 255, 255),
                            )
                            .filled(true)
                            .build();
                        cursor.y += block_size_and_spacing + spacing;
                    }
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
                    cursor.y += block_size_and_spacing;
                }
            }

            cursor.x -= block_size_and_spacing * debth_stack.advance() as f32;

        }
    }
}
