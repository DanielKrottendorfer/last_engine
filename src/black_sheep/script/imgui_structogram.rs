use imgui::{DrawListMut, ImColor32, Ui};

use super::{debth_stack::DebthStack, *};

pub struct StructogramDrawer<'a> {
    cursor: Vector2<f32>,
    right_border: f32,
    block_size: f32,
    spacing: f32,
    block_size_and_spacing: f32,
    window_pos: Vector2<f32>,
    ui: &'a Ui<'a>,
    draw_list: DrawListMut<'a>,
    debth_stack: DebthStack,
}

impl<'a> StructogramDrawer<'a> {
    pub fn draw_placeholder(&mut self) {
        self.draw_list
            .add_rect(
                self.cursor.into(),
                [self.right_border, self.cursor.y + self.block_size],
                ImColor32::from_rgba(25, 255, 25, 255),
            )
            .filled(true)
            .build();
        self.cursor.y += self.block_size_and_spacing;
        self.debth_stack.advance();
    }
    pub fn draw_instuction(&mut self, inst: &Instruction) {
        match inst {
            Instruction::WhileLoop { condition: _ } => {
                self.draw_list
                    .add_rect(
                        self.cursor.into(),
                        [self.right_border, self.cursor.y + self.block_size],
                        ImColor32::from_rgba(255, 0, 255, 255),
                    )
                    .filled(true)
                    .build();

                self.ui.set_cursor_pos(
                    (self.cursor - self.window_pos + Vector2::new(5.0, 0.0)).into(),
                );
                self.ui.text("while");

                self.cursor.x += self.block_size_and_spacing;
                self.cursor.y += self.block_size_and_spacing;
                self.debth_stack.push();
                self.debth_stack.advance();
            }
            Instruction::EndWhileLoop => {
                self.cursor.x -= self.block_size_and_spacing;

                if let Some(h) = self.debth_stack.pop() {
                    let h = (h - 1) as f32;

                    let mut top_left = self.cursor;
                    top_left.y -= (h * self.block_size_and_spacing) + self.spacing;

                    self.draw_list
                        .add_rect(
                            top_left.into(),
                            [
                                self.cursor.x + self.block_size,
                                self.cursor.y - self.spacing,
                            ],
                            ImColor32::from_rgba(255, 0, 255, 255),
                        )
                        .filled(true)
                        .build();
                }
            }
            Instruction::IfCFlow { condition: _ } => todo!(),
            Instruction::EndIfCFlow => todo!(),
            Instruction::Action { action: _ } => {
                self.draw_list
                    .add_rect(
                        self.cursor.into(),
                        [self.right_border, self.cursor.y + self.block_size],
                        ImColor32::from_rgba(255, 255, 0, 255),
                    )
                    .filled(true)
                    .build();
                self.cursor.y += self.block_size_and_spacing;
                self.debth_stack.advance();
            }
        }
    }
}

pub struct Structogram {
    pub script: Script,
    insert_index: Option<usize>,
    block_size: f32,
    spacing: f32,
    panel_position: Vector2<f32>,
}

impl Structogram {
    pub fn new(script: Script) -> Self {
        Structogram {
            script,
            insert_index: None,
            block_size: -1.0,
            spacing: -1.0,
            panel_position: Vector2::new(-1.0, -1.0),
        }
    }

    pub fn build(&mut self, ui: &Ui) {
        let draw_list = ui.get_window_draw_list();
        let window_pos = Vector2::from(ui.window_pos());
        let top_left = ui.cursor_screen_pos().into();
        self.panel_position = top_left;

        let right_border = ui.content_region_max()[0] + window_pos.x;

        let block_size_and_spacing = ui.text_line_height_with_spacing();
        let block_size = ui.text_line_height();
        self.block_size = block_size;

        let spacing = block_size_and_spacing - block_size;
        self.spacing = spacing;

        let mouse_pos: Vector2<f32> = ui.mouse_pos_on_opening_current_popup().into();
        let mouse_pos = mouse_pos - Vector2::new(0.0, block_size);

        let top_left = ui.cursor_screen_pos().into();

        let mut drawer = StructogramDrawer {
            cursor: top_left,
            right_border,
            block_size,
            spacing,
            block_size_and_spacing,
            window_pos,
            ui,
            draw_list,
            debth_stack: DebthStack::new(),
        };

        self.insert_index = None;

        for (instr, i) in self.script.instructions.iter().zip(0..) {
            let temp = mouse_pos - drawer.cursor;
            if temp.y < 0.0 && temp.x > 0.0 && self.insert_index.is_none() {
                self.insert_index = Some(i);
                drawer.draw_placeholder();
            }
            drawer.draw_instuction(instr);
        }

        let temp = mouse_pos - drawer.cursor;
        if temp.y < 0.0 && temp.x > 0.0 && self.insert_index.is_none() {
            self.insert_index = Some(self.script.instructions.len());
            drawer.draw_placeholder();
        }

        ui.set_cursor_pos((drawer.cursor - window_pos).into());
    }
}
