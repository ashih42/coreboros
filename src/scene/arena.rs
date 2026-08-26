mod playback_manager;
mod playback_speed;
mod timer;

use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::{
    color,
    game::GameContext,
    instruction::{opcode::Opcode, operand::Operand, operation::Operation},
    mars::{Mars, address::Address, config::Config},
    renderer::Renderer,
    scene::{
        Scene,
        arena::{playback_manager::PlaybackManager, playback_speed::PlaybackSpeed},
        scene_change::SceneChange,
    },
    warrior::warrior_id::{WarriorId, WarriorIdDisplay as _},
    warrior_queue::WarriorQueue,
};

const LEFT_SIDEBAR_WIDTH: f32 = 220.0;
const RIGHT_SIDEBAR_WIDTH: f32 = 360.0;

pub struct Arena {
    mars: Mars,
    selected_address: Address,
    playback_manager: PlaybackManager,
    should_reset_scrollbar_in_coredump: bool,
    next_scene: Option<SceneChange>,
}

impl Scene for Arena {
    fn update(&mut self, game_ctx: &mut GameContext) -> Option<SceneChange> {
        let camera = Self::set_game_camera();

        self.process_mouse_events(&camera);
        self.process_keyboard_events();
        self.process_playback_events();
        self.render(&game_ctx.renderer);

        self.next_scene.take()
    }
}

impl Arena {
    #[must_use]
    pub fn new(warrior_queue: WarriorQueue, config: Config) -> Self {
        Self {
            mars: Mars::new(warrior_queue.into(), config),
            selected_address: 0,
            playback_manager: PlaybackManager::default(),
            should_reset_scrollbar_in_coredump: false,
            next_scene: None,
        }
    }

    fn process_playback_events(&mut self) {
        if self.playback_manager.poll() {
            if self.mars.game_over {
                self.stop();
            } else {
                self.step();
            }
        }
    }

    fn step(&mut self) {
        self.mars.step();
    }

    fn play(&mut self) {
        self.playback_manager.play();
        self.step();
    }

    fn stop(&mut self) {
        self.playback_manager.stop();
    }

    fn toggle_play_pause(&mut self) {
        if self.playback_manager.is_playing() {
            self.stop();
        } else {
            self.play();
        }
    }

    fn toggle_speed(&mut self) {
        let next_speed = self.playback_manager.get_next_speed();
        self.playback_manager.set_speed(next_speed);

        if !self.playback_manager.is_playing() {
            self.play();
        }
    }

    fn start_new_game(&mut self) {
        self.stop();

        let loading_next_game = self.mars.game_over;
        self.mars.reset(loading_next_game);
    }

    fn go_to_editor_scene(&mut self) {
        let warrior_queue = self
            .mars
            .warrior_contexts
            .iter()
            .map(|context| &context.warrior)
            .into();

        self.next_scene = Some(SceneChange::to_editor(warrior_queue));
    }

    /// Process keyboard events.
    fn process_keyboard_events(&mut self) {
        if is_key_pressed(KeyCode::Left) {
            self.move_selected_address_left();
        }

        if is_key_pressed(KeyCode::Right) {
            self.move_selected_address_right();
        }

        if is_key_pressed(KeyCode::Up) {
            self.move_selected_address_up();
        }

        if is_key_pressed(KeyCode::Down) {
            self.move_selected_address_down();
        }

        if is_key_pressed(KeyCode::S) {
            self.step();
        }

        if is_key_pressed(KeyCode::F) {
            self.toggle_speed();
        }

        if is_key_pressed(KeyCode::C) {
            self.zoom_to_current_warrior_task();
        }

        if is_key_pressed(KeyCode::N) {
            self.start_new_game();
        }

        if is_key_pressed(KeyCode::Space) {
            self.toggle_play_pause();
        }

        if is_key_pressed(KeyCode::T) {
            self.playback_manager.play_turbo();
        }
    }

    #[allow(clippy::arithmetic_side_effects, reason = "This expression is safe.")]
    const fn move_selected_address_left(&mut self) {
        let core_size = self.mars.config.core_dimension.as_size();

        let new_address = (self.selected_address + core_size - 1) % core_size;
        self.set_selected_address(new_address);
    }

