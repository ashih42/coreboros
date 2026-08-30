use macroquad::prelude::*;

use crate::{
    color,
    instruction::opcode::Opcode,
    mars::{Mars, address::Address, config::core_dimension::CoreDimension},
    scene::arena::rendering_utils,
    warrior::warrior_id::WarriorId,
};

/// `RingRenderer` draws the entire core and tasks to the central game area with a ring view.
pub struct RingRenderer {
    num_sectors: usize,
    num_rings: usize,
    x_center: f32,
    y_center: f32,
    big_radius: f32,
    small_radius: f32,
    ring_width: f32,
    sector_angle_in_degrees: f32,
}

const BORDER_THICKNESS: f32 = 1.0;
const BORDER_COLOR: macroquad::color::Color = WHITE;

impl RingRenderer {
    #[allow(
        clippy::cast_precision_loss,
        clippy::as_conversions,
        reason = "These conversions are safe."
    )]
    pub fn new(core_dimension: CoreDimension, game_area_width: f32, game_area_height: f32) -> Self {
        let (num_sectors, num_rings) = core_dimension.as_ring_dimensions();

        let x_center = game_area_width / 2.0;
        let y_center = game_area_height / 2.0;

        let big_radius = game_area_width.min(game_area_height) / 2.0 - BORDER_THICKNESS;
        let small_radius = big_radius / 8.0;

        let ring_width = (big_radius - small_radius) / (num_rings as f32);
        let sector_angle_in_degrees = 360.0 / (num_sectors as f32);

        Self {
            num_sectors,
            num_rings,
            x_center,
            y_center,
            big_radius,
            small_radius,
            ring_width,
            sector_angle_in_degrees,
        }
    }

    pub fn render(&self, mars: &Mars, selected_address: Address) {
        self.draw_core(mars);
        self.highlight_selected_cell(selected_address);
        self.draw_tasks(mars);
    }

    /// Draw all cores with colors indicating instruction author at each cell.
    #[allow(
        clippy::arithmetic_side_effects,
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "These operations are safe."
    )]
    #[allow(
        clippy::suboptimal_flops,
        reason = "`mul_add` would be actually worse in performance for wasm."
    )]
    fn draw_core(&self, mars: &Mars) {
        let sector_angle_in_degrees = 360.0 / (self.num_sectors as f32);

        // Draw all cells in the core, overwriting pie slices from outside to inside.
        for sector_index in 0..self.num_sectors {
            for ring_index in 0..self.num_rings {
                let address = sector_index * self.num_rings + ring_index;

                let cell = mars.core.get_cell(address);
                let cell_color = color::get_mq_color(cell.operation_author.into());

                let pie_radius = self.big_radius - self.ring_width * (ring_index as f32);

                let start_angle_in_degrees = (sector_index as f32) * sector_angle_in_degrees;
                let end_angle_in_degrees = ((sector_index + 1) as f32) * sector_angle_in_degrees;

                draw_pie(
                    self.x_center,
                    self.y_center,
                    pie_radius,
                    start_angle_in_degrees,
                    end_angle_in_degrees,
                    cell_color,
                );

                if cell.instruction.operation.opcode == Opcode::DAT
                    && cell.operation_author.is_some()
                {
                    self.draw_bomb(address);
                }
            }
        }

        // Draw all arc borders.
        for ring_index in 0..=self.num_rings {
            let radius = self.small_radius + self.ring_width * (ring_index as f32);

            draw_circle_lines(
                self.x_center,
                self.y_center,
                radius,
                BORDER_THICKNESS,
                BORDER_COLOR,
            );
        }

        // Draw all line borders.
        for i in 0..self.num_sectors {
            let angle = (i as f32) * std::f32::consts::TAU / (self.num_sectors as f32);
            let x = self.x_center + angle.cos() * self.big_radius;
            let y = self.y_center + angle.sin() * self.big_radius;
            draw_line(
                self.x_center,
                self.y_center,
                x,
                y,
                BORDER_THICKNESS,
                BORDER_COLOR,
            );
        }

        // Draw a black circle to erase pies and lines in the center.
        draw_circle(self.x_center, self.y_center, self.small_radius, BLACK);
    }

    /// Draw a diagonal line across the cell to indicate this is a `DAT` bomb.
    #[allow(
        clippy::arithmetic_side_effects,
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "These operations are safe."
    )]
    #[allow(
        clippy::suboptimal_flops,
        reason = "`mul_add` would be actually worse in performance for wasm."
    )]
    fn draw_bomb(&self, address: Address) {
        const BOMB_THICKNESS: f32 = 1.5;
        const BOMB_COLOR: macroquad::color::Color = WHITE;

        let sector_index = address / self.num_rings;
        let ring_index = address % self.num_rings;

        let radius_outer = self.big_radius - self.ring_width * (ring_index as f32);
        let radius_inner = self.big_radius - self.ring_width * ((ring_index + 1) as f32);

        let angle_1 = (sector_index as f32) * std::f32::consts::TAU / (self.num_sectors as f32);

        let x_1 = self.x_center + angle_1.cos() * radius_outer;
        let y_1 = self.y_center + angle_1.sin() * radius_outer;

        let angle_2 =
            ((sector_index + 1) as f32) * std::f32::consts::TAU / (self.num_sectors as f32);

        let x_2 = self.x_center + angle_2.cos() * radius_inner;
        let y_2 = self.y_center + angle_2.sin() * radius_inner;

        draw_line(x_1, y_1, x_2, y_2, BOMB_THICKNESS, BOMB_COLOR);
    }

    /// Draw a thick border around the selected cell.
    #[allow(
        clippy::arithmetic_side_effects,
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "These operations are safe."
    )]
    #[allow(
        clippy::suboptimal_flops,
        reason = "`mul_add` would be actually worse in performance for wasm."
    )]
    fn highlight_selected_cell(&self, selected_address: Address) {
        const SELECTED_BORDER_THICKNESS: f32 = 4.0;

        let sector_index = selected_address / self.num_rings;
        let ring_index = selected_address % self.num_rings;

        let radius_outer = self.big_radius - self.ring_width * (ring_index as f32);
        let radius_inner = self.big_radius - self.ring_width * ((ring_index + 1) as f32);

        // Draw 2 lines.
        for i in [sector_index, sector_index + 1] {
            let angle = (i as f32) * std::f32::consts::TAU / (self.num_sectors as f32);

            let x_outer = self.x_center + angle.cos() * radius_outer;
            let y_outer = self.y_center + angle.sin() * radius_outer;

            let x_inner = self.x_center + angle.cos() * radius_inner;
            let y_inner = self.y_center + angle.sin() * radius_inner;

            draw_line(
                x_inner,
                y_inner,
                x_outer,
                y_outer,
                SELECTED_BORDER_THICKNESS,
                BORDER_COLOR,
            );
        }

        // Draw 2 arcs.
        {
            const NUM_SEGMENTS: u8 = 40;

            let start_angle_in_degrees = (sector_index as f32) * self.sector_angle_in_degrees;

            draw_arc(
                self.x_center,
                self.y_center,
                NUM_SEGMENTS,
                radius_outer,
                start_angle_in_degrees,
                SELECTED_BORDER_THICKNESS,
                self.sector_angle_in_degrees,
                BORDER_COLOR,
            );

            draw_arc(
                self.x_center,
                self.y_center,
                NUM_SEGMENTS,
                radius_inner,
                start_angle_in_degrees,
                SELECTED_BORDER_THICKNESS,
                self.sector_angle_in_degrees,
                BORDER_COLOR,
            );
        }
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
    #[allow(
        clippy::arithmetic_side_effects,
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "These operations are safe."
    )]
    #[allow(
        clippy::suboptimal_flops,
        reason = "`mul_add` would be actually worse in performance for wasm."
    )]
    fn draw_task(&self, address: Address, warrior_id: WarriorId, is_current_task: bool) {
        const TASK_RADIUS: f32 = 5.0;
        const TASK_BORDER_COLOR: macroquad::color::Color = WHITE;

        let warrior_color = color::get_mq_color(Some(warrior_id));
        let thickness = if is_current_task { 5.0 } else { 1.0 };

        let sector_index = address / self.num_rings;
        let ring_index = address % self.num_rings;

        let radius = self.big_radius - self.ring_width * (ring_index as f32 + 0.5);

        let angle = (sector_index as f32 + 0.5) * std::f32::consts::TAU / (self.num_sectors as f32);

        let x = self.x_center + angle.cos() * radius;
        let y = self.y_center + angle.sin() * radius;

        draw_circle(x, y, TASK_RADIUS, warrior_color);
        draw_circle_lines(x, y, TASK_RADIUS, thickness, TASK_BORDER_COLOR);
    }
}

