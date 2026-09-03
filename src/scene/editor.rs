use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::{
    color,
    config_manager::ConfigManager,
    game_context::GameContext,
    mars::config::warrior_separation_strategy::WarriorSeparationStrategy,
    renderer::Renderer,
    scene::{
        Scene,
        editor::{text_editor::TextEditor, warrior_queue::WarriorQueue},
        scene_change::SceneChange,
    },
    warrior::{Warrior, warrior_id::WarriorIdDisplay as _},
    warrior_vault::WarriorVault,
};

mod syntax_highlighter;
mod syntax_kind;
mod text_editor;
mod warrior_queue;

/// `Editor` is a UI-only scene where the user can load, edit, and save warriors,
/// and change config values before entering the arena.
#[derive(Default)]
pub struct Editor {
    #[allow(clippy::struct_field_names, reason = "This is a good name 👌")]
    text_editor: TextEditor,
    console_text: String,
    current_warrior: Option<Warrior>,
    warrior_queue: WarriorQueue,
    next_scene: Option<SceneChange>,
}

impl Scene for Editor {
    fn update(&mut self, game_ctx: &mut GameContext) -> Option<SceneChange> {
        self.process_keyboard_events();
        self.render(game_ctx);

        self.next_scene.take()
    }
}

const LEFT_SIDEBAR_WIDTH: f32 = 250.0;
const RIGHT_SIDEBAR_WIDTH: f32 = 300.0;

impl Editor {
    #[must_use]
    pub fn new(warriors: Box<[Warrior]>) -> Self {
        Self {
            text_editor: TextEditor::default(),
            console_text: String::new(),
            current_warrior: None,
            warrior_queue: warriors.into(),
            next_scene: None,
        }
    }

    /// Process keyboard events.
    fn process_keyboard_events(&mut self) {
        if is_key_pressed(KeyCode::L) {
            self.copy_current_warrior_to_queue();
        }
    }

    /// Render only UI elements.
    fn render(&mut self, game_ctx: &mut GameContext) {
        egui_macroquad::ui(|egui_ctx| {
            self.draw_left_sidebar(egui_ctx, game_ctx);
            self.draw_right_sidebar(egui_ctx, game_ctx);
            self.draw_bottom_console(egui_ctx);
            self.draw_central_buttons(egui_ctx, &mut game_ctx.warrior_vault);
            self.draw_central_redcode_editor(egui_ctx);
        });
        egui_macroquad::draw();
    }

