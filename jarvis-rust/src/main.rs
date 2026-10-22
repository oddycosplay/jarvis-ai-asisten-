#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Color32, CornerRadius, Frame, Margin, Stroke, Vec2};
use std::time::{Duration, Instant};

const COLOR_BG_DARK: Color32 = Color32::from_rgb(10, 14, 20);
const COLOR_PANEL_BG: Color32 = Color32::from_rgba_premultiplied(15, 20, 26, 245);
const COLOR_CARD_BG: Color32 = Color32::from_rgba_premultiplied(22, 29, 38, 230);
const COLOR_GOLD: Color32 = Color32::from_rgb(224, 194, 120);
const COLOR_GOLD_BG: Color32 = Color32::from_rgba_premultiplied(224, 194, 120, 38);
const COLOR_MUTED: Color32 = Color32::from_rgb(139, 148, 158);
const COLOR_BORDER: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 22);

#[derive(Clone, Copy, PartialEq, Eq)]
enum IslandMode {
    Mini,
    ContextPlayer,
    Expanded,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DashboardTab {
    Controls,
    Today,
    Stats,
    Clipboard,
    Apps,
    Wallpaper,
    Power,
}

#[derive(Clone)]
struct NotificationItem {
    id: usize,
    app: String,
    title: String,
    body: String,
    time: String,
    color: Color32,
}

#[derive(Clone)]
struct TodoItem {
    text: String,
    completed: bool,
}

#[derive(Clone)]
struct ClipboardEntry {
    title: String,
    content: String,
    item_type: String,
    size: String,
    time: String,
    badge_color: Color32,
}

#[derive(Clone)]
struct AppEntry {
    name: String,
    category: String,
    badge: String,
    color: Color32,
}

struct JarvisOsApp {
    mode: IslandMode,
    active_tab: DashboardTab,
    show_trigger_menu: bool,
    show_ai_modal: bool,
    ai_model: String,
    ai_input: String,
    ai_messages: Vec<(String, String)>,
    status_toast: Option<(String, Instant)>,

    // Controls Tab State
    wifi_on: bool,
    bluetooth_on: bool,
    audio_on: bool,
    nightlight_on: bool,
    darkmode_on: bool,
    volume: f32,
    is_playing: bool,
    media_progress: f32,

    // Today Tab State
    clock_theme: usize,
    selected_day: String,
    timer_duration_secs: u64,
    timer_remaining_secs: u64,
    timer_is_running: bool,
    last_timer_tick: Instant,
    todo_input: String,
    todos: Vec<TodoItem>,

    // Stats Tab State
    notifications: Vec<NotificationItem>,
    cpu_usage: f32,
    ram_usage: f32,
    disk_free_gb: u32,
    last_perf_tick: Instant,

    // Clipboard Tab State
    clipboard_search: String,
    selected_clip_idx: usize,
    clipboard_items: Vec<ClipboardEntry>,

    // Apps Tab State
    app_search: String,
    app_category: String,
    apps: Vec<AppEntry>,

