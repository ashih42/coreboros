use macroquad::prelude::*;

use crate::{
    game_context::renderer::color,
    instruction::opcode::Opcode,
    mars::{Mars, address::Address, config::core_dimension::CoreDimension},
    scene::arena::rendering_utils,
    warrior::warrior_id::WarriorId,
};

/// `RingRenderer` draws the entire core and tasks to the central game area with a grid view.
pub struct GridRenderer {
    num_columns: usize,
    num_rows: usize,
    cell_width: f32,
    cell_height: f32,
}

impl GridRenderer {
    #[allow(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "These operations are safe."
    )]
    pub fn new(core_dimension: CoreDimension, game_area_width: f32, game_area_height: f32) -> Self {
        let (num_columns, num_rows) = core_dimension.as_grid_dimensions();

        let cell_width = game_area_width / (num_columns as f32);
        let cell_height = game_area_height / (num_rows as f32);

        Self {
            num_columns,
            num_rows,
            cell_width,
            cell_height,
        }
    }

    pub fn render(&self, mars: &Mars, selected_address: Address) {
        self.draw_core(mars, selected_address);
        self.draw_tasks(mars);
    }

    /// Draw all cores with colors indicating instruction author at each cell,
    /// and draw the selected cell with a thicker border.
    #[allow(
        clippy::arithmetic_side_effects,
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "These operations are safe."
    )]
    fn draw_core(&self, mars: &Mars, selected_address: Address) {
        const BORDER_THICKNESS: f32 = 1.0;
        const SELECTED_BORDER_THICKNESS: f32 = 4.0;
        const BORDER_COLOR: macroquad::color::Color = WHITE;

        for x in 0..self.num_columns {
            for y in 0..self.num_rows {
                let address = y * self.num_columns + x;

                let cell = mars.core.get_cell(address);
                let cell_color = color::get_mq_color(cell.operation_author.into());

                let x = (x as f32) * self.cell_width;
                let y = (y as f32) * self.cell_height;

                draw_rectangle(x, y, self.cell_width, self.cell_height, cell_color);

                let thickness = if address == selected_address {
                    SELECTED_BORDER_THICKNESS
                } else {
                    BORDER_THICKNESS
                };

                draw_rectangle_lines(
                    x,
                    y,
                    self.cell_width,
                    self.cell_height,
                    thickness,
                    BORDER_COLOR,
                );

                // Draw a diagonal line to indicate this cells contains a `DAT` bomb written by some warrior.
                if cell.instruction.operation.opcode == Opcode::DAT
                    && cell.operation_author.is_some()
                {
                    self.draw_bomb(x, y);
                }
            }
        }
    }

    /// Draw a diagonal line across the cell to indicate this is a `DAT` bomb.
    fn draw_bomb(&self, x: f32, y: f32) {
        const BOMB_THICKNESS: f32 = 1.5;
        const BOMB_COLOR: macroquad::color::Color = WHITE;

        draw_line(
            x,
            y,
            x + self.cell_width,
            y + self.cell_height,
            BOMB_THICKNESS,
            BOMB_COLOR,
        );
    }

    /// Draw all warriors' tasks, and then draw the current warrior's current task.
    fn draw_tasks(&self, mars: &Mars) {
        // Draw all warrior's tasks.
        for warrior_id in rendering_utils::generate_warrior_rendering_order(
            mars.warrior_contexts.len(),
            mars.current_warrior_id,
        ) {
            #[allow(clippy::indexing_slicing, reason = "The index is valid 👌")]
            for &address in mars.warrior_contexts[warrior_id].task_queue.iter() {
                self.draw_task(address, warrior_id, false);
            }
        }

        // Draw current warrior's current task.
        #[allow(clippy::indexing_slicing, reason = "The index is valid 👌")]
        if let Some(address) = mars.warrior_contexts[mars.current_warrior_id]
            .task_queue
            .peek()
        {
            self.draw_task(address, mars.current_warrior_id, true);
        }
    }

    /// Draw a circle to indicate a task at an adress in the core.
    fn draw_task(&self, address: Address, warrior_id: WarriorId, is_current_task: bool) {
        const TASK_RADIUS: f32 = 5.0;
        const TASK_BORDER_COLOR: macroquad::color::Color = WHITE;

        let warrior_color = color::get_mq_color(Some(warrior_id));
        let thickness = if is_current_task { 3.0 } else { 1.0 };

        let (x, y) = self.address_to_cell_center(address);

        draw_circle(x, y, TASK_RADIUS, warrior_color);
        draw_circle_lines(x, y, TASK_RADIUS, thickness, TASK_BORDER_COLOR);
    }

    /// Get the (x, y) center position of the cell at `address`.
    #[allow(
        clippy::arithmetic_side_effects,
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "These operations are safe."
    )]
    fn address_to_cell_center(&self, address: usize) -> (f32, f32) {
        let x = address % self.num_columns;
        let y = address / self.num_columns;

        let x_rect = (x as f32) * self.cell_width;
        let y_rect = (y as f32) * self.cell_height;

        let x_center = x_rect + self.cell_width / 2.0;
        let y_center = y_rect + self.cell_height / 2.0;

        (x_center, y_center)
    }
}