    /// Draw the bottom console, which shows a text area for error messages.
    fn draw_bottom_console(&mut self, egui_ctx: &egui::Context) {
        let bottom_panel_height = screen_height() * 0.30;

        egui::TopBottomPanel::bottom("bottom_console")
            .exact_height(bottom_panel_height)
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.add_space(4.0);
                ui.heading("Console Logs");
                ui.add_space(4.0);

                egui::ScrollArea::vertical()
                    .max_height(f32::INFINITY)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.console_text)
                                .min_size(ui.available_size())
                                .desired_width(f32::INFINITY)
                                .hint_text("Hint: Try \"dwarf\" and \"imp\" first.")
                                .interactive(false),
                        );
                    });
            });
    }

    /// Draw a row of 3 buttons to operate on the current warrior.
    fn draw_central_buttons(&mut self, egui_ctx: &egui::Context, warrior_vault: &mut WarriorVault) {
        let button_height = 24.0;

        egui::TopBottomPanel::bottom("warrior_buttons_panel")
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    let width = (ui.available_width() - 16.0) / 3.0;
                    let height = button_height;

                    self.add_button(
                        ui,
                        width,
                        height,
                        "Compile",
                        self.current_warrior.is_none(),
                        Self::compile,
                    );

                    self.add_button(
                        ui,
                        width,
                        height,
                        "Load",
                        self.current_warrior.is_some() && !self.warrior_queue.is_full(),
                        Self::copy_current_warrior_to_queue,
                    );

                    self.add_button(
                        ui,
                        width,
                        height,
                        "Save",
                        self.current_warrior.is_some(),
                        |editor| {
                            editor.save_current_warrior_to_vault(warrior_vault);
                        },
                    );
                });

                ui.add_space(5.0);

                if self.current_warrior.is_some() {
                    ui.colored_label(egui::Color32::GREEN, "Warrior is ready!");
                }

                ui.add_space(5.0);
            });
    }

    /// Draw the Redcode text editor for user to edit the current warrior.
    fn draw_central_redcode_editor(&mut self, egui_ctx: &egui::Context) {
        egui::CentralPanel::default().show(egui_ctx, |ui| {
            ui.heading("Redcode Editor");
            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            egui::ScrollArea::vertical()
                .max_height(f32::INFINITY)
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        self.draw_line_numbers_col(ui);
                        self.draw_text_editor(ui);
                    });
                });
        });
    }

    /// Draw the line numbers column part of the Redcode text editor.
    fn draw_line_numbers_col(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(2.5);
            ui.add(egui::Label::new(
                egui::RichText::new(&self.text_editor.line_numbers_col).size(16.0),
            ));
        });
    }

    /// Draw the input text area part of the Redcode text editor.
    fn draw_text_editor(&mut self, ui: &mut egui::Ui) {
        const INPUT_CHAR_LIMIT: usize = 100_000;

        let mut apply_syntax_highlighting = |ui: &egui::Ui, redcode: &str, _wrap_width: f32| {
            TextEditor::get_cached_or_build_new_galley(
                ui,
                redcode,
                &mut self.text_editor.cached_input_text,
                &mut self.text_editor.cached_galley,
            )
        };

        let scroll_area_output = egui::ScrollArea::horizontal()
            .auto_shrink(false)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.text_editor.input_text)
                        .min_size(ui.available_size())
                        .desired_width(f32::INFINITY)
                        .char_limit(INPUT_CHAR_LIMIT)
                        .layouter(&mut apply_syntax_highlighting),
                )
            });

        if scroll_area_output.inner.changed() {
            self.current_warrior = None;
            self.text_editor.update_line_numbers_col_if_changed();
        }
    }

    /// Copy the current warrior to `warrior_queue`.
    fn copy_current_warrior_to_queue(&mut self) {
        if let Some(warrior) = &self.current_warrior {
            self.warrior_queue.push_if_not_full(warrior.clone());
            self.console_text = format!("Added \"{}\" to queue.", warrior.metadata.name);
        }
    }

    /// Save the current warrior to `warrior_vault`.
    fn save_current_warrior_to_vault(&mut self, warrior_vault: &mut WarriorVault) {
        if let Some(warrior) = &self.current_warrior {
            warrior_vault.save_warrior(warrior);
            self.console_text = format!("Saved \"{}\" to vault.", warrior.metadata.name);
        }
    }

    /// Prepare a `SceneChange` message to go to `Arena` scene with the warriors in `warrior_queue`.
    /// Note: You must validate these warriors can enter the core under current config, before calling this method.
    fn enter_arena(&mut self) {
        let warrior_queue = std::mem::take(&mut self.warrior_queue);
        self.next_scene = Some(SceneChange::to_arena(warrior_queue.into_boxed_warriors()));
    }

    /// Compile the current warrior, showing a success or error message in the console area.
    fn compile(&mut self) {
        match Warrior::from_text(&self.text_editor.input_text) {
            Ok(warrior) => {
                self.console_text = format!("Compiled \"{}\" successfully.", warrior.metadata.name);
                self.current_warrior = Some(warrior);
            }
            Err(err) => {
                self.console_text = format!("{err:?}");
            }
        }
    }

    /// Add a button with a condition to enable the button and a callback closure when clicked.
    fn add_button<F>(
        &mut self,
        ui: &mut egui::Ui,
        width: f32,
        height: f32,
        label: &str,
        enabled: bool,
        mut callback: F,
    ) where
        F: FnMut(&mut Self),
    {
        ui.add_enabled_ui(enabled, |ui| {
            if ui
                .add_sized([width, height], egui::Button::new(label))
                .clicked()
            {
                callback(self);
            }
        });
    }

    /// Draw the left sidebar, which shows the warriors in `warrior_queue`, and a button to take them to `Arena` scene.
    fn draw_left_sidebar(&mut self, egui_ctx: &egui::Context, game_ctx: &GameContext) {
        egui::SidePanel::left("left_navigation")
            .exact_width(LEFT_SIDEBAR_WIDTH)
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.heading("Arena");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;

                    ui.label("Warriors (");
                    ui.label(game_ctx.renderer.usize_to_str(self.warrior_queue.len()));
                    ui.label("/");
                    ui.label(
                        game_ctx
                            .renderer
                            .usize_to_str(self.warrior_queue.get_capacity()),
                    );
                    ui.label(")");
                });

                let button_width = ui.available_width();
                let button_height = 24.0;

                self.add_button(
                    ui,
                    button_width,
                    button_height,
                    "Enter Arena",
                    self.warrior_queue.is_ready_for_arena(),
                    |editor| match game_ctx
                        .config_manager
                        .validate_entry(editor.warrior_queue.as_slice())
                    {
                        Ok(()) => {
                            editor.enter_arena();
                        }
                        Err(err_message) => {
                            editor.console_text = err_message;
                        }
                    },
                );

                ui.separator();
                ui.add_space(5.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for index in 0..self.warrior_queue.len() {
                        self.draw_queued_warrior_widget(index, ui, &game_ctx.renderer);
                        ui.add_space(10.0);
                    }
                });
            });
    }

    /// Draw a widget for a queued warrior, with a frame showing his details, and
    /// with 3 buttons to remove, move up, and move down this warrior in the queue.
    fn draw_queued_warrior_widget(&mut self, index: usize, ui: &mut egui::Ui, renderer: &Renderer) {
        let button_size = egui::vec2(18.0, 18.0);

        ui.scope_builder(egui::UiBuilder::new(), |ui| {
            // Draw the frame.
            let frame_rect = self.draw_queued_warrior_frame(index, ui, renderer);

            // Set up a response for clicking the frame.
            if ui
                .interact(frame_rect, egui::Id::new(index), egui::Sense::click())
                .clicked()
                && let Some(warrior) = self.warrior_queue.get(index)
            {
                self.load_current_warrior(warrior.clone());
            }

            // Draw and set up a response for clicking the remove button.
            if ui
                .put(
                    egui::Rect::from_min_size(
                        egui::pos2(
                            frame_rect.max.x - (button_size.x / 2.0) - 15.0,
                            frame_rect.min.y - (button_size.y / 2.0) + 15.0,
                        ),
                        button_size,
                    ),
                    egui::Button::new("❌").small().corner_radius(5),
                )
                .clicked()
            {
                self.warrior_queue.remove(index);
            }

            // Draw and set up a response for clicking the move up button.
            if ui
                .put(
                    egui::Rect::from_min_size(
                        egui::pos2(
                            frame_rect.max.x - (button_size.x / 2.0) - 15.0,
                            frame_rect.min.y - (button_size.y / 2.0) + 15.0 + 25.0,
                        ),
                        button_size,
                    ),
                    egui::Button::new("🔼").small().corner_radius(5),
                )
                .clicked()
            {
                self.warrior_queue.move_up(index);
            }

            // Draw and set up a response for clicking the move down button.
            if ui
                .put(
                    egui::Rect::from_min_size(
                        egui::pos2(
                            frame_rect.max.x - (button_size.x / 2.0) - 15.0,
                            frame_rect.min.y - (button_size.y / 2.0) + 15.0 + 50.0,
                        ),
                        button_size,
                    ),
                    egui::Button::new("🔽").small().corner_radius(5),
                )
                .clicked()
            {
                self.warrior_queue.move_down(index);
            }
        });
    }

    /// Draw a frame for a queued warrior, showing his color, name, and number of instructions.
    fn draw_queued_warrior_frame(
        &self,
        index: usize,
        ui: &mut egui::Ui,
        renderer: &Renderer,
    ) -> egui::Rect {
        let frame = egui::Frame::group(ui.style())
            .fill(egui::Color32::from_rgb(40, 40, 40))
            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
            .inner_margin(8.0)
            .show(ui, |ui| {
                if let Some(warrior) = self.warrior_queue.get(index) {
                    let warrior_id = index;
                    let warrior_name = warrior.metadata.name.as_str();
                    let warrior_color = color::get_egui_color32(Some(warrior_id));
                    let num_instructions = warrior.instructions.len();

                    ui.set_min_width(LEFT_SIDEBAR_WIDTH - 40.0);

                    ui.horizontal(|ui| {
                        // Draw a colored square indicating this warrior's color.
                        let (rect, _) =
                            ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, 2.0, warrior_color);

                        ui.add_space(4.0);

                        // Draw "Warrior X".
                        ui.colored_label(egui::Color32::WHITE, "Warrior");
                        ui.colored_label(
                            egui::Color32::WHITE,
                            renderer.usize_to_str(warrior_id.as_display_id()),
                        );
                    });

                    ui.add_space(2.0);

                    // Draw the warrior's name.
                    ui.label(
                        egui::RichText::new(warrior_name)
                            .size(20.0)
                            .color(egui::Color32::WHITE),
                    );

                    ui.add_space(2.0);

                    // Draw number of instructions for this warrior.
                    ui.horizontal(|ui| {
                        ui.label("Instructions:");
                        ui.label(renderer.usize_to_str(num_instructions));
                    });
                }
            });

        frame.response.rect
    }

    /// Draw the right sidebar, which has 2 selector tabs to show either the `warrior_vault` or `config` page.
    fn draw_right_sidebar(&mut self, egui_ctx: &egui::Context, game_ctx: &mut GameContext) {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum RightSidebarTab {
            WarriorVault,
            Config,
        }

        let tab_state_id = egui::Id::new("right_sidebar_tab_state");

        egui::SidePanel::right("right_inspector")
            .exact_width(RIGHT_SIDEBAR_WIDTH)
            .resizable(false)
            .show(egui_ctx, |ui| {
                let current_tab = ui.data_mut(|d| {
                    *d.get_persisted_mut_or_insert_with(tab_state_id, || {
                        RightSidebarTab::WarriorVault
                    })
                });

                // Draw a row of tabs.
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(current_tab == RightSidebarTab::WarriorVault, "Warriors")
                        .clicked()
                    {
                        ui.data_mut(|d| {
                            d.insert_persisted(tab_state_id, RightSidebarTab::WarriorVault);
                        });
                    }

                    if ui
                        .selectable_label(current_tab == RightSidebarTab::Config, "Settings")
                        .clicked()
                    {
                        ui.data_mut(|d| d.insert_persisted(tab_state_id, RightSidebarTab::Config));
                    }
                });

                ui.separator();

                // Draw content for the selected tab.
                egui::ScrollArea::vertical().auto_shrink([false; 2]).show(
                    ui,
                    |ui| match current_tab {
                        RightSidebarTab::WarriorVault => {
                            self.draw_warrior_vault_page(
                                ui,
                                &game_ctx.renderer,
                                &mut game_ctx.warrior_vault,
                            );
                        }
                        RightSidebarTab::Config => {
                            Self::draw_config_page(
                                ui,
                                &game_ctx.renderer,
                                &mut game_ctx.config_manager,
                            );
                        }
                    },
                );
            });
    }

    /// Set `warrior` as the current warrior, and update widgets to display his code.
    fn load_current_warrior(&mut self, warrior: Warrior) {
        self.text_editor.input_text.clone_from(&warrior.redcode);
        self.text_editor.update_line_numbers_col_if_changed();
        self.current_warrior = Some(warrior);
        self.console_text.clear();
    }

    /// Draw a page showing the warriors in `warrior_vault`.
    fn draw_warrior_vault_page(
        &mut self,
        ui: &mut egui::Ui,
        renderer: &Renderer,
        warrior_vault: &mut WarriorVault,
    ) {
        ui.heading("Warrior Vault");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("warrior_vault_table")
                .num_columns(3)
                .striped(true)
                .spacing([25.0, 10.0])
                .show(ui, |ui| {
                    // Draw the header row.
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.set_min_width(140.0);
                        ui.heading("Name");
                    });
                    ui.heading("Len");
                    ui.end_row();

                    // Draw data rows.
                    for index in 0..warrior_vault.len() {
                        self.draw_warrior_vault_row(index, ui, renderer, warrior_vault);
                    }
                });
        });
    }

    /// Draw a warrior in `warrior_vault`, showing his details, and with buttons to load or remove this warrior.
    fn draw_warrior_vault_row(
        &mut self,
        index: usize,
        ui: &mut egui::Ui,
        renderer: &Renderer,
        warrior_vault: &mut WarriorVault,
    ) {
        if let Some(warrior) = warrior_vault.get(index) {
            if ui.selectable_label(false, &warrior.metadata.name).clicked() {
                self.load_current_warrior(warrior.clone());
            }

            ui.allocate_ui_with_layout(
                egui::vec2(50.0, ui.available_height()),
                egui::Layout::centered_and_justified(egui::Direction::RightToLeft),
                |ui| {
                    ui.label(renderer.usize_to_str(warrior.instructions.len()));
                },
            );

            // 2. Add a red, small button for deletion
            let button = egui::Button::new(egui::RichText::new("❌").size(12.0));

            if ui.add(button).clicked() {
                warrior_vault.remove(index);
            }

            ui.end_row();
        }
    }

    /// Draw the config page, showing several dropdown selectors for the features in `config_manager`.
    fn draw_config_page(
        ui: &mut egui::Ui,
        renderer: &Renderer,
        config_manager: &mut ConfigManager,
    ) {
        const SPACE_BETWEEN_FEATURES: f32 = 10.0;

        let full_width = ui.available_width();

        ui.heading("Settings");
        ui.separator();

        ui.label("Core Dimension");

        egui::ComboBox::from_id_salt("core_dimension_dropdown")
            .selected_text(config_manager.selected_core_dimension.as_str())
            .width(full_width) // Force the ComboBox button to stretch entirely
            .show_ui(ui, |ui| {
                for dimension in &config_manager.available_core_dimensions {
                    ui.selectable_value(
                        &mut config_manager.selected_core_dimension,
                        *dimension,
                        dimension.as_str(),
                    );
                }
            });

        ui.add_space(SPACE_BETWEEN_FEATURES);

        ui.label("Core Initialization Strategy");

        egui::ComboBox::from_id_salt("core_initialization_strategy_dropdown")
            .selected_text(
                config_manager
                    .selected_core_initialization_strategy
                    .as_str(),
            )
            .width(full_width) // Force the ComboBox button to stretch entirely
            .show_ui(ui, |ui| {
                for strategy in &config_manager.available_core_initialization_strategies {
                    ui.selectable_value(
                        &mut config_manager.selected_core_initialization_strategy,
                        *strategy,
                        strategy.as_str(),
                    );
                }
            });

        ui.add_space(SPACE_BETWEEN_FEATURES);

        ui.label("Task Queue Capacity");

        egui::ComboBox::from_id_salt("task_queue_capacity_dropdown")
            .selected_text(renderer.usize_to_str(config_manager.selected_task_queue_capacity))
            .width(full_width) // Force the ComboBox button to stretch entirely
            .show_ui(ui, |ui| {
                for &capacity in &config_manager.available_task_queue_capacities {
                    ui.selectable_value(
                        &mut config_manager.selected_task_queue_capacity,
                        capacity,
                        renderer.usize_to_str(capacity),
                    );
                }
            });

        ui.add_space(SPACE_BETWEEN_FEATURES);

        ui.label("Turn Limit");

        egui::ComboBox::from_id_salt("turn_limit_dropdown")
            .selected_text(renderer.usize_to_str(config_manager.selected_turn_limit))
            .width(full_width) // Force the ComboBox button to stretch entirely
            .show_ui(ui, |ui| {
                for &limit in &config_manager.available_turn_limits {
                    ui.selectable_value(
                        &mut config_manager.selected_turn_limit,
                        limit,
                        renderer.usize_to_str(limit),
                    );
                }
            });

        ui.add_space(SPACE_BETWEEN_FEATURES);

        ui.label("Warrior Separation Strategy");

        egui::ComboBox::from_id_salt("warrior_separation_strategy_dropdown")
            .selected_text(config_manager.selected_warrior_separation_strategy.as_str())
            .width(full_width) // Force the ComboBox button to stretch entirely
            .show_ui(ui, |ui| {
                for strategy in &config_manager.available_warrior_separation_strategies {
                    ui.selectable_value(
                        &mut config_manager.selected_warrior_separation_strategy,
                        *strategy,
                        strategy.as_str(),
                    );
                }
            });

        ui.add_space(SPACE_BETWEEN_FEATURES);

        ui.add_enabled_ui(
            config_manager.selected_warrior_separation_strategy
                == WarriorSeparationStrategy::Random,
            |ui| {
                ui.label("Minimum Distance between Warriors");

                egui::ComboBox::from_id_salt("min_distance_between_warriors_dropdown")
                    .selected_text(
                        renderer
                            .usize_to_str(config_manager.selected_min_distance_between_warriors),
                    )
                    .width(full_width) // Force the ComboBox button to stretch entirely
                    .show_ui(ui, |ui| {
                        for &distances in &config_manager.available_min_distance_between_warriors {
                            ui.selectable_value(
                                &mut config_manager.selected_min_distance_between_warriors,
                                distances,
                                renderer.usize_to_str(distances),
                            );
                        }
                    });
            },
        );
    }
}