    // Wallpaper Tab State
    wallpaper_idx: usize,
    wallpaper_prompt: String,
    wallpaper_names: Vec<String>,
}

impl Default for JarvisOsApp {
    fn default() -> Self {
        Self {
            mode: IslandMode::Expanded,
            active_tab: DashboardTab::Controls,
            show_trigger_menu: false,
            show_ai_modal: false,
            ai_model: "Google Gemini 2.5 Flash".to_string(),
            ai_input: String::new(),
            ai_messages: vec![
                (
                    "JARVIS".to_string(),
                    "Halo Sir! Selamat datang di JARVIS OS Native Desktop. Pilih model AI di atas atau berikan perintah untuk mengatur sistem.".to_string(),
                ),
            ],
            status_toast: None,

            wifi_on: true,
            bluetooth_on: false,
            audio_on: true,
            nightlight_on: false,
            darkmode_on: true,
            volume: 28.0,
            is_playing: true,
            media_progress: 0.15,

            clock_theme: 0,
            selected_day: "29".to_string(),
            timer_duration_secs: 300,
            timer_remaining_secs: 300,
            timer_is_running: false,
            last_timer_tick: Instant::now(),
            todo_input: String::new(),
            todos: vec![
                TodoItem {
                    text: "Setup JARVIS native Rust GUI application".to_string(),
                    completed: true,
                },
                TodoItem {
                    text: "Test quick toggles & AI assistant chat".to_string(),
                    completed: false,
                },
                TodoItem {
                    text: "Monitor hardware performance metrics".to_string(),
                    completed: false,
                },
            ],

            notifications: vec![
                NotificationItem {
                    id: 1,
                    app: "WhatsApp".to_string(),
                    title: "WhatsApp • Arsh".to_string(),
                    body: "Hey Mainak! Are we still on for coffee at 10 AM?".to_string(),
                    time: "2m ago".to_string(),
                    color: Color32::from_rgb(37, 211, 102),
                },
                NotificationItem {
                    id: 2,
                    app: "Twitter / X".to_string(),
                    title: "Twitter / X • Mention".to_string(),
                    body: "@openai posted a new update regarding Gemini 2.5 flash speed.".to_string(),
                    time: "15m ago".to_string(),
                    color: Color32::from_rgb(29, 161, 242),
                },
                NotificationItem {
                    id: 3,
                    app: "Windows Update".to_string(),
                    title: "Windows Update".to_string(),
                    body: "Cumulative update successfully installed. System ready.".to_string(),
                    time: "1h ago".to_string(),
                    color: Color32::from_rgb(168, 85, 247),
                },
            ],
            cpu_usage: 24.0,
            ram_usage: 6.8,
            disk_free_gb: 142,
            last_perf_tick: Instant::now(),

            clipboard_search: String::new(),
            selected_clip_idx: 0,
            clipboard_items: vec![
                ClipboardEntry {
                    title: "Hey Arsh, what's up? How about coffee...".to_string(),
                    content: "Hey Arsh, what's up? How about we go get some coffee at 10 in the morning, and then hang out at the park for a bit?".to_string(),
                    item_type: "Text".to_string(),
                    size: "108 chars • 22 words".to_string(),
                    time: "Today, 9:41 AM".to_string(),
                    badge_color: COLOR_GOLD,
                },
                ClipboardEntry {
                    title: "https://www.amazon.in/s?k=red+boxing".to_string(),
                    content: "https://www.amazon.in/s?k=red+boxing+gloves".to_string(),
                    item_type: "Link".to_string(),
                    size: "42 chars".to_string(),
                    time: "Today, 9:37 AM".to_string(),
                    badge_color: Color32::from_rgb(56, 189, 248),
                },
                ClipboardEntry {
                    title: "cargo build --release".to_string(),
                    content: "cargo build --release --manifest-path Cargo.toml".to_string(),
                    item_type: "Command".to_string(),
                    size: "48 chars".to_string(),
                    time: "Today, 8:41 AM".to_string(),
                    badge_color: Color32::from_rgb(192, 132, 252),
                },
            ],

            app_search: String::new(),
            app_category: "All".to_string(),
            apps: vec![
                AppEntry {
                    name: "Microsoft Word".to_string(),
                    category: "Office".to_string(),
                    badge: "Office".to_string(),
                    color: Color32::from_rgb(43, 87, 154),
                },
                AppEntry {
                    name: "Microsoft Excel".to_string(),
                    category: "Office".to_string(),
                    badge: "Office".to_string(),
                    color: Color32::from_rgb(33, 115, 70),
                },
                AppEntry {
                    name: "Visual Studio Code".to_string(),
                    category: "Coding".to_string(),
                    badge: "Running".to_string(),
                    color: Color32::from_rgb(0, 122, 204),
                },
                AppEntry {
                    name: "GitHub Desktop".to_string(),
                    category: "Coding".to_string(),
                    badge: "Git".to_string(),
                    color: Color32::from_rgb(36, 41, 46),
                },
                AppEntry {
                    name: "Google Chrome".to_string(),
                    category: "Media".to_string(),
                    badge: "Browser".to_string(),
                    color: Color32::from_rgb(66, 133, 244),
                },
                AppEntry {
                    name: "Spotify".to_string(),
                    category: "Media".to_string(),
                    badge: "Music".to_string(),
                    color: Color32::from_rgb(29, 185, 84),
                },
                AppEntry {
                    name: "Discord".to_string(),
                    category: "Media".to_string(),
                    badge: "Chat".to_string(),
                    color: Color32::from_rgb(88, 101, 242),
                },
                AppEntry {
                    name: "Notion".to_string(),
                    category: "Office".to_string(),
                    badge: "Notes".to_string(),
                    color: Color32::from_rgb(30, 30, 30),
                },
            ],

            wallpaper_idx: 0,
            wallpaper_prompt: String::new(),
            wallpaper_names: vec![
                "Cyberpunk Neon Tokyo Night".to_string(),
                "Futuristic Minimal Dark Mountain".to_string(),
                "JARVIS AI Holographic Grid".to_string(),
                "Deep Space Nebula & Stars".to_string(),
            ],
        }
    }
}

impl JarvisOsApp {
    fn show_toast(&mut self, text: impl Into<String>) {
        self.status_toast = Some((text.into(), Instant::now()));
    }

    fn update_background_tasks(&mut self) {
        // Update timer
        if self.timer_is_running {
            let now = Instant::now();
            let elapsed = now.duration_since(self.last_timer_tick);
            if elapsed >= Duration::from_secs(1) {
                let secs = elapsed.as_secs();
                if self.timer_remaining_secs > secs {
                    self.timer_remaining_secs -= secs;
                } else {
                    self.timer_remaining_secs = 0;
                    self.timer_is_running = false;
                    self.show_toast("Pomodoro Timer selesai!");
                }
                self.last_timer_tick = now;
            }
        }

        // Update simulate hardware stats
        if self.last_perf_tick.elapsed() >= Duration::from_millis(2500) {
            self.last_perf_tick = Instant::now();
            self.cpu_usage = ((self.cpu_usage + 3.7) % 35.0).max(14.0);
            self.ram_usage = ((self.ram_usage + 0.2) % 3.0) + 6.2;
        }

        // Clean expired toast
        if let Some((_, created_at)) = self.status_toast {
            if created_at.elapsed() > Duration::from_secs(4) {
                self.status_toast = None;
            }
        }
    }
}

impl eframe::App for JarvisOsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_background_tasks();
        ctx.request_repaint_after(Duration::from_millis(500));

        // Dark background style
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = COLOR_BG_DARK;
        visuals.window_fill = COLOR_PANEL_BG;
        visuals.window_stroke = Stroke::new(1.0_f32, COLOR_BORDER);
        ctx.set_visuals(visuals);