    #[allow(clippy::arithmetic_side_effects, reason = "This expression is safe.")]
    const fn move_selected_address_right(&mut self) {
        let core_size = self.mars.config.core_dimension.as_size();

        let new_address = (self.selected_address + 1) % core_size;
        self.set_selected_address(new_address);
    }

    #[allow(clippy::arithmetic_side_effects, reason = "This expression is safe.")]
    const fn move_selected_address_up(&mut self) {
        let core_size = self.mars.config.core_dimension.as_size();
        let (width, _) = self.mars.config.core_dimension.as_grid_dimensions();

        let new_address = (self.selected_address + core_size - width) % core_size;
        self.set_selected_address(new_address);
    }

    #[allow(clippy::arithmetic_side_effects, reason = "This expression is safe.")]
    const fn move_selected_address_down(&mut self) {
        let core_size = self.mars.config.core_dimension.as_size();
        let (width, _) = self.mars.config.core_dimension.as_grid_dimensions();

        let new_address = (self.selected_address + width) % core_size;
        self.set_selected_address(new_address);
    }

    fn zoom_to_current_warrior_task(&mut self) {
        if let Some(address) = self
            .mars
            .warrior_contexts
            .get(self.mars.current_warrior_id)
            .and_then(|context| context.task_queue.peek())
        {
            self.selected_address = address;
        }
    }

    /// Update the selected address AND ALSO set flag to reset scrollbar.
    #[inline]
    const fn set_selected_address(&mut self, address: Address) {
        self.selected_address = address;
        self.should_reset_scrollbar_in_coredump = true;
    }