/// Draws a sector of a circle (i.e. pie shape).
#[allow(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::cast_precision_loss,
    reason = "These operations are safe."
)]
#[allow(
    clippy::suboptimal_flops,
    reason = "`mul_add` would be actually worse in performance for wasm."
)]
fn draw_pie(
    x_center: f32,
    y_center: f32,
    radius: f32,
    start_angle_in_degrees: f32,
    end_angle_in_degrees: f32,
    color: Color,
) {
    const NUM_SEGMENTS: usize = 40;

    let start_angle_in_radians = start_angle_in_degrees.to_radians();
    let end_angle_in_radians = end_angle_in_degrees.to_radians();

    let step_in_radians = (end_angle_in_radians - start_angle_in_radians) / (NUM_SEGMENTS as f32);

    // Draw many triangles to approximate the pie shape.
    for i in 0..NUM_SEGMENTS {
        let angle1 = start_angle_in_radians + (i as f32 * step_in_radians);
        let angle2 = start_angle_in_radians + ((i + 1) as f32 * step_in_radians);

        let x1 = x_center + radius * angle1.cos();
        let y1 = y_center + radius * angle1.sin();

        let x2 = x_center + radius * angle2.cos();
        let y2 = y_center + radius * angle2.sin();

        draw_triangle(vec2(x_center, y_center), vec2(x1, y1), vec2(x2, y2), color);
    }
}