        egui::CentralPanel::default()
            .frame(Frame::NONE.fill(COLOR_BG_DARK))
            .show(ctx, |ui| {
                // Top Island Bar / Mascot
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    match self.mode {
                        IslandMode::Mini => {
                            let ghost_btn = egui::Button::new(
                                egui::RichText::new("  JARVIS MINI ISLAND (Click to Expand)  ")
                                    .color(COLOR_GOLD)
                                    .strong(),
                            )
                            .fill(COLOR_PANEL_BG)
                            .stroke(Stroke::new(1.0_f32, COLOR_GOLD));

                            if ui.add(ghost_btn).clicked() {
                                self.mode = IslandMode::Expanded;
                            }
                        }
                        IslandMode::ContextPlayer => {
                            self.render_context_popup(ui);
                        }
                        IslandMode::Expanded => {
                            // Header Mode & Collapse Button
                            ui.horizontal(|ui| {
                                ui.add_space(20.0);
                                if ui
                                    .button(
                                        egui::RichText::new("Mockup Trigger Menu")
                                            .color(Color32::WHITE),
                                    )
                                    .clicked()
                                {
                                    self.show_trigger_menu = !self.show_trigger_menu;
                                }

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.add_space(20.0);
                                        if ui
                                            .button(
                                                egui::RichText::new("Mini Island Mode")
                                                    .color(COLOR_MUTED),
                                            )
                                            .clicked()
                                        {
                                            self.mode = IslandMode::Mini;
                                        }

                                        if ui
                                            .button(
                                                egui::RichText::new("AI Assistant")
                                                    .color(COLOR_GOLD)
                                                    .strong(),
                                            )
                                            .clicked()
                                        {
                                            self.show_ai_modal = true;
                                        }

                                        if ui
                                            .button(
                                                egui::RichText::new("Context Media")
                                                    .color(COLOR_MUTED),
                                            )
                                            .clicked()
                                        {
                                            self.mode = IslandMode::ContextPlayer;
                                        }
                                    },
                                );
                            });

                            ui.add_space(8.0);
                            self.render_expanded_panel(ui);
                        }
                    }
                });

                // Status Toast
                if let Some((msg, _)) = &self.status_toast {
                    ui.add_space(10.0);
                    ui.vertical_centered(|ui| {
                        Frame::NONE
                            .fill(COLOR_CARD_BG)
                            .stroke(Stroke::new(1.0_f32, COLOR_GOLD))
                            .corner_radius(CornerRadius::same(10))
                            .inner_margin(Margin::symmetric(16, 8))
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new(msg).color(COLOR_GOLD).strong());
                            });
                    });
                }
            });

        // Trigger Menu Window
        if self.show_trigger_menu {
            egui::Window::new("JARVIS Mockup Controls")
                .anchor(egui::Align2::LEFT_TOP, Vec2::new(20.0, 50.0))
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    if ui.button("Mode Compact Island").clicked() {
                        self.mode = IslandMode::Mini;
                        self.show_trigger_menu = false;
                    }
                    if ui.button("Mode Contextual Player").clicked() {
                        self.mode = IslandMode::ContextPlayer;
                        self.show_trigger_menu = false;
                    }
                    if ui.button("Mode Expanded Panel").clicked() {
                        self.mode = IslandMode::Expanded;
                        self.show_trigger_menu = false;
                    }
                    ui.separator();
                    if ui.button("Dictation (Voice Mode)").clicked() {
                        self.show_toast("Voice Dictation aktif. Mulai berbicara...");
                        self.show_trigger_menu = false;
                    }
                    if ui.button("Vision + Redact Mode").clicked() {
                        self.show_toast("Vision + Redact aktif. Mode deteksi area layar.");
                        self.show_trigger_menu = false;
                    }
                    if ui.button("Open AI Assistant Chat").clicked() {
                        self.show_ai_modal = true;
                        self.show_trigger_menu = false;
                    }
                    if ui.button("Tutup Menu").clicked() {
                        self.show_trigger_menu = false;
                    }
                });
        }

        // AI Assistant Modal Window
        if self.show_ai_modal {
            egui::Window::new("JARVIS AI Assistant")
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .default_size(Vec2::new(620.0, 480.0))
                .resizable(true)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Model AI:")
                                .color(COLOR_GOLD)
                                .strong(),
                        );
                        egui::ComboBox::from_id_salt("ai_model_selector")
                            .selected_text(&self.ai_model)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.ai_model,
                                    "Google Gemini 2.5 Flash".to_string(),
                                    "Google Gemini 2.5 Flash",
                                );
                                ui.selectable_value(
                                    &mut self.ai_model,
                                    "Claude 3.5 Sonnet".to_string(),
                                    "Claude 3.5 Sonnet",
                                );
                                ui.selectable_value(
                                    &mut self.ai_model,
                                    "GPT-4o OpenAI".to_string(),
                                    "GPT-4o OpenAI",
                                );
                            });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Tutup").clicked() {
                                self.show_ai_modal = false;
                            }
                        });
                    });

                    ui.separator();

                    // Chat messages list
                    egui::ScrollArea::vertical()
                        .max_height(320.0)
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            for (sender, text) in &self.ai_messages {
                                ui.horizontal(|ui| {
                                    if sender == "JARVIS" {
                                        ui.label(
                                            egui::RichText::new("[JARVIS]:")
                                                .color(COLOR_GOLD)
                                                .strong(),
                                        );
                                        ui.label(egui::RichText::new(text).color(Color32::WHITE));
                                    } else {
                                        ui.label(
                                            egui::RichText::new("[YOU]:")
                                                .color(Color32::LIGHT_BLUE)
                                                .strong(),
                                        );
                                        ui.label(egui::RichText::new(text).color(Color32::LIGHT_GRAY));
                                    }
                                });
                                ui.add_space(4.0);
                            }
                        });

                    ui.separator();

                    // Input & Send button
                    ui.horizontal(|ui| {
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut self.ai_input)
                                .hint_text("Ketik perintah atau tanya AI... (Enter untuk kirim)")
                                .desired_width(480.0),
                        );

                        let send_pressed = ui
                            .button(egui::RichText::new("Send").color(COLOR_GOLD).strong())
                            .clicked()
                            || (response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter)));

                        if send_pressed && !self.ai_input.trim().is_empty() {
                            let user_msg = self.ai_input.trim().to_string();
                            self.ai_messages.push(("USER".to_string(), user_msg.clone()));
                            self.ai_input.clear();

                            // Response otomatis dari model
                            let reply = format!(
                                "Perintah \"{}\" diproses via {}. Status: Sistem berjalan optimal.",
                                user_msg, self.ai_model
                            );
                            self.ai_messages.push(("JARVIS".to_string(), reply));
                        }
                    });
                });
        }
    }
}