    /// Process mouse events.
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::as_conversions,
        reason = "These casts are valid 👌"
    )]
    fn process_mouse_events(&mut self, camera: &Camera2D) {
        let game_area_width = Self::get_game_area_width();
        let game_area_height = Self::get_game_area_height();

        let (width, height) = self.mars.config.core_dimension.as_grid_dimensions();
        let num_cells_per_row = width;
        let num_cells_per_column = height;

        let cell_width = game_area_width / (num_cells_per_row as f32);
        let cell_height = game_area_height / (num_cells_per_column as f32);

        // Update `selected_address` from LMB click.
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            if LEFT_SIDEBAR_WIDTH <= mouse_x && mouse_x <= (screen_width() - RIGHT_SIDEBAR_WIDTH) {
                // Convert mouse position to world space.
                let world_pos = camera.screen_to_world(vec2(mouse_x, mouse_y));

                // println!(
                //     "mouse_x={}, mouse_y={}, world_pos={}",
                //     mouse_x, mouse_y, world_pos
                // );

                let x = (world_pos.x / cell_width) as usize;
                let y = (world_pos.y / cell_height) as usize;

                let new_address = self.get_address(x, y);

                // println!(
                //     "world_pos: {world_pos}; \t cell x = {x}, y = {y}, \t new_address = {new_address}"
                // );
                self.set_selected_address(new_address);
            }
        }
    }

    /// Update the game area and UI in sidebars.
    fn render(&mut self, renderer: &Renderer) {
        clear_background(BLACK);

        self.draw_game_area();
        self.draw_ui(renderer);
    }

    /// Configure the camera for the game area.
    /// This must be done on every frame because window size may change at any time.
    #[allow(
        clippy::cast_possible_truncation,
        clippy::as_conversions,
        reason = "These casts are valid."
    )]
    fn set_game_camera() -> Camera2D {
        // Calculate dynamic dimensions for the center gameplay area
        let game_area_width = Self::get_game_area_width();
        let game_area_height = Self::get_game_area_height();

        // 2. Build the camera using from_display_rect to auto-calculate projection math correctly
        let mut game_camera = Camera2D::from_display_rect(Rect::new(
            0.0,              // X start relative to the game world coordinate layout
            0.0,              // Y start relative to the game world coordinate layout
            game_area_width,  // Game world target width
            game_area_height, // Game world target height
        ));

        // This is necessary to unflip y, which was flipped by Macroquad's 3D projection matrix.
        game_camera.zoom.y = -game_camera.zoom.y;

        // 3. Assign the viewport bounds so it renders strictly within the center of the display screen
        game_camera.viewport = Some((
            LEFT_SIDEBAR_WIDTH as i32,
            0,
            game_area_width as i32,
            game_area_height as i32,
        ));

        set_camera(&game_camera);
        game_camera
    }

    /// Draw the entire core to the game area.
    #[allow(
        clippy::cast_precision_loss,
        clippy::as_conversions,
        reason = "These casts are valid 👌"
    )]
    fn draw_game_area(&self) {
        let game_area_width = Self::get_game_area_width();
        let game_area_height = Self::get_game_area_height();

        let (width, height) = self.mars.config.core_dimension.as_grid_dimensions();
        let num_cells_per_row = width;
        let num_cells_per_column = height;

        let cell_width = game_area_width / (num_cells_per_row as f32);
        let cell_height = game_area_height / (num_cells_per_column as f32);

        // Draw cores
        for x in 0..num_cells_per_row {
            for y in 0..num_cells_per_column {
                let address = self.get_address(x, y);

                let cell = self.mars.core.get_cell(address);
                let cell_color = color::get_mq_color(cell.operation_author.into());

                let x = (x as f32) * cell_width;
                let y = (y as f32) * cell_height;

                draw_rectangle(x, y, cell_width, cell_height, cell_color);

                let thickness = if address == self.selected_address {
                    5.0
                } else {
                    1.0
                };
                draw_rectangle_lines(x, y, cell_width, cell_height, thickness, WHITE);

                // Draw a diagonal line if this cell contains a `DAT` written by some warrior.
                if cell.instruction.operation.opcode == Opcode::DAT
                    && cell.operation_author.is_some()
                {
                    draw_line(x, y, x + cell_width, y + cell_height, 1.5, WHITE);
                }
            }
        }

        // Draw all warrior's tasks
        for warrior_id in self.get_warrior_rendering_order() {
            let warrior_color = color::get_mq_color(Some(warrior_id));
            let radius = 5.0;
            let thickness = 1.0;

            #[allow(clippy::indexing_slicing, reason = "The index is valid 👌")]
            for &address in self.mars.warrior_contexts[warrior_id].task_queue.iter() {
                let (x, y) = self.address_to_game_area_x_y(address);

                draw_circle(x, y, radius, warrior_color);
                draw_circle_lines(x, y, radius, thickness, WHITE);
            }
        }

        // Draw current warrior's current task
        {
            let warrior_id = self.mars.current_warrior_id;
            let warrior_color = color::get_mq_color(Some(warrior_id));
            let radius = 5.0;
            let thickness = 3.0;

            if let Some(address) = self
                .mars
                .warrior_contexts
                .get(warrior_id)
                .and_then(|context| context.task_queue.peek())
            {
                let (x, y) = self.address_to_game_area_x_y(address);

                draw_circle(x, y, radius, warrior_color);
                draw_circle_lines(x, y, radius, thickness, WHITE);
            }
        }
    }

    #[allow(
        clippy::cast_precision_loss,
        clippy::as_conversions,
        clippy::arithmetic_side_effects,
        reason = "These operations are valid 👌"
    )]
    fn address_to_game_area_x_y(&self, address: usize) -> (f32, f32) {
        let game_area_width = Self::get_game_area_width();
        let game_area_height = Self::get_game_area_height();

        let (width, height) = self.mars.config.core_dimension.as_grid_dimensions();

        let cell_width = game_area_width / (width as f32);
        let cell_height = game_area_height / (height as f32);

        let y = address / width;
        let x = address % width;

        let rect_y = (y as f32) * cell_height;
        let rect_x = (x as f32) * cell_width;

        let circle_y = rect_y + cell_height / 2.0;
        let circle_x = rect_x + cell_width / 2.0;

        (circle_x, circle_y)
    }

    /// Define and draw all UI elements.
    fn draw_ui(&mut self, renderer: &Renderer) {
        egui_macroquad::ui(|egui_ctx| {
            self.draw_left_sidebar(egui_ctx, renderer);
            self.draw_right_sidebar(egui_ctx, renderer);
        });

        egui_macroquad::draw();
    }

    /// Draw the left sidebar.
    ///
    /// The left side bar includes:
    /// - General context of the current game.
    /// - Details of all warriors.
    fn draw_left_sidebar(&self, egui_ctx: &egui::Context, renderer: &Renderer) {
        const SUBHEADING_FONT_SIZE: f32 = 20.0;

        // Keep Mars logic in 0-based counting, but display these numbers in 1-based counting.
        let game_str = renderer.usize_plus_1_to_str(self.mars.game_counter);
        let turn_str = renderer.usize_plus_1_to_str(self.mars.turn_counter);
        let cycle_str = renderer.usize_plus_1_to_str(self.mars.cycle_counter);

        let turn_limit_str = renderer.usize_to_str(self.mars.config.turn_limit);

        egui::SidePanel::left("left_sidebar")
            .exact_width(LEFT_SIDEBAR_WIDTH)
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.label(egui::RichText::new("Coreboros").size(30.0));
                ui.separator();
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Game:").size(SUBHEADING_FONT_SIZE));
                    ui.label(egui::RichText::new(game_str).size(SUBHEADING_FONT_SIZE));
                });

                ui.add_space(2.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Turn:").size(SUBHEADING_FONT_SIZE));
                    ui.label(egui::RichText::new(turn_str).size(SUBHEADING_FONT_SIZE));
                    ui.label(egui::RichText::new("/").size(SUBHEADING_FONT_SIZE));
                    ui.label(egui::RichText::new(turn_limit_str).size(SUBHEADING_FONT_SIZE));
                });

                ui.add_space(2.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Cycle:").size(SUBHEADING_FONT_SIZE));
                    ui.label(egui::RichText::new(cycle_str).size(SUBHEADING_FONT_SIZE));
                });

                ui.separator();
                ui.add_space(5.0);

                self.draw_game_over_info(ui, renderer);

                ui.add_space(5.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for warrior_id in 0..self.mars.warrior_contexts.len() {
                        self.draw_warrior_info(warrior_id, ui, renderer);
                        ui.add_space(10.0);
                    }
                });
            });
    }

    /// If applicable, draw the "Game Over" info and show the winner.
    fn draw_game_over_info(&self, ui: &mut egui::Ui, renderer: &Renderer) {
        const WINNER_FONT_SIZE: f32 = 18.0;

        if self.mars.game_over {
            ui.label(egui::RichText::new("GAME OVER").size(30.0));

            if let Some(winner_id) = self.mars.winner {
                let color = color::get_egui_color32(Some(winner_id));
                let winner_id_str = renderer.usize_to_str(winner_id.as_display_id());

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Winner:").size(WINNER_FONT_SIZE));
                    ui.colored_label(color, egui::RichText::new("Warrior").size(WINNER_FONT_SIZE));
                    ui.colored_label(
                        color,
                        egui::RichText::new(winner_id_str).size(WINNER_FONT_SIZE),
                    );
                });
            } else {
                ui.label(egui::RichText::new("Draw").size(WINNER_FONT_SIZE));
            }
        }
    }

    /// Draw a frame to show a warrior's details.
    fn draw_warrior_info(&self, warrior_id: WarriorId, ui: &mut egui::Ui, renderer: &Renderer) {
        #[allow(clippy::indexing_slicing, reason = "This index is valid 👌")]
        let warrior_context = &self.mars.warrior_contexts[warrior_id];

        let warrior_name = &warrior_context.warrior.metadata.name;
        let warrior_color = color::get_egui_color32(Some(warrior_id));

        let tasks_str = renderer.usize_to_str(warrior_context.task_queue.len());
        let task_capacity_str = renderer.usize_to_str(warrior_context.task_queue.get_capacity());
        let wins_str = renderer.usize_to_str(warrior_context.num_wins);

        // Use a thick colored border for the current-turn warrior.
        let card_stroke = if warrior_id == self.mars.current_warrior_id {
            egui::Stroke::new(1.0, egui::Color32::WHITE)
        } else {
            egui::Stroke::new(1.0, egui::Color32::GRAY)
        };

        // Draw a container card for the warrior.
        egui::Frame::group(ui.style())
            .fill(egui::Color32::from_rgb(40, 40, 40))
            .stroke(card_stroke)
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.set_min_width(LEFT_SIDEBAR_WIDTH - 40.0);

                ui.horizontal(|ui| {
                    // Draw a colored square indicating this warrior's color.
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 2.0, warrior_color);

                    ui.add_space(4.0);

                    // Draw a skull if this warrior is dead.
                    if !warrior_context.is_alive()
                        && let Some(skull) = &renderer.texture_manager.skull
                    {
                        ui.add(egui::Image::new(skull).max_width(16.0));
                    }

                    // Draw a trophy if this warrior is the winner.
                    if let Some(winner_id) = self.mars.winner
                        && winner_id == warrior_id
                        && let Some(trophy) = &renderer.texture_manager.trophy
                    {
                        ui.add(egui::Image::new(trophy).max_width(16.0));
                    }

                    // Draw "Warrior X".
                    ui.colored_label(egui::Color32::WHITE, "Warrior");
                    ui.colored_label(
                        egui::Color32::WHITE,
                        renderer.usize_to_str(warrior_id.as_display_id()),
                    );
                });

                ui.add_space(2.0);

                // Draw the warrior's name in the warrior's color.
                ui.label(
                    egui::RichText::new(warrior_name)
                        .size(20.0)
                        .color(warrior_color),
                );

                ui.add_space(2.0);

                // Draw number of tasks for this warrior.
                ui.horizontal(|ui| {
                    ui.label("Tasks:");
                    ui.label(tasks_str);
                    ui.label("/");
                    ui.label(task_capacity_str);
                    if warrior_context.task_queue.is_full() {
                        ui.label("(Full)");
                    }
                });

                ui.add_space(2.0);

                // Draw number of wins for this warrior.
                ui.horizontal(|ui| {
                    ui.label("Wins:");
                    ui.label(wins_str);
                });
            });
    }

    /// Draw the right sidebar.
    ///
    /// The right sidebar includes:
    /// - Buttons to navigate the game.
    /// - A core dump showing details at core addresses.
    fn draw_right_sidebar(&mut self, egui_ctx: &egui::Context, renderer: &Renderer) {
        egui::SidePanel::right("right_sidebar")
            .exact_width(RIGHT_SIDEBAR_WIDTH)
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.add_space(5.0);

                // Draw 2 buttons for top-level operations.
                ui.horizontal(|ui| {
                    let width = (ui.available_width() - 16.0) / 3.0;
                    let height = 24.0;

                    self.add_button(ui, width, height, "New Game", Self::start_new_game);
                    self.add_button(ui, width, height, "To Editor", Self::go_to_editor_scene);
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                ui.heading("Controls");
                ui.add_space(5.0);

                // Draw current playback speed.
                ui.horizontal(|ui| {
                    if self.playback_manager.is_playing() {
                        // ui.label(egui::RichText::new("Speed").size(30.0));
                        let speed = self.playback_manager.get_speed();
                        ui.label("Speed:");
                        ui.label(speed.as_ref());
                    } else {
                        ui.label("Paused");
                    }
                });

                ui.add_space(5.0);

                // Draw 3 buttons for playback control.
                ui.horizontal(|ui| {
                    let width = (ui.available_width() - 16.0) / 3.0;
                    let height = 24.0;

                    let play_pause_label = if self.playback_manager.is_playing() {
                        "⏸ Pause"
                    } else {
                        "▶ Play"
                    };

                    let speed_label = match self.playback_manager.get_speed() {
                        PlaybackSpeed::Turbo => "Normal",
                        _ => "Faster",
                    };

                    self.add_button(ui, width, height, "→ Step", Self::step);
                    self.add_button(ui, width, height, play_pause_label, Self::toggle_play_pause);
                    self.add_button(ui, width, height, speed_label, Self::toggle_speed);
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                ui.heading("Core Dump");
                ui.add_space(5.0);

                self.draw_core_dump(ui, renderer);
            });
    }

    /// Add a button with a callback when clicked.
    fn add_button(
        &mut self,
        ui: &mut egui::Ui,
        width: f32,
        height: f32,
        label: &str,
        callback: fn(&mut Self),
    ) {
        let button_response = ui.add_sized([width, height], egui::Button::new(label));

        if button_response.clicked() {
            callback(self);
        }
    }

    /// Draw the core dump, displaying rows of addresses with these 5 columns:
    /// - warrior icon (to indicate if there is a warrior with a task at this address)
    /// - the absolute address
    /// - the operation at this address
    /// - the operand a at this address
    /// - the operand b at this address
    fn draw_core_dump(&mut self, ui: &mut egui::Ui, renderer: &Renderer) {
        const NUM_ROWS_IN_COREDUMP: usize = 40;

        let core_size = self.mars.config.core_dimension.as_size();

        egui::ScrollArea::vertical().show(ui, |ui| {
            if self.should_reset_scrollbar_in_coredump {
                ui.scroll_to_cursor(Some(egui::Align::TOP));
                self.should_reset_scrollbar_in_coredump = false;
            }

            let slot_width = (ui.available_width() - 40.0) / 4.0;

            #[allow(clippy::arithmetic_side_effects, reason = "This operation is valid 👌")]
            for i in 0..NUM_ROWS_IN_COREDUMP {
                let address = (self.selected_address + i) % core_size;
                let cell = self.mars.core.get_cell(address);

                ui.horizontal(|ui| {
                    self.draw_warrior_icon_at_address(address, ui, renderer);

                    // Draw a slot showing the absolute address.
                    Self::draw_address_slot(
                        address,
                        egui::Color32::DARK_GRAY,
                        slot_width,
                        ui,
                        renderer,
                    );

                    // Draw a slot showing the operation.
                    Self::draw_operation_slot(
                        cell.instruction.operation,
                        color::get_egui_color32(cell.operation_author.into()),
                        slot_width,
                        ui,
                    );

                    // Draw a slot showing the A operand.
                    Self::draw_operand_slot(
                        cell.instruction.a,
                        color::get_egui_color32(cell.operation_author.into()),
                        color::get_egui_color32(cell.a_author.into()),
                        slot_width - 10.0,
                        ui,
                        renderer,
                    );

                    // Draw a slot showing the B operand.
                    Self::draw_operand_slot(
                        cell.instruction.b,
                        color::get_egui_color32(cell.operation_author.into()),
                        color::get_egui_color32(cell.b_author.into()),
                        slot_width - 10.0,
                        ui,
                        renderer,
                    );
                });

                ui.add_space(2.0);
            }
        });
    }

    /// Draw a slot showing the absolute address number of a row in the core dump.
    fn draw_address_slot(
        address: usize,
        bg_color: egui::Color32,
        slot_width: f32,
        ui: &mut egui::Ui,
        renderer: &Renderer,
    ) {
        ui.allocate_ui(egui::vec2(slot_width, ui.available_height()), |ui| {
            egui::Frame::NONE
                .fill(bg_color)
                .inner_margin(egui::Margin::symmetric(4, 2))
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.colored_label(egui::Color32::WHITE, renderer.usize_to_str(address));
                    });
                });
        });
    }

    /// Draw a slot showing the operation (opcode.modifier) of a row in the core dump.
    fn draw_operation_slot(
        operation: Operation,
        bg_color: egui::Color32,
        slot_width: f32,
        ui: &mut egui::Ui,
    ) {
        ui.allocate_ui(egui::vec2(slot_width, ui.available_height()), |ui| {
            egui::Frame::NONE
                .fill(bg_color)
                .inner_margin(egui::Margin::symmetric(4, 2))
                .show(ui, |ui| {
                    ui.set_min_width(slot_width);
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.colored_label(egui::Color32::WHITE, operation.opcode.as_ref());
                            ui.colored_label(egui::Color32::WHITE, ".");
                            ui.colored_label(egui::Color32::WHITE, operation.modifier.as_ref());
                        });
                    });
                });
        });
    }

    /// Draw a slot showing the an operand (addressing mode and number) of a row in the core dump.
    fn draw_operand_slot(
        operand: Operand,
        addressing_mode_bg_color: egui::Color32,
        number_bg_color: egui::Color32,
        slot_width: f32,
        ui: &mut egui::Ui,
        renderer: &Renderer,
    ) {
        const ADDRESSING_MODE_WIDTH: f32 = 10.0;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            // Draw the operand's addressing mode, with its own background color.
            ui.allocate_ui(
                egui::vec2(ADDRESSING_MODE_WIDTH, ui.available_height()),
                |ui| {
                    egui::Frame::NONE
                        .fill(addressing_mode_bg_color)
                        .inner_margin(egui::Margin::symmetric(4, 2))
                        .show(ui, |ui| {
                            ui.with_layout(
                                egui::Layout::top_down_justified(egui::Align::LEFT),
                                |ui| {
                                    ui.colored_label(egui::Color32::WHITE, operand.mode.as_ref());
                                },
                            );
                        });
                },
            );

            // Draw the operand's number, with its own background color.
            ui.allocate_ui(
                egui::vec2(slot_width - ADDRESSING_MODE_WIDTH, ui.available_height()),
                |ui| {
                    egui::Frame::NONE
                        .fill(number_bg_color)
                        .inner_margin(egui::Margin::symmetric(4, 2))
                        .show(ui, |ui| {
                            ui.with_layout(
                                egui::Layout::top_down_justified(egui::Align::LEFT),
                                |ui| {
                                    ui.colored_label(
                                        egui::Color32::WHITE,
                                        renderer.i32_to_str(operand.number),
                                    );
                                },
                            );
                        });
                },
            );
        });
    }

    /// Draw an warrior icon at this address if there is a warrior with a task at this address.
    fn draw_warrior_icon_at_address(
        &self,
        address: Address,
        ui: &mut egui::Ui,
        renderer: &Renderer,
    ) {
        const ICON_WIDTH: f32 = 20.0;

        // If the current warrior's current task is at this address, draw a tinted warrior icon.
        if let Some(task) = self
            .mars
            .warrior_contexts
            .get(self.mars.current_warrior_id)
            .and_then(|context| context.task_queue.peek())
            && task == address
            && let Some(warrior_icon) = renderer
                .texture_manager
                .get_warrior_icon(self.mars.current_warrior_id)
        {
            let color = color::get_egui_color32(Some(self.mars.current_warrior_id));
            ui.add(
                egui::Image::new(warrior_icon)
                    .max_width(ICON_WIDTH)
                    .tint(color),
            );
            return;
        }

        if let Some(warrior_id) = self
            .get_warrior_rendering_order()
            .rev()
            .find(|&warrior_id| {
                self.mars
                    .warrior_contexts
                    .get(warrior_id)
                    .is_some_and(|context| context.task_queue.contains(address))
            })
            && let Some(warrior_icon) = renderer.texture_manager.get_warrior_icon(warrior_id)
        {
            ui.add(egui::Image::new(warrior_icon).max_width(ICON_WIDTH));
            return;
        }

        // Otherwise, pad this area with empty space.
        ui.allocate_space(egui::vec2(ICON_WIDTH, 16.0));
    }

    /// Get the width of the game area.
    #[inline]
    fn get_game_area_width() -> f32 {
        screen_width() - (LEFT_SIDEBAR_WIDTH + RIGHT_SIDEBAR_WIDTH)
    }

    /// Get the height of the game area.
    #[inline]
    fn get_game_area_height() -> f32 {
        screen_height()
    }

    /// Convert `(x, y)` to an address value to index into the `Core`.
    #[inline]
    #[allow(clippy::arithmetic_side_effects, reason = "This operation is valid 👌")]
    const fn get_address(&self, x: usize, y: usize) -> usize {
        let (width, _) = self.mars.config.core_dimension.as_grid_dimensions();

        y * width + x
    }

    #[inline]
    fn get_warrior_rendering_order(&self) -> impl DoubleEndedIterator<Item = WarriorId> {
        generate_warrior_rendering_order(
            self.mars.warrior_contexts.len(),
            self.mars.current_warrior_id,
        )
    }
}

/// Find the appropriate order to draw the effects of warriors such that the current-turn warrior is drawn last,
/// so his effects are drawn on top of those of other warriors.
///
/// Example: In a 4 player game, if it is currently player 2's turn, then the rendering order is [3, 0, 1, 2].
#[inline]
#[allow(clippy::arithmetic_side_effects, reason = "This operation is valid 👌")]
fn generate_warrior_rendering_order(
    num_warriors: usize,
    current_warrior_id: WarriorId,
) -> impl DoubleEndedIterator<Item = WarriorId> {
    (0..num_warriors).map(move |i| (i + current_warrior_id + 1) % num_warriors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_warrior_rendering_order() {
        assert!(generate_warrior_rendering_order(4, 0).eq([1, 2, 3, 0]));
        assert!(generate_warrior_rendering_order(4, 1).eq([2, 3, 0, 1]));
        assert!(generate_warrior_rendering_order(4, 2).eq([3, 0, 1, 2]));
        assert!(generate_warrior_rendering_order(4, 3).eq([0, 1, 2, 3]));

        assert!(generate_warrior_rendering_order(2, 0).eq([1, 0]));
        assert!(generate_warrior_rendering_order(2, 1).eq([0, 1]));

        assert!(generate_warrior_rendering_order(1, 0).eq([0]));
    }
}