impl JarvisOsApp {
    fn render_context_popup(&mut self, ui: &mut egui::Ui) {
        Frame::NONE
            .fill(COLOR_PANEL_BG)
            .stroke(Stroke::new(1.0_f32, COLOR_GOLD))
            .corner_radius(CornerRadius::same(20))
            .inner_margin(Margin::same(20))
            .show(ui, |ui| {
                ui.set_width(380.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("lofi beats to study and relax")
                            .color(Color32::WHITE)
                            .strong()
                            .size(15.0),
                    );
                    ui.label(egui::RichText::new("lofi radio - 24/7 stream").color(COLOR_MUTED));
                    ui.add_space(8.0);

                    // Media Slider
                    ui.add(
                        egui::Slider::new(&mut self.media_progress, 0.0..=1.0)
                            .show_value(false)
                            .text("00:50 / 62:15"),
                    );
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(16.0, 0.0);
                        if ui.button("<< Prev").clicked() {
                            self.show_toast("Track sebelumnya dimainkan");
                        }

                        let play_label = if self.is_playing { "|| Pause" } else { "> Play" };
                        if ui
                            .button(egui::RichText::new(play_label).color(COLOR_GOLD).strong())
                            .clicked()
                        {
                            self.is_playing = !self.is_playing;
                        }

                        if ui.button("Next >>").clicked() {
                            self.show_toast("Track selanjutnya dimainkan");
                        }
                    });

                    ui.add_space(10.0);
                    if ui.button("Buka Panel Lengkap").clicked() {
                        self.mode = IslandMode::Expanded;
                    }
                });
            });
    }

    fn render_expanded_panel(&mut self, ui: &mut egui::Ui) {
        Frame::NONE
            .fill(COLOR_PANEL_BG)
            .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
            .corner_radius(CornerRadius::same(24))
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                ui.set_max_width(900.0);

                // Navigation Tabs
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(8.0, 0.0);
                    let tabs = [
                        (DashboardTab::Controls, "Controls"),
                        (DashboardTab::Today, "Today"),
                        (DashboardTab::Stats, "Stats"),
                        (DashboardTab::Clipboard, "Clipboard"),
                        (DashboardTab::Apps, "Apps"),
                        (DashboardTab::Wallpaper, "Wallpaper"),
                        (DashboardTab::Power, "Power"),
                    ];

                    for (tab, name) in tabs {
                        let is_selected = self.active_tab == tab;
                        let text = egui::RichText::new(name)
                            .color(if is_selected { COLOR_GOLD } else { COLOR_MUTED })
                            .strong();

                        let btn = egui::Button::new(text).fill(if is_selected {
                            COLOR_GOLD_BG
                        } else {
                            Color32::TRANSPARENT
                        });

                        if ui.add(btn).clicked() {
                            self.active_tab = tab;
                        }
                    }
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                // Tab Content Rendering
                match self.active_tab {
                    DashboardTab::Controls => self.render_controls_tab(ui),
                    DashboardTab::Today => self.render_today_tab(ui),
                    DashboardTab::Stats => self.render_stats_tab(ui),
                    DashboardTab::Clipboard => self.render_clipboard_tab(ui),
                    DashboardTab::Apps => self.render_apps_tab(ui),
                    DashboardTab::Wallpaper => self.render_wallpaper_tab(ui),
                    DashboardTab::Power => self.render_power_tab(ui),
                }
            });
    }

    fn render_controls_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Quick Toggles Grid
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(12.0, 0.0);

                // Wi-Fi
                let wifi_txt = if self.wifi_on { "Wi-Fi: ON (Home 5G)" } else { "Wi-Fi: OFF" };
                let wifi_fill = if self.wifi_on { COLOR_GOLD_BG } else { COLOR_CARD_BG };
                if ui
                    .add(
                        egui::Button::new(egui::RichText::new(wifi_txt).color(Color32::WHITE))
                            .fill(wifi_fill)
                            .min_size(Vec2::new(160.0, 48.0)),
                    )
                    .clicked()
                {
                    self.wifi_on = !self.wifi_on;
                }

                // Bluetooth
                let bt_txt = if self.bluetooth_on { "Bluetooth: ON" } else { "Bluetooth: OFF" };
                let bt_fill = if self.bluetooth_on { COLOR_GOLD_BG } else { COLOR_CARD_BG };
                if ui
                    .add(
                        egui::Button::new(egui::RichText::new(bt_txt).color(Color32::WHITE))
                            .fill(bt_fill)
                            .min_size(Vec2::new(160.0, 48.0)),
                    )
                    .clicked()
                {
                    self.bluetooth_on = !self.bluetooth_on;
                }

                // Audio
                let audio_txt = if self.audio_on { "Audio: Speakers" } else { "Audio: Mute" };
                let audio_fill = if self.audio_on { COLOR_GOLD_BG } else { COLOR_CARD_BG };
                if ui
                    .add(
                        egui::Button::new(egui::RichText::new(audio_txt).color(Color32::WHITE))
                            .fill(audio_fill)
                            .min_size(Vec2::new(160.0, 48.0)),
                    )
                    .clicked()
                {
                    self.audio_on = !self.audio_on;
                }
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(12.0, 0.0);

                // Night Light
                let nl_txt = if self.nightlight_on { "Night Light: ON" } else { "Night Light: OFF" };
                let nl_fill = if self.nightlight_on { COLOR_GOLD_BG } else { COLOR_CARD_BG };
                if ui
                    .add(
                        egui::Button::new(egui::RichText::new(nl_txt).color(Color32::WHITE))
                            .fill(nl_fill)
                            .min_size(Vec2::new(245.0, 44.0)),
                    )
                    .clicked()
                {
                    self.nightlight_on = !self.nightlight_on;
                }

                // Dark Mode
                let dm_txt = if self.darkmode_on { "Dark Mode: ON" } else { "Dark Mode: OFF" };
                let dm_fill = if self.darkmode_on { COLOR_GOLD_BG } else { COLOR_CARD_BG };
                if ui
                    .add(
                        egui::Button::new(egui::RichText::new(dm_txt).color(Color32::WHITE))
                            .fill(dm_fill)
                            .min_size(Vec2::new(245.0, 44.0)),
                    )
                    .clicked()
                {
                    self.darkmode_on = !self.darkmode_on;
                }
            });

            ui.add_space(12.0);

            // Volume Slider
            Frame::NONE
                .fill(COLOR_CARD_BG)
                .corner_radius(CornerRadius::same(12))
                .inner_margin(Margin::same(12))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Master Volume:").color(Color32::WHITE));
                        ui.add(
                            egui::Slider::new(&mut self.volume, 0.0..=100.0)
                                .text("%")
                                .suffix("%"),
                        );
                    });
                });

            ui.add_space(12.0);

            // System Utilities Action Buttons
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(10.0, 0.0);
                if ui.button("Screenshot").clicked() {
                    self.show_toast("Screenshot diambil dan disimpan ke Pictures");
                }
                if ui.button("Screen Rec").clicked() {
                    self.show_toast("Screen Recording dimulai...");
                }
                if ui.button("Voice Assistant").clicked() {
                    self.show_toast("Voice recognition aktif");
                }
                if ui.button("Lihat Stats").clicked() {
                    self.active_tab = DashboardTab::Stats;
                }
            });

            ui.add_space(12.0);

            // Integrated Media Player
            Frame::NONE
                .fill(COLOR_CARD_BG)
                .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                .corner_radius(CornerRadius::same(14))
                .inner_margin(Margin::same(14))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("lofi beats to study and relax")
                                    .color(Color32::WHITE)
                                    .strong(),
                            );
                            ui.label(
                                egui::RichText::new("lofi radio • 24/7 online").color(COLOR_MUTED),
                            );
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Next >>").clicked() {
                                self.show_toast("Skip track berikutnya");
                            }

                            let play_icon = if self.is_playing { "|| Pause" } else { "> Play" };
                            if ui
                                .button(egui::RichText::new(play_icon).color(COLOR_GOLD).strong())
                                .clicked()
                            {
                                self.is_playing = !self.is_playing;
                            }

                            if ui.button("<< Prev").clicked() {
                                self.show_toast("Kembali ke track sebelumnya");
                            }
                        });
                    });
                });
        });
    }

    fn render_today_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            // Kolom Kiri: Clock & Weather
            ui.vertical(|ui| {
                ui.set_width(200.0);

                // Clock Widget dengan Theme Switcher
                let (clock_bg, clock_color) = match self.clock_theme {
                    1 => (COLOR_GOLD_BG, COLOR_GOLD),
                    2 => (
                        Color32::from_rgb(18, 52, 86),
                        Color32::from_rgb(56, 189, 248),
                    ),
                    _ => (COLOR_CARD_BG, Color32::WHITE),
                };

                Frame::NONE
                    .fill(clock_bg)
                    .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                    .corner_radius(CornerRadius::same(16))
                    .inner_margin(Margin::same(16))
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new("12 : 50")
                                    .color(clock_color)
                                    .size(32.0)
                                    .strong(),
                            );
                            ui.label(
                                egui::RichText::new("PM • Tue 29 Sep").color(COLOR_MUTED).size(12.0),
                            );
                            ui.add_space(4.0);
                            if ui.button("Ubah Tema Jam").clicked() {
                                self.clock_theme = (self.clock_theme + 1) % 3;
                            }
                        });
                    });

                ui.add_space(8.0);

                // Weather Widget
                Frame::NONE
                    .fill(COLOR_CARD_BG)
                    .corner_radius(CornerRadius::same(14))
                    .inner_margin(Margin::same(12))
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new("32°C")
                                    .color(Color32::from_rgb(250, 204, 21))
                                    .size(20.0)
                                    .strong(),
                            );
                            ui.label(
                                egui::RichText::new("Mostly Clear • Bandung").color(COLOR_MUTED),
                            );
                        });
                    });
            });

            ui.add_space(14.0);

            // Kolom Tengah: Calendar & Pomodoro Timer
            ui.vertical(|ui| {
                ui.set_width(320.0);

                // Calendar Row
                Frame::NONE
                    .fill(COLOR_CARD_BG)
                    .corner_radius(CornerRadius::same(14))
                    .inner_margin(Margin::same(12))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Calendar: Sep 28 - Oct 4")
                                .color(COLOR_MUTED)
                                .size(11.0),
                        );
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            let days = ["28", "29", "30", "1", "2", "3", "4"];
                            for day in days {
                                let is_active = self.selected_day == day;
                                let btn_text = egui::RichText::new(day).color(if is_active {
                                    COLOR_GOLD
                                } else {
                                    Color32::WHITE
                                });
                                let fill = if is_active { COLOR_GOLD_BG } else { Color32::TRANSPARENT };

                                if ui.add(egui::Button::new(btn_text).fill(fill)).clicked() {
                                    self.selected_day = day.to_string();
                                }
                            }
                        });

                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "Jadwal ({}) : 11:00 AM • Daily AI news research",
                                self.selected_day
                            ))
                            .color(COLOR_GOLD)
                            .size(11.0),
                        );
                    });

                ui.add_space(8.0);

                // Pomodoro Timer
                Frame::NONE
                    .fill(COLOR_CARD_BG)
                    .corner_radius(CornerRadius::same(14))
                    .inner_margin(Margin::same(12))
                    .show(ui, |ui| {
                        let mins = self.timer_remaining_secs / 60;
                        let secs = self.timer_remaining_secs % 60;
                        let timer_str = format!("{:02}:{:02}", mins, secs);

                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(timer_str)
                                    .color(COLOR_GOLD)
                                    .size(24.0)
                                    .strong(),
                            );
                            ui.add_space(10.0);
                            if ui.button("1m").clicked() {
                                self.timer_duration_secs = 60;
                                self.timer_remaining_secs = 60;
                                self.timer_is_running = false;
                            }
                            if ui.button("5m").clicked() {
                                self.timer_duration_secs = 300;
                                self.timer_remaining_secs = 300;
                                self.timer_is_running = false;
                            }
                            if ui.button("15m").clicked() {
                                self.timer_duration_secs = 900;
                                self.timer_remaining_secs = 900;
                                self.timer_is_running = false;
                            }
                        });

                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            let start_label = if self.timer_is_running { "Pause" } else { "Start" };
                            if ui
                                .button(egui::RichText::new(start_label).color(COLOR_GOLD).strong())
                                .clicked()
                            {
                                self.timer_is_running = !self.timer_is_running;
                                self.last_timer_tick = Instant::now();
                            }
                            if ui.button("Reset").clicked() {
                                self.timer_remaining_secs = self.timer_duration_secs;
                                self.timer_is_running = false;
                            }
                        });
                    });
            });

            ui.add_space(14.0);

            // Kolom Kanan: To-Do List Dinamis
            ui.vertical(|ui| {
                ui.set_width(280.0);
                Frame::NONE
                    .fill(COLOR_CARD_BG)
                    .corner_radius(CornerRadius::same(14))
                    .inner_margin(Margin::same(14))
                    .show(ui, |ui| {
                        let completed_count = self.todos.iter().filter(|t| t.completed).count();
                        ui.label(
                            egui::RichText::new(format!(
                                "To-Do List ({}/{})",
                                completed_count,
                                self.todos.len()
                            ))
                            .color(Color32::WHITE)
                            .strong(),
                        );
                        ui.add_space(8.0);

                        egui::ScrollArea::vertical()
                            .max_height(140.0)
                            .show(ui, |ui| {
                                for item in &mut self.todos {
                                    let text = if item.completed {
                                        egui::RichText::new(&item.text)
                                            .strikethrough()
                                            .color(COLOR_MUTED)
                                    } else {
                                        egui::RichText::new(&item.text).color(Color32::WHITE)
                                    };
                                    ui.checkbox(&mut item.completed, text);
                                }
                            });

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.todo_input)
                                    .hint_text("Tambah tugas...")
                                    .desired_width(190.0),
                            );
                            if ui.button("+").clicked() && !self.todo_input.trim().is_empty() {
                                self.todos.push(TodoItem {
                                    text: self.todo_input.trim().to_string(),
                                    completed: false,
                                });
                                self.todo_input.clear();
                            }
                        });
                    });
            });
        });
    }

    fn render_stats_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            // Left Column: Windows OS Notifications
            ui.vertical(|ui| {
                ui.set_width(480.0);
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("WINDOWS OS NOTIFICATIONS")
                            .color(Color32::WHITE)
                            .strong(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Clear All").clicked() {
                            self.notifications.clear();
                            self.show_toast("Semua notifikasi dihapus");
                        }
                    });
                });

                ui.add_space(8.0);

                if self.notifications.is_empty() {
                    ui.label(egui::RichText::new("Tidak ada notifikasi baru.").color(COLOR_MUTED));
                } else {
                    let mut to_remove = None;
                    for (idx, notif) in self.notifications.iter().enumerate() {
                        Frame::NONE
                            .fill(COLOR_CARD_BG)
                            .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                            .corner_radius(CornerRadius::same(12))
                            .inner_margin(Margin::same(10))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                egui::RichText::new(&notif.title)
                                                    .color(notif.color)
                                                    .strong(),
                                            );
                                            ui.label(
                                                egui::RichText::new(&notif.time)
                                                    .color(COLOR_MUTED)
                                                    .size(10.0),
                                            );
                                        });
                                        ui.label(
                                            egui::RichText::new(&notif.body)
                                                .color(Color32::LIGHT_GRAY)
                                                .size(11.0),
                                        );
                                    });

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if ui.button("X").clicked() {
                                                to_remove = Some(idx);
                                            }
                                        },
                                    );
                                });
                            });
                        ui.add_space(4.0);
                    }

                    if let Some(idx) = to_remove {
                        self.notifications.remove(idx);
                    }
                }
            });

            ui.add_space(20.0);

            // Right Column: Laptop Performance Monitor
            ui.vertical(|ui| {
                ui.set_width(320.0);
                ui.label(
                    egui::RichText::new("HARDWARE PERFORMANCE")
                        .color(COLOR_GOLD)
                        .strong(),
                );
                ui.add_space(8.0);

                // CPU
                Frame::NONE
                    .fill(COLOR_CARD_BG)
                    .corner_radius(CornerRadius::same(12))
                    .inner_margin(Margin::same(12))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("CPU Usage").color(Color32::WHITE));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    egui::RichText::new(format!("{:.1}%", self.cpu_usage))
                                        .color(Color32::from_rgb(56, 189, 248))
                                        .strong(),
                                );
                            });
                        });
                        ui.add(
                            egui::ProgressBar::new(self.cpu_usage / 100.0)
                                .fill(Color32::from_rgb(56, 189, 248)),
                        );
                    });

                ui.add_space(8.0);

                // RAM
                Frame::NONE
                    .fill(COLOR_CARD_BG)
                    .corner_radius(CornerRadius::same(12))
                    .inner_margin(Margin::same(12))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Memory (RAM)").color(Color32::WHITE));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    egui::RichText::new(format!("{:.1} / 16.0 GB", self.ram_usage))
                                        .color(COLOR_GOLD)
                                        .strong(),
                                );
                            });
                        });
                        ui.add(
                            egui::ProgressBar::new(self.ram_usage / 16.0)
                                .fill(COLOR_GOLD),
                        );
                    });

                ui.add_space(8.0);

                // SSD
                Frame::NONE
                    .fill(COLOR_CARD_BG)
                    .corner_radius(CornerRadius::same(12))
                    .inner_margin(Margin::same(12))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("SSD Disk").color(Color32::WHITE));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    egui::RichText::new(format!("{} GB free", self.disk_free_gb))
                                        .color(Color32::from_rgb(74, 222, 128))
                                        .strong(),
                                );
                            });
                        });
                        ui.add(
                            egui::ProgressBar::new(0.65)
                                .fill(Color32::from_rgb(74, 222, 128)),
                        );
                    });
            });
        });
    }

    fn render_clipboard_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            // Kolom Kiri: Search & Items
            ui.vertical(|ui| {
                ui.set_width(320.0);
                ui.add(
                    egui::TextEdit::singleline(&mut self.clipboard_search)
                        .hint_text("Cari riwayat clipboard..."),
                );
                ui.add_space(8.0);

                egui::ScrollArea::vertical()
                    .max_height(240.0)
                    .show(ui, |ui| {
                        for (idx, item) in self.clipboard_items.iter().enumerate() {
                            if !self.clipboard_search.is_empty()
                                && !item.title.to_lowercase().contains(&self.clipboard_search.to_lowercase())
                            {
                                continue;
                            }

                            let is_sel = self.selected_clip_idx == idx;
                            let fill = if is_sel { COLOR_GOLD_BG } else { COLOR_CARD_BG };

                            Frame::NONE
                                .fill(fill)
                                .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                                .corner_radius(CornerRadius::same(10))
                                .inner_margin(Margin::same(10))
                                .show(ui, |ui| {
                                    if ui
                                        .add(
                                            egui::Button::new(
                                                egui::RichText::new(&item.title)
                                                    .color(Color32::WHITE)
                                                    .strong(),
                                            )
                                            .fill(Color32::TRANSPARENT),
                                        )
                                        .clicked()
                                    {
                                        self.selected_clip_idx = idx;
                                    }
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new(&item.item_type)
                                                .color(item.badge_color)
                                                .size(10.0),
                                        );
                                        ui.label(
                                            egui::RichText::new(&item.time)
                                                .color(COLOR_MUTED)
                                                .size(10.0),
                                        );
                                    });
                                });
                            ui.add_space(4.0);
                        }
                    });
            });

            ui.add_space(16.0);

            // Kolom Kanan: Detail & Metadata
            ui.vertical(|ui| {
                ui.set_width(480.0);
                if let Some(item) = self.clipboard_items.get(self.selected_clip_idx) {
                    Frame::NONE
                        .fill(COLOR_CARD_BG)
                        .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                        .corner_radius(CornerRadius::same(14))
                        .inner_margin(Margin::same(16))
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new("Clipboard Content Preview")
                                    .color(COLOR_GOLD)
                                    .strong(),
                            );
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(&item.content)
                                    .color(Color32::WHITE)
                                    .size(13.0),
                            );

                            ui.add_space(16.0);
                            ui.separator();
                            ui.add_space(12.0);

                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new("Type").color(COLOR_MUTED).size(11.0));
                                    ui.label(egui::RichText::new(&item.item_type).color(item.badge_color).strong());
                                });
                                ui.add_space(30.0);
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new("Size").color(COLOR_MUTED).size(11.0));
                                    ui.label(egui::RichText::new(&item.size).color(Color32::WHITE).strong());
                                });
                                ui.add_space(30.0);
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new("Copied").color(COLOR_MUTED).size(11.0));
                                    ui.label(egui::RichText::new(&item.time).color(Color32::WHITE).strong());
                                });
                            });
                        });
                }
            });
        });
    }

    fn render_apps_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.app_search)
                        .hint_text("Cari aplikasi...")
                        .desired_width(260.0),
                );

                ui.add_space(12.0);
                let categories = ["All", "Office", "Coding", "Media"];
                for cat in categories {
                    let is_sel = self.app_category == cat;
                    let fill = if is_sel { COLOR_GOLD_BG } else { COLOR_CARD_BG };
                    let text = egui::RichText::new(cat).color(if is_sel { COLOR_GOLD } else { Color32::WHITE });
                    if ui.add(egui::Button::new(text).fill(fill)).clicked() {
                        self.app_category = cat.to_string();
                    }
                }
            });

            ui.add_space(14.0);

            // Apps Grid
            egui::ScrollArea::vertical()
                .max_height(240.0)
                .show(ui, |ui| {
                    egui::Grid::new("apps_grid")
                        .num_columns(2)
                        .spacing(Vec2::new(12.0, 10.0))
                        .show(ui, |ui| {
                            let mut col = 0;
                            for app in &self.apps {
                                if self.app_category != "All" && app.category != self.app_category {
                                    continue;
                                }
                                if !self.app_search.is_empty()
                                    && !app.name.to_lowercase().contains(&self.app_search.to_lowercase())
                                {
                                    continue;
                                }

                                Frame::NONE
                                    .fill(COLOR_CARD_BG)
                                    .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                                    .corner_radius(CornerRadius::same(12))
                                    .inner_margin(Margin::same(10))
                                    .show(ui, |ui| {
                                        ui.set_width(380.0);
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                egui::RichText::new(&app.name)
                                                    .color(Color32::WHITE)
                                                    .strong(),
                                            );
                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    ui.label(
                                                        egui::RichText::new(&app.badge)
                                                            .color(app.color)
                                                            .size(11.0),
                                                    );
                                                },
                                            );
                                        });
                                    });

                                col += 1;
                                if col % 2 == 0 {
                                    ui.end_row();
                                }
                            }
                        });
                });
        });
    }

    fn render_wallpaper_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new("WALLPAPER MANAGEMENT & AI GENERATOR")
                    .color(COLOR_GOLD)
                    .strong(),
            );
            ui.add_space(10.0);

            let current_name = &self.wallpaper_names[self.wallpaper_idx];
            Frame::NONE
                .fill(COLOR_CARD_BG)
                .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                .corner_radius(CornerRadius::same(16))
                .inner_margin(Margin::same(24))
                .show(ui, |ui| {
                    ui.set_width(520.0);
                    ui.label(
                        egui::RichText::new(format!("Active Wallpaper: {}", current_name))
                            .color(Color32::WHITE)
                            .size(16.0)
                            .strong(),
                    );
                    ui.add_space(12.0);

                    ui.horizontal(|ui| {
                        if ui.button("< Preset Sebelumnya").clicked() {
                            if self.wallpaper_idx == 0 {
                                self.wallpaper_idx = self.wallpaper_names.len() - 1;
                            } else {
                                self.wallpaper_idx -= 1;
                            }
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Preset Selanjutnya >").clicked() {
                                self.wallpaper_idx = (self.wallpaper_idx + 1) % self.wallpaper_names.len();
                            }
                        });
                    });
                });

            ui.add_space(14.0);

            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.wallpaper_prompt)
                        .hint_text("Prompt AI (misal: Cyberpunk Tokyo Night Neon)...")
                        .desired_width(360.0),
                );
                if ui.button("Generate AI").clicked() && !self.wallpaper_prompt.trim().is_empty() {
                    let new_wall = format!("AI: {}", self.wallpaper_prompt.trim());
                    self.wallpaper_names.push(new_wall);
                    self.wallpaper_idx = self.wallpaper_names.len() - 1;
                    self.wallpaper_prompt.clear();
                    self.show_toast("Wallpaper baru berhasil digenerate via AI");
                }
                if ui.button("Terapkan ke Windows").clicked() {
                    self.show_toast("Wallpaper berhasil diterapkan ke sistem");
                }
            });
        });
    }

    fn render_power_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new("SYSTEM POWER CONTROLS")
                    .color(COLOR_GOLD)
                    .strong(),
            );
            ui.add_space(16.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(16.0, 0.0);

                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("Lock Screen")
                                .color(COLOR_GOLD)
                                .size(14.0)
                                .strong(),
                        )
                        .fill(COLOR_CARD_BG)
                        .min_size(Vec2::new(160.0, 80.0)),
                    )
                    .clicked()
                {
                    self.show_toast("Sistem: Permintaan Lock Screen dijalankan");
                }

                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("Sleep Mode")
                                .color(Color32::from_rgb(56, 189, 248))
                                .size(14.0)
                                .strong(),
                        )
                        .fill(COLOR_CARD_BG)
                        .min_size(Vec2::new(160.0, 80.0)),
                    )
                    .clicked()
                {
                    self.show_toast("Sistem: Memasuki mode Sleep");
                }

                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("Restart")
                                .color(Color32::from_rgb(74, 222, 128))
                                .size(14.0)
                                .strong(),
                        )
                        .fill(COLOR_CARD_BG)
                        .min_size(Vec2::new(160.0, 80.0)),
                    )
                    .clicked()
                {
                    self.show_toast("Sistem: Permintaan Restart dikirim");
                }

                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("Shut Down")
                                .color(Color32::from_rgb(248, 113, 113))
                                .size(14.0)
                                .strong(),
                        )
                        .fill(COLOR_CARD_BG)
                        .min_size(Vec2::new(160.0, 80.0)),
                    )
                    .clicked()
                {
                    self.show_toast("Sistem: Permintaan Shut Down dikirim");
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("JARVIS OS")
            .with_inner_size([1100.0, 720.0])
            .with_min_inner_size([850.0, 600.0])
            .with_decorations(true)
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "JARVIS OS",
        native_options,
        Box::new(|_cc| Ok(Box::new(JarvisOsApp::default()))),
    )
}
