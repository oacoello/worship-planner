use crate::db::Database;
use crate::export::{
    decode_base64_to_color_image, decode_bytes_to_color_image, encode_image_file_to_base64,
    extract_ambient_tint, generate_printable_html, LOGO_DARK_BYTES, LOGO_LIGHT_BYTES,
};
use crate::i18n::I18n;
use crate::models::{
    AppSettings, Arrangement, ItemType, Language, SermonSeries, ServiceItem, ServicePlan, Song,
    ThemeMode, DEFAULT_KEYS,
};
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveTime};
use egui::{
    Align, Color32, Context, CornerRadius, FontId, Frame, Id, Layout, Margin, RichText,
    ScrollArea, Sense, Stroke, TextureHandle, TextureOptions, Ui, Vec2, ViewportCommand, Visuals,
};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveView {
    Services,
    Calendar,
    Series,
    Builder,
    SongBank,
    Stats,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFilter {
    All,
    Days90,
    ThisYear,
}

#[derive(Default, Clone)]
struct ServiceFormState {
    editing_id: Option<String>,
    title: String,
    series_id: String,
    sermon_title: String,
    week_number: String,
    date: String,
    time: String,
    creating_new_series: bool,
    new_series_title: String,
    new_series_artwork: Option<String>,
}

#[derive(Default, Clone)]
struct SeriesFormState {
    editing_id: Option<String>,
    title: String,
    description: String,
    artwork: Option<String>,
}

#[derive(Clone)]
struct AddItemFormState {
    item_type: ItemType,
    title: String,
    subtitle: String,
    arrangement_name: String,
    key: String,
    lead_vocalist: String,
    notes: String,
    duration_mins: u32,
    duration_secs: u32,
    selected_song_id: String,
    selected_arrangement_id: String,
    song_search_query: String,
}

impl Default for AddItemFormState {
    fn default() -> Self {
        Self {
            item_type: ItemType::Song,
            title: String::new(),
            subtitle: String::new(),
            arrangement_name: "Standard".to_string(),
            key: "C".to_string(),
            lead_vocalist: String::new(),
            notes: String::new(),
            duration_mins: 5,
            duration_secs: 0,
            selected_song_id: String::new(),
            selected_arrangement_id: String::new(),
            song_search_query: String::new(),
        }
    }
}

#[derive(Clone)]
struct NewSongFormState {
    title: String,
    artist: String,
    default_arr_name: String,
    key: String,
    bpm: u32,
    duration_mins: u32,
    duration_secs: u32,
}

impl Default for NewSongFormState {
    fn default() -> Self {
        Self {
            title: String::new(),
            artist: String::new(),
            default_arr_name: "Standard".to_string(),
            key: "G".to_string(),
            bpm: 72,
            duration_mins: 4,
            duration_secs: 30,
        }
    }
}

#[derive(Clone)]
struct NewArrFormState {
    target_song_id: String,
    target_song_title: String,
    name: String,
    key: String,
    bpm: u32,
    duration_mins: u32,
    duration_secs: u32,
}

impl Default for NewArrFormState {
    fn default() -> Self {
        Self {
            target_song_id: String::new(),
            target_song_title: String::new(),
            name: "Acoustic".to_string(),
            key: "G".to_string(),
            bpm: 72,
            duration_mins: 4,
            duration_secs: 0,
        }
    }
}

#[derive(Clone)]
enum ConfirmAction {
    DeleteService(String),
    DeleteSeries(String),
    DeleteSong(String),
}

pub struct GrancoPlannerApp {
    db: Database,
    settings: AppSettings,
    current_view: ActiveView,
    active_service_id: Option<String>,

    // Cached state from DB
    services: Vec<ServicePlan>,
    series_list: Vec<SermonSeries>,
    songs: Vec<Song>,
    all_items: Vec<ServiceItem>,
    builder_items: Vec<ServiceItem>,

    // Calendar state
    calendar_year: i32,
    calendar_month: u32,

    // Song bank & analytics state
    song_bank_search: String,
    expanded_songs: BTreeSet<String>,
    analytics_search: String,
    analytics_time_filter: TimeFilter,

    // Modals & forms
    show_service_modal: bool,
    service_form: ServiceFormState,

    show_series_modal: bool,
    series_form: SeriesFormState,

    show_add_item_modal: bool,
    add_item_form: AddItemFormState,

    show_new_song_modal: bool,
    new_song_form: NewSongFormState,

    show_new_arr_modal: bool,
    new_arr_form: NewArrFormState,

    confirm_modal: Option<(String, ConfirmAction)>,

    // Inline editing in Builder
    editing_notes_item_id: Option<String>,
    editing_notes_text: String,
    editing_vocalist_item_id: Option<String>,
    editing_vocalist_text: String,

    // Screenshot save path pending
    pending_screenshot_path: Option<PathBuf>,
    status_toast: Option<(String, f64)>,

    // Texture caches
    logo_dark_tex: Option<TextureHandle>,
    logo_light_tex: Option<TextureHandle>,
    artwork_textures: HashMap<String, TextureHandle>,
}

impl GrancoPlannerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let db = Database::open_default().unwrap_or_else(|_| {
            Database::open_in_memory().expect("Fallback in-memory DB failed")
        });
        let settings = db.get_settings();
        let now = Local::now();

        let mut app = Self {
            db,
            settings,
            current_view: ActiveView::Services,
            active_service_id: None,
            services: Vec::new(),
            series_list: Vec::new(),
            songs: Vec::new(),
            all_items: Vec::new(),
            builder_items: Vec::new(),
            calendar_year: now.year(),
            calendar_month: now.month(),
            song_bank_search: String::new(),
            expanded_songs: BTreeSet::new(),
            analytics_search: String::new(),
            analytics_time_filter: TimeFilter::All,
            show_service_modal: false,
            service_form: ServiceFormState::default(),
            show_series_modal: false,
            series_form: SeriesFormState::default(),
            show_add_item_modal: false,
            add_item_form: AddItemFormState::default(),
            show_new_song_modal: false,
            new_song_form: NewSongFormState::default(),
            show_new_arr_modal: false,
            new_arr_form: NewArrFormState::default(),
            confirm_modal: None,
            editing_notes_item_id: None,
            editing_notes_text: String::new(),
            editing_vocalist_item_id: None,
            editing_vocalist_text: String::new(),
            pending_screenshot_path: None,
            status_toast: None,
            logo_dark_tex: None,
            logo_light_tex: None,
            artwork_textures: HashMap::new(),
        };

        app.reload_data();
        app.apply_theme(&cc.egui_ctx);
        app
    }

    fn reload_data(&mut self) {
        self.services = self.db.get_services().unwrap_or_default();
        self.series_list = self.db.get_sermon_series().unwrap_or_default();
        self.songs = self.db.get_songs().unwrap_or_default();
        self.all_items = self.db.get_all_items().unwrap_or_default();
        if let Some(ref svc_id) = self.active_service_id {
            self.builder_items = self.db.get_items_for_service(svc_id).unwrap_or_default();
        }
    }

    fn is_dark(&self, ctx: &Context) -> bool {
        match self.settings.theme_mode {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
            ThemeMode::System => ctx.style().visuals.dark_mode,
        }
    }

    fn apply_theme(&self, ctx: &Context) {
        let dark = match self.settings.theme_mode {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
            ThemeMode::System => true,
        };

        let mut visuals = if dark {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        if dark {
            visuals.panel_fill = Color32::from_rgb(20, 20, 23);
            visuals.window_fill = Color32::from_rgb(28, 28, 32);
            visuals.faint_bg_color = Color32::from_rgb(32, 32, 37);
            visuals.extreme_bg_color = Color32::from_rgb(15, 15, 18);
            visuals.selection.bg_fill = Color32::from_rgb(0, 120, 212);
            visuals.selection.stroke = Stroke::new(1.0_f32, Color32::from_rgb(96, 165, 250));
            visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(28, 28, 32);
            visuals.widgets.inactive.bg_fill = Color32::from_rgb(38, 38, 44);
            visuals.widgets.hovered.bg_fill = Color32::from_rgb(50, 50, 58);
            visuals.widgets.active.bg_fill = Color32::from_rgb(0, 120, 212);
        } else {
            visuals.panel_fill = Color32::from_rgb(245, 247, 250);
            visuals.window_fill = Color32::from_rgb(255, 255, 255);
            visuals.faint_bg_color = Color32::from_rgb(238, 242, 246);
            visuals.extreme_bg_color = Color32::from_rgb(255, 255, 255);
            visuals.selection.bg_fill = Color32::from_rgb(0, 120, 212);
            visuals.selection.stroke = Stroke::new(1.0_f32, Color32::WHITE);
        }

        ctx.set_visuals(visuals);
    }

    fn ensure_logos_loaded(&mut self, ctx: &Context) {
        if self.logo_dark_tex.is_none() {
            if let Some(ci) = decode_bytes_to_color_image(LOGO_DARK_BYTES) {
                self.logo_dark_tex =
                    Some(ctx.load_texture("logo_dark", ci, TextureOptions::LINEAR));
            }
        }
        if self.logo_light_tex.is_none() {
            if let Some(ci) = decode_bytes_to_color_image(LOGO_LIGHT_BYTES) {
                self.logo_light_tex =
                    Some(ctx.load_texture("logo_light", ci, TextureOptions::LINEAR));
            }
        }
    }

    fn get_or_load_artwork_tex(
        &mut self,
        ctx: &Context,
        key: &str,
        b64: &str,
    ) -> Option<TextureHandle> {
        let cache_key = format!("{}_{}", key, b64.len());
        if let Some(tex) = self.artwork_textures.get(&cache_key) {
            return Some(tex.clone());
        }
        if let Some(ci) = decode_base64_to_color_image(b64) {
            let tex = ctx.load_texture(&cache_key, ci, TextureOptions::LINEAR);
            self.artwork_textures.insert(cache_key, tex.clone());
            return Some(tex);
        }
        None
    }

    fn open_new_service_modal(&mut self, date_override: Option<String>) {
        self.service_form = ServiceFormState {
            editing_id: None,
            title: "Sunday Service".to_string(),
            series_id: String::new(),
            sermon_title: String::new(),
            week_number: "Week 1".to_string(),
            date: date_override
                .unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string()),
            time: "10:00".to_string(),
            creating_new_series: false,
            new_series_title: String::new(),
            new_series_artwork: None,
        };
        self.show_service_modal = true;
    }

    fn open_edit_service_modal(&mut self, svc: &ServicePlan) {
        self.service_form = ServiceFormState {
            editing_id: Some(svc.id.clone()),
            title: svc.title.clone(),
            series_id: svc.series_id.clone(),
            sermon_title: svc.sermon_title.clone(),
            week_number: svc.week_number.clone(),
            date: svc.date.clone(),
            time: svc.time.clone(),
            creating_new_series: false,
            new_series_title: String::new(),
            new_series_artwork: None,
        };
        self.show_service_modal = true;
    }

    fn save_service_from_modal(&mut self) {
        if self.service_form.title.trim().is_empty() {
            return;
        }
        let mut chosen_series_id = self.service_form.series_id.clone();
        let mut chosen_series_title = String::new();
        let mut chosen_series_art = None;

        if self.service_form.creating_new_series
            && !self.service_form.new_series_title.trim().is_empty()
        {
            let new_series = SermonSeries::new(
                self.service_form.new_series_title.trim().to_string(),
                String::new(),
                self.service_form.new_series_artwork.clone(),
            );
            if let Ok(saved) = self.db.save_sermon_series(new_series) {
                chosen_series_id = saved.id;
                chosen_series_title = saved.title;
                chosen_series_art = saved.artwork;
            }
        } else if !chosen_series_id.is_empty() {
            if let Some(existing) = self
                .series_list
                .iter()
                .find(|s| s.id == chosen_series_id)
            {
                chosen_series_title = existing.title.clone();
                chosen_series_art = existing.artwork.clone();
            }
        }

        let plan = ServicePlan {
            id: self.service_form.editing_id.clone().unwrap_or_default(),
            title: self.service_form.title.trim().to_string(),
            series_id: chosen_series_id,
            series: chosen_series_title,
            sermon_title: self.service_form.sermon_title.trim().to_string(),
            week_number: self.service_form.week_number.trim().to_string(),
            sermon_art: chosen_series_art,
            date: self.service_form.date.trim().to_string(),
            time: self.service_form.time.trim().to_string(),
        };

        let _ = self.db.save_service(plan);
        self.show_service_modal = false;
        self.reload_data();
    }

    fn open_service_builder(&mut self, service_id: String) {
        self.active_service_id = Some(service_id);
        self.current_view = ActiveView::Builder;
        self.editing_notes_item_id = None;
        self.editing_vocalist_item_id = None;
        self.reload_data();
    }

    fn handle_dropped_files(&mut self, ctx: &Context) {
        let dropped = ctx.input(|i| i.raw.dropped_files.clone());
        if let Some(file) = dropped.first() {
            let b64 = if let Some(ref path) = file.path {
                encode_image_file_to_base64(path)
            } else if let Some(ref bytes) = file.bytes {
                crate::export::encode_image_bytes_to_base64(bytes)
            } else {
                None
            };
            if let Some(encoded) = b64 {
                if self.show_series_modal {
                    self.series_form.artwork = Some(encoded);
                } else if self.show_service_modal && self.service_form.creating_new_series {
                    self.service_form.new_series_artwork = Some(encoded);
                }
            }
        }
    }

    fn handle_screenshot_events(&mut self, ctx: &Context) {
        let mut captured_image = None;
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Screenshot { image, .. } = event {
                    captured_image = Some(image.clone());
                }
            }
        });

        if let (Some(img), Some(path)) = (captured_image, self.pending_screenshot_path.take()) {
            let w = img.width() as u32;
            let h = img.height() as u32;
            let raw: Vec<u8> = img
                .pixels
                .iter()
                .flat_map(|p| [p.r(), p.g(), p.b(), p.a()])
                .collect();
            if let Some(rgba_buf) = image::RgbaImage::from_raw(w, h, raw) {
                if rgba_buf.save(&path).is_ok() {
                    self.status_toast = Some((
                        format!("PNG saved: {}", path.display()),
                        ctx.input(|i| i.time) + 4.0,
                    ));
                }
            }
        }
    }

    // ==========================================
    // SIDEBAR
    // ==========================================
    fn render_sidebar(&mut self, ctx: &Context) {
        let is_dark = self.is_dark(ctx);
        let i18n = I18n::new(self.settings.language);

        let sidebar_bg = if is_dark {
            Color32::from_rgb(26, 26, 30)
        } else {
            Color32::from_rgb(255, 255, 255)
        };
        let stroke_col = if is_dark {
            Color32::from_rgb(45, 45, 52)
        } else {
            Color32::from_rgb(226, 232, 240)
        };

        egui::SidePanel::left("main_sidebar")
            .exact_width(200.0)
            .resizable(false)
            .frame(
                Frame::NONE
                    .fill(sidebar_bg)
                    .stroke(Stroke::new(1.0_f32, stroke_col))
                    .inner_margin(Margin::symmetric(12, 18)),
            )
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // App Header
                    ui.horizontal(|ui| {
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new("⛪")
                                .size(20.0)
                                .color(Color32::from_rgb(56, 189, 248)),
                        );
                        ui.label(
                            RichText::new(i18n.t("appTitle"))
                                .size(17.0)
                                .strong(),
                        );
                    });
                    ui.add_space(20.0);

                    let nav_items = [
                        (ActiveView::Services, "📅", i18n.t("services")),
                        (ActiveView::Calendar, "🗓", i18n.t("calendar")),
                        (ActiveView::Series, "🖼", i18n.t("series")),
                        (ActiveView::SongBank, "🎵", i18n.t("songBank")),
                        (ActiveView::Stats, "📊", i18n.t("stats")),
                    ];

                    let effective_selected = if self.current_view == ActiveView::Builder {
                        ActiveView::Services
                    } else {
                        self.current_view
                    };

                    for (view, icon, label) in nav_items {
                        let is_selected = effective_selected == view;
                        if Self::sidebar_tab_button(ui, icon, label, is_selected, is_dark) {
                            self.current_view = view;
                            if view != ActiveView::Builder {
                                self.active_service_id = None;
                            }
                        }
                        ui.add_space(4.0);
                    }

                    // Push Settings to bottom
                    let remaining = ui.available_height() - 52.0;
                    if remaining > 0.0 {
                        ui.add_space(remaining);
                    }
                    ui.separator();
                    ui.add_space(4.0);
                    let settings_selected = self.current_view == ActiveView::Settings;
                    if Self::sidebar_tab_button(
                        ui,
                        "⚙",
                        i18n.t("settings"),
                        settings_selected,
                        is_dark,
                    ) {
                        self.current_view = ActiveView::Settings;
                        self.active_service_id = None;
                    }
                });
            });
    }

    fn sidebar_tab_button(
        ui: &mut Ui,
        icon: &str,
        label: &str,
        selected: bool,
        is_dark: bool,
    ) -> bool {
        let fill = if selected {
            if is_dark {
                Color32::from_rgb(0, 102, 180)
            } else {
                Color32::from_rgb(224, 242, 254)
            }
        } else {
            Color32::TRANSPARENT
        };
        let text_color = if selected {
            if is_dark {
                Color32::WHITE
            } else {
                Color32::from_rgb(3, 105, 161)
            }
        } else if is_dark {
            Color32::from_rgb(212, 212, 216)
        } else {
            Color32::from_rgb(51, 65, 85)
        };

        let btn = egui::Button::new(
            RichText::new(format!("{}   {}", icon, label))
                .size(14.0)
                .color(text_color)
                .strong(),
        )
        .fill(fill)
        .corner_radius(CornerRadius::same(6))
        .min_size(Vec2::new(ui.available_width(), 36.0));

        ui.add(btn).clicked()
    }

    // ==========================================
    // VIEW 1: SERVICES LIST
    // ==========================================
    fn render_services_view(&mut self, ctx: &Context, ui: &mut Ui) {
        let is_dark = self.is_dark(ctx);
        let i18n = I18n::new(self.settings.language);

        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new(i18n.t("servicePlans")).size(28.0).strong());
                        ui.label(
                            RichText::new(i18n.t("servicePlansDesc"))
                                .size(14.0)
                                .color( Color32::GRAY),
                        );
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(24.0);
                        let new_btn = egui::Button::new(
                            RichText::new(format!("➕  {}", i18n.t("newService")))
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(Color32::from_rgb(0, 120, 212))
                        .corner_radius(CornerRadius::same(6))
                        .min_size(Vec2::new(140.0, 36.0));
                        if ui.add(new_btn).clicked() {
                            self.open_new_service_modal(None);
                        }
                    });
                });

                ui.add_space(24.0);

                if self.services.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(64.0);
                        ui.label(
                            RichText::new(i18n.t("noServicesFound"))
                                .size(15.0)
                                .color(Color32::GRAY),
                        );
                    });
                    return;
                }

                // Group services by series_id
                let mut grouped: BTreeMap<String, Vec<ServicePlan>> = BTreeMap::new();
                let mut standalone: Vec<ServicePlan> = Vec::new();

                for svc in &self.services {
                    if !svc.series_id.is_empty()
                        && self.series_list.iter().any(|s| s.id == svc.series_id)
                    {
                        grouped
                            .entry(svc.series_id.clone())
                            .or_default()
                            .push(svc.clone());
                    } else {
                        standalone.push(svc.clone());
                    }
                }

                let series_snapshot = self.series_list.clone();
                let mut open_id = None;
                let mut edit_svc = None;
                let mut delete_svc_id = None;

                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.vertical(|ui| {
                        ui.set_width((ui.available_width() - 24.0).max(320.0));

                        for series in &series_snapshot {
                            if let Some(svcs) = grouped.get(&series.id) {
                                if !svcs.is_empty() {
                                    ui.label(
                                        RichText::new(format!(
                                            "{}: {}",
                                            i18n.t("seriesPrefix"),
                                            series.title
                                        ))
                                        .size(20.0)
                                        .strong(),
                                    );
                                    ui.add_space(12.0);
                                    ui.horizontal_wrapped(|ui| {
                                        for svc in svcs {
                                            let (o, e, d) = self.render_service_card(
                                                ctx,
                                                ui,
                                                svc,
                                                Some(series),
                                                is_dark,
                                                &i18n,
                                            );
                                            if o.is_some() {
                                                open_id = o;
                                            }
                                            if e.is_some() {
                                                edit_svc = e;
                                            }
                                            if d.is_some() {
                                                delete_svc_id = d;
                                            }
                                        }
                                    });
                                    ui.add_space(28.0);
                                }
                            }
                        }

                        if !standalone.is_empty() {
                            ui.label(
                                RichText::new(i18n.t("standaloneServicesGroup"))
                                    .size(20.0)
                                    .strong(),
                            );
                            ui.add_space(12.0);
                            ui.horizontal_wrapped(|ui| {
                                for svc in &standalone {
                                    let (o, e, d) =
                                        self.render_service_card(ctx, ui, svc, None, is_dark, &i18n);
                                    if o.is_some() {
                                        open_id = o;
                                    }
                                    if e.is_some() {
                                        edit_svc = e;
                                    }
                                    if d.is_some() {
                                        delete_svc_id = d;
                                    }
                                }
                            });
                            ui.add_space(28.0);
                        }
                    });
                });

                if let Some(id) = open_id {
                    self.open_service_builder(id);
                }
                if let Some(svc) = edit_svc {
                    self.open_edit_service_modal(&svc);
                }
                if let Some(id) = delete_svc_id {
                    self.confirm_modal = Some((
                        i18n.t("confirmDeleteService").to_string(),
                        ConfirmAction::DeleteService(id),
                    ));
                }
            });
    }

    fn render_service_card(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        service: &ServicePlan,
        linked_series: Option<&SermonSeries>,
        is_dark: bool,
        i18n: &I18n,
    ) -> (Option<String>, Option<ServicePlan>, Option<String>) {
        let mut open_id = None;
        let mut edit_svc = None;
        let mut delete_id = None;

        let art_b64 = linked_series
            .and_then(|s| s.artwork.as_deref())
            .or(service.sermon_art.as_deref());
        let series_name = linked_series
            .map(|s| s.title.as_str())
            .filter(|t| !t.is_empty())
            .unwrap_or(&service.series);

        let mut subtitle_parts = Vec::new();
        if !series_name.is_empty() {
            subtitle_parts.push(format!("{}: {}", i18n.t("seriesPrefix"), series_name));
        }
        if !service.week_number.is_empty() {
            subtitle_parts.push(service.week_number.clone());
        }
        if !service.sermon_title.is_empty() {
            subtitle_parts.push(format!("\"{}\"", service.sermon_title));
        }
        let sermon_subtitle = subtitle_parts.join(" • ");

        let card_bg = if is_dark {
            Color32::from_rgb(30, 30, 35)
        } else {
            Color32::WHITE
        };
        let border_col = if is_dark {
            Color32::from_rgb(48, 48, 56)
        } else {
            Color32::from_rgb(226, 232, 240)
        };
        let banner_bg = if is_dark {
            Color32::from_rgb(38, 38, 45)
        } else {
            Color32::from_rgb(241, 245, 249)
        };

        let tex_opt = if let Some(b64) = art_b64 {
            self.get_or_load_artwork_tex(ctx, &service.id, b64)
        } else {
            None
        };

        Frame::NONE
            .fill(card_bg)
            .stroke(Stroke::new(1.0_f32, border_col))
            .corner_radius(CornerRadius::same(8))
            .show(ui, |ui| {
                ui.set_width(320.0);
                ui.vertical(|ui| {
                    // Banner (140px)
                    let (rect, resp) =
                        ui.allocate_exact_size(Vec2::new(320.0, 140.0), Sense::click());
                    ui.painter().rect_filled(rect, CornerRadius::ZERO, banner_bg);
                    if let Some(tex) = tex_opt {
                        ui.painter().image(
                            tex.id(),
                            rect,
                            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                            Color32::WHITE,
                        );
                    } else {
                        ui.painter().text(
                            rect.center() - Vec2::new(0.0, 10.0),
                            egui::Align2::CENTER_CENTER,
                            "📋",
                            FontId::proportional(28.0),
                            Color32::GRAY,
                        );
                        ui.painter().text(
                            rect.center() + Vec2::new(0.0, 18.0),
                            egui::Align2::CENTER_CENTER,
                            i18n.t("standaloneWorshipService"),
                            FontId::proportional(13.0),
                            Color32::GRAY,
                        );
                    }
                    if resp.clicked() {
                        open_id = Some(service.id.clone());
                    }

                    // Card Content
                    Frame::NONE
                        .inner_margin(Margin::symmetric(16, 12))
                        .show(ui, |ui| {
                            ui.set_width(288.0);
                            if ui
                                .add(
                                    egui::Label::new(
                                        RichText::new(&service.title).size(18.0).strong(),
                                    )
                                    .sense(Sense::click()),
                                )
                                .clicked()
                            {
                                open_id = Some(service.id.clone());
                            }

                            if !sermon_subtitle.is_empty() {
                                ui.add_space(4.0);
                                ui.label(
                                    RichText::new(sermon_subtitle)
                                        .size(13.0)
                                        .color(Color32::from_rgb(56, 189, 248))
                                        .strong(),
                                );
                            }

                            ui.add_space(6.0);
                            let formatted_dt =
                                i18n.format_card_date_time(&service.date, &service.time);
                            ui.label(
                                RichText::new(format!("📅  {}", formatted_dt))
                                    .size(13.0)
                                    .color(Color32::GRAY),
                            );
                        });

                    ui.separator();

                    // Footer Actions
                    Frame::NONE
                        .inner_margin(Margin::symmetric(12, 8))
                        .show(ui, |ui| {
                            ui.set_width(296.0);
                            ui.horizontal(|ui| {
                                if ui
                                    .button(format!("✏ {}", i18n.t("editPlan")))
                                    .clicked()
                                {
                                    edit_svc = Some(service.clone());
                                }
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui
                                        .add(egui::Button::new(
                                            RichText::new(format!("🗑 {}", i18n.t("delete")))
                                                .color(Color32::from_rgb(239, 68, 68)),
                                        ))
                                        .clicked()
                                    {
                                        delete_id = Some(service.id.clone());
                                    }
                                    if ui.button("▶ Open").clicked() {
                                        open_id = Some(service.id.clone());
                                    }
                                });
                            });
                        });
                });
            });

        (open_id, edit_svc, delete_id)
    }

    // ==========================================
    // VIEW 2: CALENDAR VIEW
    // ==========================================
    fn render_calendar_view(&mut self, ctx: &Context, ui: &mut Ui) {
        let is_dark = self.is_dark(ctx);
        let i18n = I18n::new(self.settings.language);

        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new(i18n.t("calendarTitle")).size(28.0).strong());
                        ui.label(
                            RichText::new(i18n.t("calendarDesc"))
                                .size(14.0)
                                .color(Color32::GRAY),
                        );
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(24.0);
                        let new_btn = egui::Button::new(
                            RichText::new(format!("➕  {}", i18n.t("newService")))
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(Color32::from_rgb(0, 120, 212))
                        .corner_radius(CornerRadius::same(6))
                        .min_size(Vec2::new(130.0, 34.0));
                        if ui.add(new_btn).clicked() {
                            self.open_new_service_modal(None);
                        }

                        if ui.button(i18n.t("today")).clicked() {
                            let now = Local::now();
                            self.calendar_year = now.year();
                            self.calendar_month = now.month();
                        }

                        if ui.button("▶").clicked() {
                            if self.calendar_month == 12 {
                                self.calendar_month = 1;
                                self.calendar_year += 1;
                            } else {
                                self.calendar_month += 1;
                            }
                        }

                        let month_title = format!(
                            "{} {}",
                            i18n.month_name(self.calendar_month),
                            self.calendar_year
                        );
                        ui.add_sized(
                            [160.0, 28.0],
                            egui::Label::new(RichText::new(month_title).size(18.0).strong()),
                        );

                        if ui.button("◀").clicked() {
                            if self.calendar_month == 1 {
                                self.calendar_month = 12;
                                self.calendar_year -= 1;
                            } else {
                                self.calendar_month -= 1;
                            }
                        }
                    });
                });

                ui.add_space(20.0);

                let first_of_month =
                    NaiveDate::from_ymd_opt(self.calendar_year, self.calendar_month, 1)
                        .unwrap_or_else(|| Local::now().date_naive());
                let start_offset = first_of_month.weekday().num_days_from_sunday() as i64;
                let grid_start = first_of_month - Duration::days(start_offset);

                let next_month_first = if self.calendar_month == 12 {
                    NaiveDate::from_ymd_opt(self.calendar_year + 1, 1, 1).unwrap()
                } else {
                    NaiveDate::from_ymd_opt(self.calendar_year, self.calendar_month + 1, 1).unwrap()
                };
                let last_of_month = next_month_first - Duration::days(1);
                let end_offset = 6 - (last_of_month.weekday().num_days_from_sunday() as i64);
                let grid_end = last_of_month + Duration::days(end_offset);

                let total_days = ((grid_end - grid_start).num_days() + 1) as usize;
                let num_weeks = (total_days / 7).max(5);

                let weekdays = [
                    i18n.t("sun"),
                    i18n.t("mon"),
                    i18n.t("tue"),
                    i18n.t("wed"),
                    i18n.t("thu"),
                    i18n.t("fri"),
                    i18n.t("sat"),
                ];

                let today_naive = Local::now().date_naive();
                let mut open_svc_id = None;
                let mut create_on_date = None;

                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    let avail_w = (ui.available_width() - 24.0).max(700.0);
                    let col_w = (avail_w / 7.0).floor() - 4.0;

                    let frame_bg = if is_dark {
                        Color32::from_rgb(26, 26, 30)
                    } else {
                        Color32::WHITE
                    };
                    let stroke_col = if is_dark {
                        Color32::from_rgb(48, 48, 56)
                    } else {
                        Color32::from_rgb(226, 232, 240)
                    };

                    Frame::NONE
                        .fill(frame_bg)
                        .stroke(Stroke::new(1.0_f32, stroke_col))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::same(8))
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                // Weekday header row
                                ui.horizontal(|ui| {
                                    for wd in weekdays {
                                        ui.allocate_ui(Vec2::new(col_w, 28.0), |ui| {
                                            ui.centered_and_justified(|ui| {
                                                ui.label(
                                                    RichText::new(wd.to_uppercase())
                                                        .size(12.0)
                                                        .strong()
                                                        .color(Color32::GRAY),
                                                );
                                            });
                                        });
                                    }
                                });
                                ui.separator();

                                // Weeks rows
                                for week in 0..num_weeks {
                                    ui.horizontal(|ui| {
                                        for dow in 0..7 {
                                            let day_idx = (week * 7 + dow) as i64;
                                            let day_date = grid_start + Duration::days(day_idx);
                                            let is_curr_month =
                                                day_date.month() == self.calendar_month;
                                            let is_today = day_date == today_naive;
                                            let day_str = day_date.format("%Y-%m-%d").to_string();

                                            let day_services: Vec<ServicePlan> = self
                                                .services
                                                .iter()
                                                .filter(|s| s.date == day_str)
                                                .cloned()
                                                .collect();

                                            let cell_bg = if !is_curr_month {
                                                if is_dark {
                                                    Color32::from_rgb(20, 20, 24)
                                                } else {
                                                    Color32::from_rgb(241, 245, 249)
                                                }
                                            } else if is_dark {
                                                Color32::from_rgb(30, 30, 36)
                                            } else {
                                                Color32::WHITE
                                            };

                                            Frame::NONE
                                                .fill(cell_bg)
                                                .stroke(Stroke::new(1.0_f32, stroke_col))
                                                .corner_radius(CornerRadius::same(4))
                                                .inner_margin(Margin::same(6))
                                                .show(ui, |ui| {
                                                    ui.set_min_size(Vec2::new(
                                                        col_w - 14.0,
                                                        115.0,
                                                    ));
                                                    ui.set_max_width(col_w - 14.0);
                                                    ui.vertical(|ui| {
                                                        // Day number + Add button
                                                        ui.horizontal(|ui| {
                                                            let num_txt =
                                                                format!("{}", day_date.day());
                                                            if is_today {
                                                                Frame::NONE
                                                                    .fill(Color32::from_rgb(
                                                                        0, 120, 212,
                                                                    ))
                                                                    .corner_radius(
                                                                        CornerRadius::same(12),
                                                                    )
                                                                    .inner_margin(
                                                                        Margin::symmetric(6, 2),
                                                                    )
                                                                    .show(ui, |ui| {
                                                                        ui.label(
                                                                            RichText::new(num_txt)
                                                                                .size(12.0)
                                                                                .color(
                                                                                    Color32::WHITE,
                                                                                )
                                                                                .strong(),
                                                                        );
                                                                    });
                                                            } else {
                                                                let col = if is_curr_month {
                                                                    if is_dark {
                                                                        Color32::LIGHT_GRAY
                                                                    } else {
                                                                        Color32::DARK_GRAY
                                                                    }
                                                                } else {
                                                                    Color32::GRAY
                                                                };
                                                                ui.label(
                                                                    RichText::new(num_txt)
                                                                        .size(12.0)
                                                                        .strong()
                                                                        .color(col),
                                                                );
                                                            }

                                                            ui.with_layout(
                                                                Layout::right_to_left(
                                                                    Align::Center,
                                                                ),
                                                                |ui| {
                                                                    if ui
                                                                        .small_button("＋")
                                                                        .on_hover_text(i18n.t(
                                                                            "addServiceForDay",
                                                                        ))
                                                                        .clicked()
                                                                    {
                                                                        create_on_date =
                                                                            Some(day_str.clone());
                                                                    }
                                                                },
                                                            );
                                                        });

                                                        ui.add_space(4.0);

                                                        // Services on this day
                                                        for svc in day_services {
                                                            let series_name = if !svc
                                                                .series_id
                                                                .is_empty()
                                                            {
                                                                self.series_list
                                                                    .iter()
                                                                    .find(|s| s.id == svc.series_id)
                                                                    .map(|s| s.title.clone())
                                                                    .unwrap_or_else(|| {
                                                                        svc.series.clone()
                                                                    })
                                                            } else {
                                                                svc.series.clone()
                                                            };

                                                            let pill_bg = if is_dark {
                                                                Color32::from_rgb(42, 44, 54)
                                                            } else {
                                                                Color32::from_rgb(239, 246, 255)
                                                            };

                                                            let resp = Frame::NONE
                                                                .fill(pill_bg)
                                                                .stroke(Stroke::new(
                                                                    1.0_f32,
                                                                    Color32::from_rgb(0, 120, 212),
                                                                ))
                                                                .corner_radius(CornerRadius::same(
                                                                    4,
                                                                ))
                                                                .inner_margin(Margin::symmetric(
                                                                    6, 4,
                                                                ))
                                                                .show(ui, |ui| {
                                                                    ui.set_width(
                                                                        ui.available_width(),
                                                                    );
                                                                    ui.vertical(|ui| {
                                                                        ui.label(
                                                                            RichText::new(
                                                                                &svc.title,
                                                                            )
                                                                            .size(11.5)
                                                                            .strong(),
                                                                        );
                                                                        if !series_name.is_empty() {
                                                                            ui.label(
                                                                                RichText::new(
                                                                                    series_name,
                                                                                )
                                                                                .size(10.5)
                                                                                .color(
                                                                                    Color32::from_rgb(
                                                                                        56, 189, 248,
                                                                                    ),
                                                                                ),
                                                                            );
                                                                        }
                                                                        ui.label(
                                                                            RichText::new(format!(
                                                                                "🕒 {}",
                                                                                svc.time
                                                                            ))
                                                                            .size(10.0)
                                                                            .color(Color32::GRAY),
                                                                        );
                                                                    });
                                                                })
                                                                .response
                                                                .interact(Sense::click());

                                                            if resp.clicked() {
                                                                open_svc_id = Some(svc.id.clone());
                                                            }
                                                            ui.add_space(3.0);
                                                        }
                                                    });
                                                });
                                        }
                                    });
                                }
                            });
                        });
                });

                if let Some(id) = open_svc_id {
                    self.open_service_builder(id);
                }
                if let Some(date_str) = create_on_date {
                    self.open_new_service_modal(Some(date_str));
                }
            });
    }

    // ==========================================
    // VIEW 3: SERMON SERIES
    // ==========================================
    fn render_series_view(&mut self, ctx: &Context, ui: &mut Ui) {
        let is_dark = self.is_dark(ctx);
        let i18n = I18n::new(self.settings.language);

        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(i18n.t("sermonSeriesTitle"))
                                .size(28.0)
                                .strong(),
                        );
                        ui.label(
                            RichText::new(i18n.t("sermonSeriesDesc"))
                                .size(14.0)
                                .color(Color32::GRAY),
                        );
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(24.0);
                        let btn = egui::Button::new(
                            RichText::new(format!("➕  {}", i18n.t("newSeries")))
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(Color32::from_rgb(0, 120, 212))
                        .corner_radius(CornerRadius::same(6))
                        .min_size(Vec2::new(130.0, 36.0));
                        if ui.add(btn).clicked() {
                            self.series_form = SeriesFormState::default();
                            self.show_series_modal = true;
                        }
                    });
                });

                ui.add_space(24.0);

                if self.series_list.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(64.0);
                        ui.label(
                            RichText::new(i18n.t("noSeriesFound"))
                                .size(15.0)
                                .color(Color32::GRAY),
                        );
                    });
                    return;
                }

                let list_snapshot = self.series_list.clone();
                let mut edit_series = None;
                let mut delete_series_id = None;

                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.horizontal_wrapped(|ui| {
                        for series in &list_snapshot {
                            let card_bg = if is_dark {
                                Color32::from_rgb(30, 30, 35)
                            } else {
                                Color32::WHITE
                            };
                            let border_col = if is_dark {
                                Color32::from_rgb(48, 48, 56)
                            } else {
                                Color32::from_rgb(226, 232, 240)
                            };
                            let banner_bg = if is_dark {
                                Color32::from_rgb(38, 38, 45)
                            } else {
                                Color32::from_rgb(241, 245, 249)
                            };

                            let tex_opt = if let Some(ref b64) = series.artwork {
                                self.get_or_load_artwork_tex(ctx, &series.id, b64)
                            } else {
                                None
                            };

                            Frame::NONE
                                .fill(card_bg)
                                .stroke(Stroke::new(1.0_f32, border_col))
                                .corner_radius(CornerRadius::same(8))
                                .show(ui, |ui| {
                                    ui.set_width(290.0);
                                    ui.vertical(|ui| {
                                        let (rect, _) = ui.allocate_exact_size(
                                            Vec2::new(290.0, 150.0),
                                            Sense::hover(),
                                        );
                                        ui.painter()
                                            .rect_filled(rect, CornerRadius::ZERO, banner_bg);
                                        if let Some(tex) = tex_opt {
                                            ui.painter().image(
                                                tex.id(),
                                                rect,
                                                egui::Rect::from_min_max(
                                                    egui::pos2(0.0, 0.0),
                                                    egui::pos2(1.0, 1.0),
                                                ),
                                                Color32::WHITE,
                                            );
                                        } else {
                                            ui.painter().text(
                                                rect.center() - Vec2::new(0.0, 8.0),
                                                egui::Align2::CENTER_CENTER,
                                                "🖼",
                                                FontId::proportional(28.0),
                                                Color32::GRAY,
                                            );
                                            ui.painter().text(
                                                rect.center() + Vec2::new(0.0, 18.0),
                                                egui::Align2::CENTER_CENTER,
                                                i18n.t("noArtworkUploaded"),
                                                FontId::proportional(12.5),
                                                Color32::GRAY,
                                            );
                                        }

                                        Frame::NONE
                                            .inner_margin(Margin::symmetric(14, 12))
                                            .show(ui, |ui| {
                                                ui.set_width(262.0);
                                                ui.label(
                                                    RichText::new(&series.title)
                                                        .size(17.0)
                                                        .strong(),
                                                );
                                                ui.add_space(4.0);
                                                ui.label(
                                                    RichText::new(
                                                        if series.description.is_empty() {
                                                            "—"
                                                        } else {
                                                            &series.description
                                                        },
                                                    )
                                                    .size(13.0)
                                                    .color(Color32::GRAY),
                                                );
                                            });

                                        ui.separator();

                                        Frame::NONE
                                            .inner_margin(Margin::symmetric(12, 8))
                                            .show(ui, |ui| {
                                                ui.set_width(266.0);
                                                ui.horizontal(|ui| {
                                                    if ui
                                                        .button(format!("✏ {}", i18n.t("editPlan")))
                                                        .clicked()
                                                    {
                                                        edit_series = Some(series.clone());
                                                    }
                                                    ui.with_layout(
                                                        Layout::right_to_left(Align::Center),
                                                        |ui| {
                                                            if ui
                                                                .add(egui::Button::new(
                                                                    RichText::new(format!(
                                                                        "🗑 {}",
                                                                        i18n.t("delete")
                                                                    ))
                                                                    .color(Color32::from_rgb(
                                                                        239, 68, 68,
                                                                    )),
                                                                ))
                                                                .clicked()
                                                            {
                                                                delete_series_id =
                                                                    Some(series.id.clone());
                                                            }
                                                        },
                                                    );
                                                });
                                            });
                                    });
                                });
                        }
                    });
                });

                if let Some(s) = edit_series {
                    self.series_form = SeriesFormState {
                        editing_id: Some(s.id),
                        title: s.title,
                        description: s.description,
                        artwork: s.artwork,
                    };
                    self.show_series_modal = true;
                }
                if let Some(id) = delete_series_id {
                    self.confirm_modal = Some((
                        i18n.t("confirmDeleteSeries").to_string(),
                        ConfirmAction::DeleteSeries(id),
                    ));
                }
            });
    }

    // ==========================================
    // VIEW 4: SERVICE BUILDER / RUN SHEET
    // ==========================================
    fn render_builder_view(&mut self, ctx: &Context, ui: &mut Ui) {
        let is_dark = self.is_dark(ctx);
        let i18n = I18n::new(self.settings.language);
        self.ensure_logos_loaded(ctx);

        let Some(ref active_id) = self.active_service_id.clone() else {
            self.current_view = ActiveView::Services;
            return;
        };
        let Some(service) = self.services.iter().find(|s| &s.id == active_id).cloned() else {
            self.current_view = ActiveView::Services;
            return;
        };

        let linked_series = if !service.series_id.is_empty() {
            self.series_list
                .iter()
                .find(|s| s.id == service.series_id)
                .cloned()
        } else {
            None
        };
        let art_b64 = linked_series
            .as_ref()
            .and_then(|s| s.artwork.clone())
            .or_else(|| service.sermon_art.clone());

        let ambient_bg = extract_ambient_tint(art_b64.as_deref(), is_dark);

        Frame::NONE
            .fill(ambient_bg)
            .inner_margin(Margin::symmetric(24, 20))
            .show(ui, |ui| {
                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        // Top Navigation & Export Bar
                        ui.horizontal(|ui| {
                            if ui
                                .button(
                                    RichText::new(format!("⬅  {}", i18n.t("backToServices")))
                                        .size(13.5)
                                        .strong(),
                                )
                                .clicked()
                            {
                                self.current_view = ActiveView::Services;
                                self.active_service_id = None;
                            }

                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                let png_btn = egui::Button::new(
                                    RichText::new(format!("📷  {}", i18n.t("savePngScreenshot")))
                                        .color(Color32::WHITE)
                                        .strong(),
                                )
                                .fill(Color32::from_rgb(0, 120, 212))
                                .corner_radius(CornerRadius::same(6));

                                if ui.add(png_btn).clicked() {
                                    let default_name = format!(
                                        "{}-{}.png",
                                        service.title.replace(' ', "-"),
                                        service.date
                                    );
                                    if let Some(path) = rfd::FileDialog::new()
                                        .set_file_name(&default_name)
                                        .add_filter("PNG Image", &["png"])
                                        .save_file()
                                    {
                                        self.pending_screenshot_path = Some(path);
                                        ctx.send_viewport_cmd(ViewportCommand::Screenshot(
                                            egui::UserData::default(),
                                        ));
                                    }
                                }

                                if ui
                                    .button(format!("🖨  {}", i18n.t("printRunSheet")))
                                    .clicked()
                                {
                                    let html = generate_printable_html(
                                        &service,
                                        &self.builder_items,
                                        &i18n,
                                    );
                                    let tmp_path = std::env::temp_dir().join(format!(
                                        "granco-runsheet-{}.html",
                                        service.id
                                    ));
                                    if std::fs::write(&tmp_path, html).is_ok() {
                                        let _ = open::that(&tmp_path);
                                    }
                                }

                                if ui
                                    .button(format!("✏  {}", i18n.t("editPlanDetails")))
                                    .clicked()
                                {
                                    self.open_edit_service_modal(&service);
                                }
                            });
                        });

                        ui.add_space(12.0);

                        // Action Toolbar for adding items
                        let toolbar_bg = if is_dark {
                            Color32::from_rgb(28, 28, 33)
                        } else {
                            Color32::WHITE
                        };
                        let toolbar_stroke = if is_dark {
                            Color32::from_rgb(48, 48, 56)
                        } else {
                            Color32::from_rgb(226, 232, 240)
                        };

                        Frame::NONE
                            .fill(toolbar_bg)
                            .stroke(Stroke::new(1.0_f32, toolbar_stroke))
                            .corner_radius(CornerRadius::same(8))
                            .inner_margin(Margin::symmetric(14, 10))
                            .show(ui, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    let buttons = [
                                        (ItemType::Song, "🎵", i18n.t("addSong")),
                                        (ItemType::Conference, "🎤", i18n.t("addConference")),
                                        (ItemType::Video, "▶", i18n.t("addVideo")),
                                        (ItemType::Loop, "↻", i18n.t("addLoop")),
                                        (ItemType::Announcements, "📢", i18n.t("addAnnouncements")),
                                        (ItemType::Countdown, "⏱", i18n.t("addCountdown")),
                                    ];
                                    for (itype, icon, label) in buttons {
                                        if ui
                                            .button(
                                                RichText::new(format!("{}  {}", icon, label))
                                                    .size(13.0)
                                                    .strong(),
                                            )
                                            .clicked()
                                        {
                                            self.add_item_form = AddItemFormState {
                                                item_type: itype,
                                                duration_mins: itype.default_duration_mins(),
                                                duration_secs: 0,
                                                ..Default::default()
                                            };
                                            self.show_add_item_modal = true;
                                        }
                                    }
                                });
                            });

                        ui.add_space(16.0);

                        // Run Sheet Container
                        let sheet_bg = if is_dark {
                            Color32::from_rgb(24, 24, 24)
                        } else {
                            Color32::WHITE
                        };
                        let sheet_border = if is_dark {
                            Color32::from_rgb(45, 45, 45)
                        } else {
                            Color32::from_rgb(226, 232, 240)
                        };
                        let banner_bg = if is_dark {
                            Color32::from_rgb(20, 20, 20)
                        } else {
                            Color32::from_rgb(248, 250, 252)
                        };

                        let start_naive = NaiveTime::parse_from_str(&service.time, "%H:%M")
                            .unwrap_or_else(|_| NaiveTime::from_hms_opt(10, 0, 0).unwrap());
                        let total_secs: u32 = self.builder_items.iter().map(|i| i.duration).sum();
                        let end_naive = start_naive + Duration::seconds(total_secs as i64);
                        let total_formatted = if total_secs >= 3600 {
                            format!("{}h {}m", total_secs / 3600, (total_secs % 3600) / 60)
                        } else {
                            format!("{} min", total_secs / 60)
                        };

                        let art_tex = if let Some(ref b64) = art_b64 {
                            self.get_or_load_artwork_tex(ctx, &service.id, b64)
                        } else {
                            None
                        };

                        Frame::NONE
                            .fill(sheet_bg)
                            .stroke(Stroke::new(1.0_f32, sheet_border))
                            .corner_radius(CornerRadius::same(8))
                            .show(ui, |ui| {
                                ui.vertical(|ui| {
                                    // Top Banner
                                    Frame::NONE
                                        .fill(banner_bg)
                                        .inner_margin(Margin::symmetric(24, 20))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                if let Some(tex) = art_tex {
                                                    let (rect, _) = ui.allocate_exact_size(
                                                        Vec2::new(120.0, 75.0),
                                                        Sense::hover(),
                                                    );
                                                    ui.painter().image(
                                                        tex.id(),
                                                        rect,
                                                        egui::Rect::from_min_max(
                                                            egui::pos2(0.0, 0.0),
                                                            egui::pos2(1.0, 1.0),
                                                        ),
                                                        Color32::WHITE,
                                                    );
                                                    ui.add_space(16.0);
                                                }

                                                ui.vertical(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            RichText::new(&service.title)
                                                                .size(24.0)
                                                                .strong(),
                                                        );
                                                        if ui
                                                            .small_button("✏")
                                                            .on_hover_text(
                                                                i18n.t("editPlanDetails"),
                                                            )
                                                            .clicked()
                                                        {
                                                            self.open_edit_service_modal(&service);
                                                        }
                                                    });

                                                    let series_title = linked_series
                                                        .as_ref()
                                                        .map(|s| s.title.as_str())
                                                        .filter(|t| !t.is_empty())
                                                        .unwrap_or(&service.series);

                                                    let mut sub_parts = Vec::new();
                                                    if !series_title.is_empty() {
                                                        sub_parts.push(format!(
                                                            "{}: {}",
                                                            i18n.t("seriesPrefix"),
                                                            series_title
                                                        ));
                                                    }
                                                    if !service.week_number.is_empty() {
                                                        sub_parts.push(service.week_number.clone());
                                                    }
                                                    if !service.sermon_title.is_empty() {
                                                        sub_parts.push(format!(
                                                            "\"{}\"",
                                                            service.sermon_title
                                                        ));
                                                    }
                                                    if !sub_parts.is_empty() {
                                                        ui.label(
                                                            RichText::new(sub_parts.join(" • "))
                                                                .size(15.0)
                                                                .color(Color32::from_rgb(
                                                                    56, 189, 248,
                                                                ))
                                                                .strong(),
                                                        );
                                                    }

                                                    ui.add_space(4.0);
                                                    ui.horizontal_wrapped(|ui| {
                                                        ui.label(
                                                            RichText::new(format!(
                                                                "📅 {}",
                                                                i18n.format_long_date(
                                                                    &service.date
                                                                )
                                                            ))
                                                            .size(13.0)
                                                            .color(Color32::GRAY),
                                                        );
                                                        ui.add_space(12.0);
                                                        ui.label(
                                                            RichText::new(format!(
                                                                "🕒 {}: {}",
                                                                i18n.t("start"),
                                                                I18n::format_naive_time_12h(
                                                                    start_naive
                                                                )
                                                            ))
                                                            .size(13.0),
                                                        );
                                                        ui.add_space(12.0);
                                                        ui.label(
                                                            RichText::new(format!(
                                                                "⏱ {}: {} ({}: {})",
                                                                i18n.t("total"),
                                                                total_formatted,
                                                                i18n.t("estEnd"),
                                                                I18n::format_naive_time_12h(
                                                                    end_naive
                                                                )
                                                            ))
                                                            .size(13.0),
                                                        );
                                                    });
                                                });

                                                // Church Logo on right
                                                ui.with_layout(
                                                    Layout::right_to_left(Align::Center),
                                                    |ui| {
                                                        let logo_tex = if is_dark {
                                                            self.logo_light_tex.as_ref()
                                                        } else {
                                                            self.logo_dark_tex.as_ref()
                                                        };
                                                        if let Some(tex) = logo_tex {
                                                            let (rect, _) = ui.allocate_exact_size(
                                                                Vec2::new(76.0, 76.0),
                                                                Sense::hover(),
                                                            );
                                                            ui.painter().image(
                                                                tex.id(),
                                                                rect,
                                                                egui::Rect::from_min_max(
                                                                    egui::pos2(0.0, 0.0),
                                                                    egui::pos2(1.0, 1.0),
                                                                ),
                                                                Color32::WHITE,
                                                            );
                                                        }
                                                    },
                                                );
                                            });
                                        });

                                    // Setlist Table Header
                                    let th_bg = if is_dark {
                                        Color32::from_rgb(32, 32, 32)
                                    } else {
                                        Color32::from_rgb(241, 245, 249)
                                    };
                                    Frame::NONE
                                        .fill(th_bg)
                                        .inner_margin(Margin::symmetric(16, 10))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.add_sized([52.0, 18.0], egui::Label::new(""));
                                                ui.add_sized(
                                                    [64.0, 18.0],
                                                    egui::Label::new(
                                                        RichText::new(
                                                            i18n.t("length").to_uppercase(),
                                                        )
                                                        .size(11.0)
                                                        .strong()
                                                        .color(Color32::GRAY),
                                                    ),
                                                );
                                                ui.add_sized(
                                                    [80.0, 18.0],
                                                    egui::Label::new(
                                                        RichText::new(
                                                            i18n.t("start").to_uppercase(),
                                                        )
                                                        .size(11.0)
                                                        .strong()
                                                        .color(Color32::GRAY),
                                                    ),
                                                );
                                                ui.add_sized([30.0, 18.0], egui::Label::new(""));
                                                ui.add_sized(
                                                    [260.0, 18.0],
                                                    egui::Label::new(
                                                        RichText::new(
                                                            i18n.t("itemArrangement")
                                                                .to_uppercase(),
                                                        )
                                                        .size(11.0)
                                                        .strong()
                                                        .color(Color32::GRAY),
                                                    ),
                                                );
                                                ui.add_sized(
                                                    [250.0, 18.0],
                                                    egui::Label::new(
                                                        RichText::new(
                                                            i18n.t("notesCues").to_uppercase(),
                                                        )
                                                        .size(11.0)
                                                        .strong()
                                                        .color(Color32::GRAY),
                                                    ),
                                                );
                                                ui.add_sized(
                                                    [160.0, 18.0],
                                                    egui::Label::new(
                                                        RichText::new(
                                                            i18n.t("leadVocalist").to_uppercase(),
                                                        )
                                                        .size(11.0)
                                                        .strong()
                                                        .color(Color32::GRAY),
                                                    ),
                                                );
                                            });
                                        });

                                    if self.builder_items.is_empty() {
                                        ui.vertical_centered(|ui| {
                                            ui.add_space(48.0);
                                            ui.label(
                                                RichText::new(i18n.t("emptySheetPrompt"))
                                                    .size(14.0)
                                                    .color(Color32::GRAY),
                                            );
                                            ui.add_space(48.0);
                                        });
                                    } else {
                                        let mut running_time = start_naive;
                                        let items_snapshot = self.builder_items.clone();
                                        let count = items_snapshot.len();
                                        let mut swap_indices: Option<(usize, usize)> = None;
                                        let mut delete_item_id: Option<String> = None;
                                        let mut save_notes_for: Option<(String, String)> = None;
                                        let mut save_vocalist_for: Option<(String, String)> = None;

                                        for (idx, item) in items_snapshot.iter().enumerate() {
                                            let item_start = running_time;
                                            running_time += Duration::seconds(item.duration as i64);

                                            let (row_bg, accent_col) =
                                                Self::item_row_colors(item.item_type, is_dark);

                                            Frame::NONE
                                                .fill(row_bg)
                                                .inner_margin(Margin::symmetric(14, 12))
                                                .show(ui, |ui| {
                                                    let rect = ui.max_rect();
                                                    // Draw 4px left accent bar
                                                    ui.painter().rect_filled(
                                                        egui::Rect::from_min_max(
                                                            rect.left_top(),
                                                            egui::pos2(
                                                                rect.left() + 4.0,
                                                                rect.bottom(),
                                                            ),
                                                        ),
                                                        CornerRadius::ZERO,
                                                        accent_col,
                                                    );

                                                    ui.vertical(|ui| {
                                                        ui.horizontal(|ui| {
                                                            // 1. Reorder Up/Down buttons
                                                            ui.allocate_ui(
                                                                Vec2::new(52.0, 24.0),
                                                                |ui| {
                                                                    ui.horizontal(|ui| {
                                                                        if ui
                                                                            .add_enabled(
                                                                                idx > 0,
                                                                                egui::Button::new(
                                                                                    "⬆",
                                                                                )
                                                                                .small(),
                                                                            )
                                                                            .clicked()
                                                                        {
                                                                            swap_indices = Some((
                                                                                idx,
                                                                                idx - 1,
                                                                            ));
                                                                        }
                                                                        if ui
                                                                            .add_enabled(
                                                                                idx + 1 < count,
                                                                                egui::Button::new(
                                                                                    "⬇",
                                                                                )
                                                                                .small(),
                                                                            )
                                                                            .clicked()
                                                                        {
                                                                            swap_indices = Some((
                                                                                idx,
                                                                                idx + 1,
                                                                            ));
                                                                        }
                                                                    });
                                                                },
                                                            );

                                                            // 2. Duration
                                                            ui.add_sized(
                                                                [64.0, 24.0],
                                                                egui::Label::new(
                                                                    RichText::new(
                                                                        item.format_duration(),
                                                                    )
                                                                    .monospace()
                                                                    .strong(),
                                                                ),
                                                            );

                                                            // 3. Start Time
                                                            ui.add_sized(
                                                                [80.0, 24.0],
                                                                egui::Label::new(
                                                                    RichText::new(
                                                                        I18n::format_naive_time_12h(
                                                                            item_start,
                                                                        ),
                                                                    )
                                                                    .monospace()
                                                                    .strong(),
                                                                ),
                                                            );

                                                            // 4. Type Icon
                                                            ui.add_sized(
                                                                [30.0, 24.0],
                                                                egui::Label::new(
                                                                    RichText::new(
                                                                        item.item_type
                                                                            .icon_symbol(),
                                                                    )
                                                                    .size(16.0)
                                                                    .color(accent_col),
                                                                ),
                                                            );

                                                            // 5. Title & Key / Arrangement Badges
                                                            ui.allocate_ui(
                                                                Vec2::new(260.0, 36.0),
                                                                |ui| {
                                                                    ui.vertical(|ui| {
                                                                        ui.label(
                                                                            RichText::new(
                                                                                &item.title,
                                                                            )
                                                                            .size(15.0)
                                                                            .strong(),
                                                                        );
                                                                        if !item.key.is_empty()
                                                                            || !item
                                                                                .arrangement_name
                                                                                .is_empty()
                                                                        {
                                                                            ui.horizontal(|ui| {
                                                                                if !item
                                                                                    .key
                                                                                    .is_empty()
                                                                                {
                                                                                    Frame::NONE
                                                                                        .fill(Color32::from_rgba_unmultiplied(234, 179, 8, 45))
                                                                                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(234, 179, 8)))
                                                                                        .corner_radius(CornerRadius::same(4))
                                                                                        .inner_margin(Margin::symmetric(6, 1))
                                                                                        .show(ui, |ui| {
                                                                                            ui.label(
                                                                                                RichText::new(format!("{}: {}", i18n.t("key"), item.key))
                                                                                                    .size(11.0)
                                                                                                    .strong(),
                                                                                            );
                                                                                        });
                                                                                }
                                                                                if !item
                                                                                    .arrangement_name
                                                                                    .is_empty()
                                                                                {
                                                                                    Frame::NONE
                                                                                        .fill(Color32::from_rgba_unmultiplied(59, 130, 246, 45))
                                                                                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(59, 130, 246)))
                                                                                        .corner_radius(CornerRadius::same(4))
                                                                                        .inner_margin(Margin::symmetric(6, 1))
                                                                                        .show(ui, |ui| {
                                                                                            ui.label(
                                                                                                RichText::new(&item.arrangement_name)
                                                                                                    .size(11.0),
                                                                                            );
                                                                                        });
                                                                                }
                                                                            });
                                                                        }
                                                                    });
                                                                },
                                                            );

                                                            // 6. Notes & Cues
                                                            ui.allocate_ui(
                                                                Vec2::new(250.0, 28.0),
                                                                |ui| {
                                                                    ui.horizontal(|ui| {
                                                                        ui.label(
                                                                            RichText::new(
                                                                                if item
                                                                                    .notes
                                                                                    .is_empty()
                                                                                {
                                                                                    "—"
                                                                                } else {
                                                                                    &item.notes
                                                                                },
                                                                            )
                                                                            .italics()
                                                                            .size(13.0),
                                                                        );
                                                                        if ui
                                                                            .small_button("✏")
                                                                            .on_hover_text(i18n.t(
                                                                                "saveNotes",
                                                                            ))
                                                                            .clicked()
                                                                        {
                                                                            self.editing_notes_item_id =
                                                                                Some(
                                                                                    item.id.clone(),
                                                                                );
                                                                            self.editing_notes_text =
                                                                                item.notes.clone();
                                                                            self.editing_vocalist_item_id =
                                                                                None;
                                                                        }
                                                                    });
                                                                },
                                                            );

                                                            // 7. Lead Vocalist / Speaker
                                                            ui.allocate_ui(
                                                                Vec2::new(160.0, 28.0),
                                                                |ui| {
                                                                    ui.horizontal(|ui| {
                                                                        ui.label(
                                                                            RichText::new(
                                                                                if item
                                                                                    .lead_vocalist
                                                                                    .is_empty()
                                                                                {
                                                                                    "—"
                                                                                } else {
                                                                                    &item.lead_vocalist
                                                                                },
                                                                            )
                                                                            .strong()
                                                                            .size(13.0),
                                                                        );
                                                                        if ui
                                                                            .small_button("✏")
                                                                            .on_hover_text(i18n.t(
                                                                                "mainVocalistLead",
                                                                            ))
                                                                            .clicked()
                                                                        {
                                                                            self.editing_vocalist_item_id =
                                                                                Some(
                                                                                    item.id.clone(),
                                                                                );
                                                                            self.editing_vocalist_text =
                                                                                item.lead_vocalist
                                                                                    .clone();
                                                                            self.editing_notes_item_id =
                                                                                None;
                                                                        }
                                                                    });
                                                                },
                                                            );

                                                            // 8. Delete button
                                                            ui.with_layout(
                                                                Layout::right_to_left(
                                                                    Align::Center,
                                                                ),
                                                                |ui| {
                                                                    if ui
                                                                        .add(
                                                                            egui::Button::new(
                                                                                RichText::new("🗑")
                                                                                    .color(
                                                                                        Color32::from_rgb(
                                                                                            239,
                                                                                            68,
                                                                                            68,
                                                                                        ),
                                                                                    ),
                                                                            )
                                                                            .small(),
                                                                        )
                                                                        .clicked()
                                                                    {
                                                                        delete_item_id =
                                                                            Some(item.id.clone());
                                                                    }
                                                                },
                                                            );
                                                        });

                                                        // Inline Notes Editor
                                                        if self.editing_notes_item_id.as_deref()
                                                            == Some(&item.id)
                                                        {
                                                            ui.add_space(6.0);
                                                            ui.horizontal(|ui| {
                                                                ui.add_space(60.0);
                                                                ui.label(
                                                                    RichText::new(i18n.t(
                                                                        "extraNotesTechnicalCues",
                                                                    ))
                                                                    .size(12.0)
                                                                    .strong(),
                                                                );
                                                                ui.text_edit_singleline(
                                                                    &mut self.editing_notes_text,
                                                                );
                                                                if ui
                                                                    .button(i18n.t("saveNotes"))
                                                                    .clicked()
                                                                {
                                                                    save_notes_for = Some((
                                                                        item.id.clone(),
                                                                        self.editing_notes_text
                                                                            .clone(),
                                                                    ));
                                                                }
                                                                if ui
                                                                    .button(i18n.t("cancel"))
                                                                    .clicked()
                                                                {
                                                                    self.editing_notes_item_id =
                                                                        None;
                                                                }
                                                            });
                                                        }

                                                        // Inline Vocalist Editor
                                                        if self
                                                            .editing_vocalist_item_id
                                                            .as_deref()
                                                            == Some(&item.id)
                                                        {
                                                            ui.add_space(6.0);
                                                            ui.horizontal(|ui| {
                                                                ui.add_space(60.0);
                                                                ui.label(
                                                                    RichText::new(
                                                                        i18n.t("mainVocalistLead"),
                                                                    )
                                                                    .size(12.0)
                                                                    .strong(),
                                                                );
                                                                ui.text_edit_singleline(
                                                                    &mut self
                                                                        .editing_vocalist_text,
                                                                );
                                                                if ui
                                                                    .button(i18n.t("saveChanges"))
                                                                    .clicked()
                                                                {
                                                                    save_vocalist_for = Some((
                                                                        item.id.clone(),
                                                                        self.editing_vocalist_text
                                                                            .clone(),
                                                                    ));
                                                                }
                                                                if ui
                                                                    .button(i18n.t("cancel"))
                                                                    .clicked()
                                                                {
                                                                    self.editing_vocalist_item_id =
                                                                        None;
                                                                }
                                                            });
                                                        }
                                                    });
                                                });
                                        }

                                        if let Some((a, b)) = swap_indices {
                                            self.builder_items.swap(a, b);
                                            for (idx, itm) in
                                                self.builder_items.iter_mut().enumerate()
                                            {
                                                itm.order = idx as i32;
                                            }
                                            let _ = self.db.save_items(&self.builder_items);
                                            self.reload_data();
                                        }
                                        if let Some(del_id) = delete_item_id {
                                            let _ = self.db.delete_item(&del_id);
                                            self.reload_data();
                                        }
                                        if let Some((id, new_notes)) = save_notes_for {
                                            if let Some(mut itm) = self
                                                .builder_items
                                                .iter()
                                                .find(|i| i.id == id)
                                                .cloned()
                                            {
                                                itm.notes = new_notes;
                                                let _ = self.db.save_item(itm);
                                                self.editing_notes_item_id = None;
                                                self.reload_data();
                                            }
                                        }
                                        if let Some((id, new_voc)) = save_vocalist_for {
                                            if let Some(mut itm) = self
                                                .builder_items
                                                .iter()
                                                .find(|i| i.id == id)
                                                .cloned()
                                            {
                                                itm.lead_vocalist = new_voc;
                                                let _ = self.db.save_item(itm);
                                                self.editing_vocalist_item_id = None;
                                                self.reload_data();
                                            }
                                        }
                                    }
                                });
                            });
                    });
            });
    }

    fn item_row_colors(item_type: ItemType, is_dark: bool) -> (Color32, Color32) {
        if is_dark {
            match item_type {
                ItemType::Video => (
                    Color32::from_rgb(10, 35, 24),
                    Color32::from_rgb(34, 197, 94),
                ),
                ItemType::Loop => (
                    Color32::from_rgb(10, 29, 51),
                    Color32::from_rgb(56, 189, 248),
                ),
                ItemType::Announcements => (
                    Color32::from_rgb(43, 26, 6),
                    Color32::from_rgb(249, 115, 22),
                ),
                ItemType::Conference => (
                    Color32::from_rgb(31, 19, 43),
                    Color32::from_rgb(168, 85, 247),
                ),
                ItemType::Song => (
                    Color32::from_rgb(28, 28, 28),
                    Color32::from_rgb(115, 115, 115),
                ),
                ItemType::Countdown => (
                    Color32::from_rgb(26, 26, 26),
                    Color32::from_rgb(100, 100, 100),
                ),
            }
        } else {
            match item_type {
                ItemType::Video => (
                    Color32::from_rgb(240, 253, 244),
                    Color32::from_rgb(22, 163, 74),
                ),
                ItemType::Loop => (
                    Color32::from_rgb(239, 246, 255),
                    Color32::from_rgb(37, 99, 235),
                ),
                ItemType::Announcements => (
                    Color32::from_rgb(255, 247, 237),
                    Color32::from_rgb(234, 88, 12),
                ),
                ItemType::Conference => (
                    Color32::from_rgb(250, 245, 255),
                    Color32::from_rgb(147, 51, 234),
                ),
                ItemType::Song => (Color32::WHITE, Color32::from_rgb(148, 163, 184)),
                ItemType::Countdown => (
                    Color32::from_rgb(248, 250, 252),
                    Color32::from_rgb(203, 213, 225),
                ),
            }
        }
    }

    // ==========================================
    // VIEW 5: SONG BANK
    // ==========================================
    fn render_song_bank_view(&mut self, ctx: &Context, ui: &mut Ui) {
        let is_dark = self.is_dark(ctx);
        let i18n = I18n::new(self.settings.language);

        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new(i18n.t("songBankTitle")).size(28.0).strong());
                        ui.label(
                            RichText::new(i18n.t("songBankDesc"))
                                .size(14.0)
                                .color(Color32::GRAY),
                        );
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(24.0);
                        let btn = egui::Button::new(
                            RichText::new(format!("➕  {}", i18n.t("newSong")))
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(Color32::from_rgb(0, 120, 212))
                        .corner_radius(CornerRadius::same(6))
                        .min_size(Vec2::new(130.0, 36.0));
                        if ui.add(btn).clicked() {
                            self.new_song_form = NewSongFormState::default();
                            self.show_new_song_modal = true;
                        }
                    });
                });

                ui.add_space(18.0);
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.label("🔍");
                    ui.add_sized(
                        [340.0, 28.0],
                        egui::TextEdit::singleline(&mut self.song_bank_search)
                            .hint_text(i18n.t("searchSongs")),
                    );
                });

                ui.add_space(16.0);

                let q = self.song_bank_search.trim().to_lowercase();
                let filtered: Vec<Song> = self
                    .songs
                    .iter()
                    .filter(|s| {
                        q.is_empty()
                            || s.title.to_lowercase().contains(&q)
                            || s.artist.to_lowercase().contains(&q)
                    })
                    .cloned()
                    .collect();

                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.vertical(|ui| {
                        ui.set_width((ui.available_width() - 24.0).max(650.0));

                        // Table Header
                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [280.0, 20.0],
                                egui::Label::new(
                                    RichText::new("TITLE")
                                        .size(11.5)
                                        .strong()
                                        .color(Color32::GRAY),
                                ),
                            );
                            ui.add_sized(
                                [80.0, 20.0],
                                egui::Label::new(
                                    RichText::new("BPM")
                                        .size(11.5)
                                        .strong()
                                        .color(Color32::GRAY),
                                ),
                            );
                            ui.add_sized(
                                [120.0, 20.0],
                                egui::Label::new(
                                    RichText::new("KEYS")
                                        .size(11.5)
                                        .strong()
                                        .color(Color32::GRAY),
                                ),
                            );
                            ui.add_sized(
                                [130.0, 20.0],
                                egui::Label::new(
                                    RichText::new("ARRANGEMENTS")
                                        .size(11.5)
                                        .strong()
                                        .color(Color32::GRAY),
                                ),
                            );
                        });
                        ui.separator();

                        if filtered.is_empty() {
                            ui.vertical_centered(|ui| {
                                ui.add_space(48.0);
                                ui.label(
                                    RichText::new(i18n.t("noSongsFound"))
                                        .size(14.0)
                                        .color(Color32::GRAY),
                                );
                            });
                            return;
                        }

                        let mut add_arr_for: Option<Song> = None;
                        let mut del_song_id: Option<String> = None;
                        let mut del_arr: Option<(Song, String)> = None;

                        for song in filtered {
                            let is_expanded = self.expanded_songs.contains(&song.id);
                            let row_bg = if is_dark {
                                Color32::from_rgb(28, 28, 33)
                            } else {
                                Color32::WHITE
                            };
                            let border_col = if is_dark {
                                Color32::from_rgb(45, 45, 52)
                            } else {
                                Color32::from_rgb(226, 232, 240)
                            };

                            Frame::NONE
                                .fill(row_bg)
                                .stroke(Stroke::new(1.0_f32, border_col))
                                .corner_radius(CornerRadius::same(6))
                                .inner_margin(Margin::symmetric(14, 10))
                                .show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            let toggle_icon = if is_expanded { "▾" } else { "▸" };
                                            if ui.small_button(toggle_icon).clicked() {
                                                if is_expanded {
                                                    self.expanded_songs.remove(&song.id);
                                                } else {
                                                    self.expanded_songs.insert(song.id.clone());
                                                }
                                            }

                                            ui.allocate_ui(Vec2::new(250.0, 32.0), |ui| {
                                                ui.vertical(|ui| {
                                                    ui.label(
                                                        RichText::new(&song.title)
                                                            .size(14.5)
                                                            .strong(),
                                                    );
                                                    if !song.artist.is_empty() {
                                                        ui.label(
                                                            RichText::new(&song.artist)
                                                                .size(11.5)
                                                                .color(Color32::GRAY),
                                                        );
                                                    }
                                                });
                                            });

                                            ui.add_sized(
                                                [80.0, 24.0],
                                                egui::Label::new(song.bpm_summary()),
                                            );
                                            ui.add_sized(
                                                [120.0, 24.0],
                                                egui::Label::new(song.key_summary()),
                                            );
                                            ui.add_sized(
                                                [130.0, 24.0],
                                                egui::Label::new(format!(
                                                    "{} arr.",
                                                    song.arrangements.len()
                                                )),
                                            );

                                            ui.with_layout(
                                                Layout::right_to_left(Align::Center),
                                                |ui| {
                                                    if ui
                                                        .add(
                                                            egui::Button::new(
                                                                RichText::new("🗑").color(
                                                                    Color32::from_rgb(239, 68, 68),
                                                                ),
                                                            )
                                                            .small(),
                                                        )
                                                        .on_hover_text(i18n.t("delete"))
                                                        .clicked()
                                                    {
                                                        del_song_id = Some(song.id.clone());
                                                    }
                                                    if ui
                                                        .small_button("➕ Arr.")
                                                        .on_hover_text(i18n.t("addArrangement"))
                                                        .clicked()
                                                    {
                                                        add_arr_for = Some(song.clone());
                                                    }
                                                },
                                            );
                                        });

                                        if is_expanded {
                                            ui.add_space(8.0);
                                            let sub_bg = if is_dark {
                                                Color32::from_rgb(20, 20, 24)
                                            } else {
                                                Color32::from_rgb(248, 250, 252)
                                            };
                                            Frame::NONE
                                                .fill(sub_bg)
                                                .corner_radius(CornerRadius::same(4))
                                                .inner_margin(Margin::symmetric(12, 8))
                                                .show(ui, |ui| {
                                                    let can_delete_arr =
                                                        song.arrangements.len() > 1;
                                                    for arr in &song.arrangements {
                                                        ui.horizontal(|ui| {
                                                            ui.add_sized(
                                                                [220.0, 22.0],
                                                                egui::Label::new(
                                                                    RichText::new(&arr.name)
                                                                        .strong(),
                                                                ),
                                                            );
                                                            ui.add_sized(
                                                                [80.0, 22.0],
                                                                egui::Label::new(if arr.bpm > 0 {
                                                                    format!("{} BPM", arr.bpm)
                                                                } else {
                                                                    "-".to_string()
                                                                }),
                                                            );
                                                            ui.add_sized(
                                                                [90.0, 22.0],
                                                                egui::Label::new(
                                                                    RichText::new(format!(
                                                                        "{}: {}",
                                                                        i18n.t("key"),
                                                                        arr.key
                                                                    ))
                                                                    .color(Color32::from_rgb(
                                                                        56, 189, 248,
                                                                    ))
                                                                    .strong(),
                                                                ),
                                                            );
                                                            ui.add_sized(
                                                                [80.0, 22.0],
                                                                egui::Label::new(
                                                                    RichText::new(
                                                                        arr.format_duration(),
                                                                    )
                                                                    .monospace(),
                                                                ),
                                                            );
                                                            if can_delete_arr {
                                                                ui.with_layout(
                                                                    Layout::right_to_left(
                                                                        Align::Center,
                                                                    ),
                                                                    |ui| {
                                                                        if ui
                                                                            .small_button("🗑")
                                                                            .clicked()
                                                                        {
                                                                            del_arr = Some((
                                                                                song.clone(),
                                                                                arr.id.clone(),
                                                                            ));
                                                                        }
                                                                    },
                                                                );
                                                            }
                                                        });
                                                    }
                                                });
                                        }
                                    });
                                });
                            ui.add_space(6.0);
                        }

                        if let Some(song) = add_arr_for {
                            self.new_arr_form = NewArrFormState {
                                target_song_id: song.id,
                                target_song_title: song.title,
                                ..Default::default()
                            };
                            self.show_new_arr_modal = true;
                        }
                        if let Some(id) = del_song_id {
                            self.confirm_modal = Some((
                                i18n.t("confirmDeleteSong").to_string(),
                                ConfirmAction::DeleteSong(id),
                            ));
                        }
                        if let Some((mut song, arr_id)) = del_arr {
                            song.arrangements.retain(|a| a.id != arr_id);
                            let _ = self.db.save_song(song);
                            self.reload_data();
                        }
                    });
                });
            });
    }

    // ==========================================
    // VIEW 6: SONG ANALYTICS
    // ==========================================
    fn render_analytics_view(&mut self, ctx: &Context, ui: &mut Ui) {
        let is_dark = self.is_dark(ctx);
        let i18n = I18n::new(self.settings.language);

        // Compute filtered services based on time_filter
        let today = Local::now().date_naive();
        let active_service_dates: HashMap<String, String> = self
            .services
            .iter()
            .filter(|svc| {
                if let Ok(d) = NaiveDate::parse_from_str(&svc.date, "%Y-%m-%d") {
                    match self.analytics_time_filter {
                        TimeFilter::All => true,
                        TimeFilter::Days90 => d >= today - Duration::days(90),
                        TimeFilter::ThisYear => d.year() == today.year(),
                    }
                } else {
                    true
                }
            })
            .map(|svc| (svc.id.clone(), svc.date.clone()))
            .collect();

        #[derive(Clone)]
        struct SongUsageRecord {
            title: String,
            artist: String,
            count: usize,
            keys: BTreeSet<String>,
            last_date: Option<String>,
        }

        let mut usage_map: BTreeMap<String, SongUsageRecord> = BTreeMap::new();
        let mut key_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut total_scheduled_songs = 0usize;

        for s in &self.songs {
            let norm = s.title.trim().to_lowercase();
            usage_map.insert(
                norm,
                SongUsageRecord {
                    title: s.title.clone(),
                    artist: s.artist.clone(),
                    count: 0,
                    keys: BTreeSet::new(),
                    last_date: None,
                },
            );
        }

        for item in &self.all_items {
            if item.item_type != ItemType::Song {
                continue;
            }
            let Some(svc_date) = active_service_dates.get(&item.service_id) else {
                continue;
            };
            total_scheduled_songs += 1;
            if !item.key.is_empty() {
                *key_counts.entry(item.key.clone()).or_insert(0) += 1;
            }

            let norm = item.title.trim().to_lowercase();
            let entry = usage_map.entry(norm).or_insert_with(|| SongUsageRecord {
                title: item.title.clone(),
                artist: item.subtitle.clone(),
                count: 0,
                keys: BTreeSet::new(),
                last_date: None,
            });
            entry.count += 1;
            if !item.key.is_empty() {
                entry.keys.insert(item.key.clone());
            }
            if entry
                .last_date
                .as_ref()
                .map(|ld| svc_date > ld)
                .unwrap_or(true)
            {
                entry.last_date = Some(svc_date.clone());
            }
        }

        let mut records: Vec<SongUsageRecord> = usage_map.into_values().collect();
        records.sort_by(|a, b| {
            b.count
                .cmp(&a.count)
                .then_with(|| a.title.to_lowercase().cmp(&b.title.to_lowercase()))
        });

        let max_count = records.iter().map(|r| r.count).max().unwrap_or(1).max(1);
        let unique_used = records.iter().filter(|r| r.count > 0).count();
        let top_song_str = records
            .first()
            .filter(|r| r.count > 0)
            .map(|r| format!("{} ({})", r.title, r.count))
            .unwrap_or_else(|| "—".to_string());
        let top_key_str = key_counts
            .into_iter()
            .max_by_key(|(_, cnt)| *cnt)
            .map(|(k, _)| k)
            .unwrap_or_else(|| "—".to_string());

        let q = self.analytics_search.trim().to_lowercase();
        let filtered_records: Vec<SongUsageRecord> = records
            .into_iter()
            .filter(|r| {
                q.is_empty()
                    || r.title.to_lowercase().contains(&q)
                    || r.artist.to_lowercase().contains(&q)
            })
            .collect();

        let most_played: Vec<&SongUsageRecord> =
            filtered_records.iter().filter(|r| r.count > 0).collect();
        let least_played: Vec<&SongUsageRecord> =
            filtered_records.iter().filter(|r| r.count == 0).collect();

        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new(i18n.t("analyticsTitle")).size(28.0).strong());
                        ui.label(
                            RichText::new(i18n.t("analyticsDesc"))
                                .size(14.0)
                                .color(Color32::GRAY),
                        );
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(24.0);
                        egui::ComboBox::from_id_salt("analytics_time_filter")
                            .selected_text(match self.analytics_time_filter {
                                TimeFilter::All => i18n.t("allTime"),
                                TimeFilter::Days90 => i18n.t("last90Days"),
                                TimeFilter::ThisYear => i18n.t("thisYear"),
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.analytics_time_filter,
                                    TimeFilter::All,
                                    i18n.t("allTime"),
                                );
                                ui.selectable_value(
                                    &mut self.analytics_time_filter,
                                    TimeFilter::Days90,
                                    i18n.t("last90Days"),
                                );
                                ui.selectable_value(
                                    &mut self.analytics_time_filter,
                                    TimeFilter::ThisYear,
                                    i18n.t("thisYear"),
                                );
                            });
                    });
                });

                ui.add_space(20.0);

                // KPI Cards
                let card_bg = if is_dark {
                    Color32::from_rgb(28, 28, 33)
                } else {
                    Color32::WHITE
                };
                let border_col = if is_dark {
                    Color32::from_rgb(48, 48, 56)
                } else {
                    Color32::from_rgb(226, 232, 240)
                };

                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.horizontal_wrapped(|ui| {
                        let kpis = [
                            (
                                "📅",
                                i18n.t("totalServicesScheduled"),
                                format!("{}", active_service_dates.len()),
                            ),
                            (
                                "🎵",
                                i18n.t("totalSongInstances"),
                                format!("{}", total_scheduled_songs),
                            ),
                            (
                                "📊",
                                i18n.t("uniqueSongsUsed"),
                                format!("{}", unique_used),
                            ),
                            ("🏆", i18n.t("mostFrequentSong"), top_song_str.clone()),
                            ("🎹", i18n.t("mostFrequentKey"), top_key_str.clone()),
                        ];

                        for (icon, label, val) in kpis {
                            Frame::NONE
                                .fill(card_bg)
                                .stroke(Stroke::new(1.0_f32, border_col))
                                .corner_radius(CornerRadius::same(8))
                                .inner_margin(Margin::symmetric(16, 14))
                                .show(ui, |ui| {
                                    ui.set_min_size(Vec2::new(175.0, 68.0));
                                    ui.vertical(|ui| {
                                        ui.label(
                                            RichText::new(format!("{}  {}", icon, label))
                                                .size(12.5)
                                                .color(Color32::GRAY)
                                                .strong(),
                                        );
                                        ui.add_space(6.0);
                                        ui.label(
                                            RichText::new(val)
                                                .size(21.0)
                                                .strong()
                                                .color(Color32::from_rgb(56, 189, 248)),
                                        );
                                    });
                                });
                        }
                    });
                });

                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.label("🔍");
                    ui.add_sized(
                        [340.0, 28.0],
                        egui::TextEdit::singleline(&mut self.analytics_search)
                            .hint_text(i18n.t("searchSongs")),
                    );
                });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.vertical(|ui| {
                        ui.set_width((ui.available_width() - 24.0).max(650.0));

                        // Most Played Table
                        ui.label(
                            RichText::new(format!(
                                "{} ({})",
                                i18n.t("mostPlayedTitle"),
                                most_played.len()
                            ))
                            .size(18.0)
                            .strong(),
                        );
                        ui.add_space(8.0);

                        Frame::NONE
                            .fill(card_bg)
                            .stroke(Stroke::new(1.0_f32, border_col))
                            .corner_radius(CornerRadius::same(8))
                            .inner_margin(Margin::symmetric(16, 12))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.add_sized(
                                        [40.0, 18.0],
                                        egui::Label::new(
                                            RichText::new("#").size(11.0).strong().color(Color32::GRAY),
                                        ),
                                    );
                                    ui.add_sized(
                                        [220.0, 18.0],
                                        egui::Label::new(
                                            RichText::new(i18n.t("songTitleCol").to_uppercase())
                                                .size(11.0)
                                                .strong()
                                                .color(Color32::GRAY),
                                        ),
                                    );
                                    ui.add_sized(
                                        [160.0, 18.0],
                                        egui::Label::new(
                                            RichText::new(i18n.t("artistCol").to_uppercase())
                                                .size(11.0)
                                                .strong()
                                                .color(Color32::GRAY),
                                        ),
                                    );
                                    ui.add_sized(
                                        [90.0, 18.0],
                                        egui::Label::new(
                                            RichText::new(i18n.t("usageCountCol").to_uppercase())
                                                .size(11.0)
                                                .strong()
                                                .color(Color32::GRAY),
                                        ),
                                    );
                                    ui.add_sized(
                                        [160.0, 18.0],
                                        egui::Label::new(
                                            RichText::new(i18n.t("frequencyCol").to_uppercase())
                                                .size(11.0)
                                                .strong()
                                                .color(Color32::GRAY),
                                        ),
                                    );
                                    ui.add_sized(
                                        [110.0, 18.0],
                                        egui::Label::new(
                                            RichText::new(i18n.t("lastDateCol").to_uppercase())
                                                .size(11.0)
                                                .strong()
                                                .color(Color32::GRAY),
                                        ),
                                    );
                                });
                                ui.separator();

                                if most_played.is_empty() {
                                    ui.vertical_centered(|ui| {
                                        ui.add_space(24.0);
                                        ui.label(
                                            RichText::new(i18n.t("noSongsRecorded"))
                                                .color(Color32::GRAY),
                                        );
                                        ui.add_space(24.0);
                                    });
                                } else {
                                    for (idx, rec) in most_played.iter().enumerate() {
                                        let ratio = (rec.count as f32) / (max_count as f32);
                                        let rank_col = match idx {
                                            0 => Color32::from_rgb(234, 179, 8),
                                            1 => Color32::from_rgb(148, 163, 184),
                                            2 => Color32::from_rgb(217, 119, 6),
                                            _ => Color32::GRAY,
                                        };
                                        ui.horizontal(|ui| {
                                            ui.add_sized(
                                                [40.0, 26.0],
                                                egui::Label::new(
                                                    RichText::new(format!("#{}", idx + 1))
                                                        .color(rank_col)
                                                        .strong(),
                                                ),
                                            );
                                            let keys_joined: Vec<String> =
                                                rec.keys.iter().cloned().collect();
                                            let title_with_keys = if keys_joined.is_empty() {
                                                rec.title.clone()
                                            } else {
                                                format!(
                                                    "{} [{}]",
                                                    rec.title,
                                                    keys_joined.join(", ")
                                                )
                                            };
                                            ui.add_sized(
                                                [220.0, 26.0],
                                                egui::Label::new(
                                                    RichText::new(title_with_keys).strong(),
                                                ),
                                            );
                                            ui.add_sized(
                                                [160.0, 26.0],
                                                egui::Label::new(if rec.artist.is_empty() {
                                                    "—"
                                                } else {
                                                    &rec.artist
                                                }),
                                            );
                                            ui.add_sized(
                                                [90.0, 26.0],
                                                egui::Label::new(
                                                    RichText::new(format!("{}", rec.count))
                                                        .color(Color32::from_rgb(56, 189, 248))
                                                        .strong(),
                                                ),
                                            );
                                            ui.allocate_ui(Vec2::new(160.0, 22.0), |ui| {
                                                ui.add(
                                                    egui::ProgressBar::new(ratio)
                                                        .desired_width(145.0),
                                                );
                                            });
                                            ui.add_sized(
                                                [110.0, 26.0],
                                                egui::Label::new(
                                                    rec.last_date.as_deref().unwrap_or("—"),
                                                ),
                                            );
                                        });
                                    }
                                }
                            });

                        // Least Played / Inactive Songs Table
                        if !least_played.is_empty() {
                            ui.add_space(24.0);
                            ui.label(
                                RichText::new(format!(
                                    "⏳ {} ({})",
                                    i18n.t("leastPlayedTitle"),
                                    least_played.len()
                                ))
                                .size(18.0)
                                .strong(),
                            );
                            ui.add_space(8.0);

                            Frame::NONE
                                .fill(card_bg)
                                .stroke(Stroke::new(1.0_f32, border_col))
                                .corner_radius(CornerRadius::same(8))
                                .inner_margin(Margin::symmetric(16, 12))
                                .show(ui, |ui| {
                                    for rec in least_played {
                                        ui.horizontal(|ui| {
                                            ui.add_sized(
                                                [260.0, 24.0],
                                                egui::Label::new(
                                                    RichText::new(&rec.title).strong(),
                                                ),
                                            );
                                            ui.add_sized(
                                                [220.0, 24.0],
                                                egui::Label::new(if rec.artist.is_empty() {
                                                    "—"
                                                } else {
                                                    &rec.artist
                                                }),
                                            );
                                            ui.label(
                                                RichText::new(i18n.t("neverPlayedLabel"))
                                                    .color(Color32::from_rgb(239, 68, 68))
                                                    .size(12.0)
                                                    .strong(),
                                            );
                                        });
                                    }
                                });
                        }
                    });
                });
            });
    }

    // ==========================================
    // VIEW 7: SETTINGS
    // ==========================================
    fn render_settings_view(&mut self, ctx: &Context, ui: &mut Ui) {
        let is_dark = self.is_dark(ctx);
        let i18n = I18n::new(self.settings.language);

        let card_bg = if is_dark {
            Color32::from_rgb(28, 28, 33)
        } else {
            Color32::WHITE
        };
        let border_col = if is_dark {
            Color32::from_rgb(48, 48, 56)
        } else {
            Color32::from_rgb(226, 232, 240)
        };

        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.vertical(|ui| {
                        ui.set_max_width(820.0);
                        ui.label(RichText::new(i18n.t("settingsTitle")).size(28.0).strong());
                        ui.label(
                            RichText::new(i18n.t("settingsSubtitle"))
                                .size(14.0)
                                .color(Color32::GRAY),
                        );

                        ui.add_space(24.0);

                        // Appearance Card
                        Frame::NONE
                            .fill(card_bg)
                            .stroke(Stroke::new(1.0_f32, border_col))
                            .corner_radius(CornerRadius::same(8))
                            .inner_margin(Margin::symmetric(20, 18))
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.label(
                                    RichText::new(format!("🎨  {}", i18n.t("appearance")))
                                        .size(18.0)
                                        .strong(),
                                );
                                ui.label(
                                    RichText::new(i18n.t("themeDescription"))
                                        .size(13.0)
                                        .color(Color32::GRAY),
                                );
                                ui.add_space(12.0);

                                ui.horizontal_wrapped(|ui| {
                                    let themes = [
                                        (ThemeMode::Dark, "🌙", i18n.t("darkTheme")),
                                        (ThemeMode::Light, "☀", i18n.t("lightTheme")),
                                        (ThemeMode::System, "🖥", i18n.t("systemTheme")),
                                    ];
                                    for (mode, icon, label) in themes {
                                        let active = self.settings.theme_mode == mode;
                                        let btn = egui::Button::new(
                                            RichText::new(format!("{}  {}", icon, label))
                                                .size(14.0)
                                                .strong(),
                                        )
                                        .selected(active)
                                        .min_size(Vec2::new(200.0, 44.0));
                                        if ui.add(btn).clicked() {
                                            self.settings.theme_mode = mode;
                                            let _ = self.db.save_settings(&self.settings);
                                            self.apply_theme(ctx);
                                        }
                                    }
                                });
                            });

                        ui.add_space(20.0);

                        // Language Card
                        Frame::NONE
                            .fill(card_bg)
                            .stroke(Stroke::new(1.0_f32, border_col))
                            .corner_radius(CornerRadius::same(8))
                            .inner_margin(Margin::symmetric(20, 18))
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.label(
                                    RichText::new(format!("🌐  {}", i18n.t("language")))
                                        .size(18.0)
                                        .strong(),
                                );
                                ui.label(
                                    RichText::new(i18n.t("languageDescription"))
                                        .size(13.0)
                                        .color(Color32::GRAY),
                                );
                                ui.add_space(12.0);

                                ui.horizontal_wrapped(|ui| {
                                    let langs = [
                                        (Language::Es, "🇪🇸  Español (Latinoamérica)"),
                                        (Language::En, "🇺🇸  English (United States)"),
                                    ];
                                    for (lang, label) in langs {
                                        let active = self.settings.language == lang;
                                        let btn = egui::Button::new(
                                            RichText::new(label).size(14.0).strong(),
                                        )
                                        .selected(active)
                                        .min_size(Vec2::new(240.0, 44.0));
                                        if ui.add(btn).clicked() {
                                            self.settings.language = lang;
                                            let _ = self.db.save_settings(&self.settings);
                                        }
                                    }
                                });
                            });

                        ui.add_space(40.0);
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new(format!(
                                    "© {} Granco Planner • Todos los derechos reservados, Oscar Coello",
                                    Local::now().year()
                                ))
                                .size(12.0)
                                .color(Color32::GRAY),
                            );
                        });
                    });
                });
            });
    }

    // ==========================================
    // MODALS
    // ==========================================
    fn render_modals(&mut self, ctx: &Context) {
        let i18n = I18n::new(self.settings.language);

        // 1. Create / Edit Service Modal
        if self.show_service_modal {
            let mut open = self.show_service_modal;
            let title = if self.service_form.editing_id.is_some() {
                i18n.t("editServicePlan")
            } else {
                i18n.t("createNewService")
            };
            egui::Window::new(title)
                .id(Id::new("service_modal"))
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .fixed_size(Vec2::new(460.0, 440.0))
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.add_space(6.0);
                    ui.label(RichText::new(i18n.t("serviceName")).strong());
                    ui.add_sized(
                        [430.0, 28.0],
                        egui::TextEdit::singleline(&mut self.service_form.title),
                    );

                    ui.add_space(8.0);
                    if !self.service_form.creating_new_series {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(i18n.t("sermonSeries")).strong());
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if ui
                                    .small_button(format!("➕ {}", i18n.t("createNewSeries")))
                                    .clicked()
                                {
                                    self.service_form.creating_new_series = true;
                                }
                            });
                        });

                        let selected_series_label = if self.service_form.series_id.is_empty() {
                            i18n.t("standaloneNoSeries").to_string()
                        } else {
                            self.series_list
                                .iter()
                                .find(|s| s.id == self.service_form.series_id)
                                .map(|s| s.title.clone())
                                .unwrap_or_else(|| i18n.t("standaloneNoSeries").to_string())
                        };

                        egui::ComboBox::from_id_salt("service_series_select")
                            .width(430.0)
                            .selected_text(selected_series_label)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.service_form.series_id,
                                    String::new(),
                                    i18n.t("standaloneNoSeries"),
                                );
                                for s in &self.series_list {
                                    ui.selectable_value(
                                        &mut self.service_form.series_id,
                                        s.id.clone(),
                                        &s.title,
                                    );
                                }
                            });
                    } else {
                        Frame::group(ui.style()).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(i18n.t("createNewSeries")).strong());
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui.small_button(i18n.t("cancelPickExisting")).clicked() {
                                        self.service_form.creating_new_series = false;
                                        self.service_form.new_series_title.clear();
                                        self.service_form.new_series_artwork = None;
                                    }
                                });
                            });
                            ui.label(i18n.t("seriesTitle"));
                            ui.text_edit_singleline(&mut self.service_form.new_series_title);
                            ui.horizontal(|ui| {
                                if ui.button(format!("🖼 {}", i18n.t("dropzoneText"))).clicked() {
                                    if let Some(path) = rfd::FileDialog::new()
                                        .add_filter("Images", &["png", "jpg", "jpeg", "webp"])
                                        .pick_file()
                                    {
                                        self.service_form.new_series_artwork =
                                            encode_image_file_to_base64(&path);
                                    }
                                }
                                if self.service_form.new_series_artwork.is_some() {
                                    ui.label(
                                        RichText::new("✔ Image Loaded")
                                            .color(Color32::from_rgb(34, 197, 94)),
                                    );
                                    if ui.small_button("🗑").clicked() {
                                        self.service_form.new_series_artwork = None;
                                    }
                                }
                            });
                        });
                    }

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new(i18n.t("sermonTitleTopic")).strong());
                            ui.add_sized(
                                [280.0, 28.0],
                                egui::TextEdit::singleline(&mut self.service_form.sermon_title),
                            );
                        });
                        ui.vertical(|ui| {
                            ui.label(RichText::new(i18n.t("weekPart")).strong());
                            ui.add_sized(
                                [135.0, 28.0],
                                egui::TextEdit::singleline(&mut self.service_form.week_number),
                            );
                        });
                    });

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new(i18n.t("date")).strong());
                            ui.add_sized(
                                [210.0, 28.0],
                                egui::TextEdit::singleline(&mut self.service_form.date),
                            );
                        });
                        ui.vertical(|ui| {
                            ui.label(RichText::new(i18n.t("time")).strong());
                            ui.add_sized(
                                [205.0, 28.0],
                                egui::TextEdit::singleline(&mut self.service_form.time),
                            );
                        });
                    });

                    ui.add_space(16.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(i18n.t("cancel")).clicked() {
                            self.show_service_modal = false;
                        }
                        let save_lbl = if self.service_form.editing_id.is_some() {
                            i18n.t("saveChanges")
                        } else {
                            i18n.t("createService")
                        };
                        let btn = egui::Button::new(
                            RichText::new(save_lbl).color(Color32::WHITE).strong(),
                        )
                        .fill(Color32::from_rgb(0, 120, 212));
                        if ui.add(btn).clicked() {
                            self.save_service_from_modal();
                        }
                    });
                });
            if !open {
                self.show_service_modal = false;
            }
        }

        // 2. Create / Edit Sermon Series Modal
        if self.show_series_modal {
            let mut open = self.show_series_modal;
            let title = if self.series_form.editing_id.is_some() {
                i18n.t("editSeries")
            } else {
                i18n.t("createSermonSeries")
            };
            egui::Window::new(title)
                .id(Id::new("series_modal"))
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .fixed_size(Vec2::new(450.0, 360.0))
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label(RichText::new(i18n.t("seriesTitle")).strong());
                    ui.add_sized(
                        [420.0, 28.0],
                        egui::TextEdit::singleline(&mut self.series_form.title),
                    );

                    ui.add_space(8.0);
                    ui.label(RichText::new(i18n.t("descriptionOptional")).strong());
                    ui.add_sized(
                        [420.0, 64.0],
                        egui::TextEdit::multiline(&mut self.series_form.description),
                    );

                    ui.add_space(8.0);
                    ui.label(RichText::new(i18n.t("seriesArtworkGraphic")).strong());
                    ui.horizontal(|ui| {
                        if ui.button(format!("🖼  {}", i18n.t("dropzoneText"))).clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("Images", &["png", "jpg", "jpeg", "webp"])
                                .pick_file()
                            {
                                self.series_form.artwork = encode_image_file_to_base64(&path);
                            }
                        }
                        if self.series_form.artwork.is_some() {
                            ui.label(
                                RichText::new("✔ Artwork attached")
                                    .color(Color32::from_rgb(34, 197, 94)),
                            );
                            if ui.small_button("🗑").clicked() {
                                self.series_form.artwork = None;
                            }
                        }
                    });

                    ui.add_space(16.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(i18n.t("cancel")).clicked() {
                            self.show_series_modal = false;
                        }
                        let btn = egui::Button::new(
                            RichText::new(i18n.t("saveSeries"))
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(Color32::from_rgb(0, 120, 212));
                        if ui.add(btn).clicked() && !self.series_form.title.trim().is_empty() {
                            let s = SermonSeries {
                                id: self.series_form.editing_id.clone().unwrap_or_default(),
                                title: self.series_form.title.trim().to_string(),
                                description: self.series_form.description.trim().to_string(),
                                artwork: self.series_form.artwork.clone(),
                                created_at: String::new(),
                            };
                            let _ = self.db.save_sermon_series(s);
                            self.show_series_modal = false;
                            self.reload_data();
                        }
                    });
                });
            if !open {
                self.show_series_modal = false;
            }
        }

        // 3. Add Item to Service Run Sheet Modal
        if self.show_add_item_modal {
            let mut open = self.show_add_item_modal;
            let modal_title = match self.add_item_form.item_type {
                ItemType::Song => i18n.t("addSong"),
                ItemType::Conference => i18n.t("addConference"),
                ItemType::Video => i18n.t("addVideo"),
                ItemType::Loop => i18n.t("addLoop"),
                ItemType::Announcements => i18n.t("addAnnouncements"),
                ItemType::Countdown => i18n.t("addCountdown"),
            };

            egui::Window::new(modal_title)
                .id(Id::new("add_item_modal"))
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .fixed_size(Vec2::new(480.0, 420.0))
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    if self.add_item_form.item_type == ItemType::Song {
                        if self.add_item_form.selected_song_id.is_empty() {
                            ui.label(RichText::new(i18n.t("chooseFromSongBank")).strong());
                            ui.add_sized(
                                [440.0, 28.0],
                                egui::TextEdit::singleline(
                                    &mut self.add_item_form.song_search_query,
                                )
                                .hint_text(i18n.t("searchSongs")),
                            );
                            ui.add_space(6.0);
                            let q = self.add_item_form.song_search_query.trim().to_lowercase();
                            let matching: Vec<Song> = self
                                .songs
                                .iter()
                                .filter(|s| {
                                    q.is_empty()
                                        || s.title.to_lowercase().contains(&q)
                                        || s.artist.to_lowercase().contains(&q)
                                })
                                .cloned()
                                .collect();

                            ScrollArea::vertical().max_height(210.0).show(ui, |ui| {
                                if matching.is_empty() {
                                    ui.label(
                                        RichText::new(i18n.t("noResultsFound"))
                                            .color(Color32::GRAY),
                                    );
                                }
                                for song in matching {
                                    let default_arr = song.arrangements.first().cloned();
                                    let btn_txt = format!(
                                        "🎵 {} — {} ({})",
                                        song.title,
                                        if song.artist.is_empty() {
                                            "—"
                                        } else {
                                            &song.artist
                                        },
                                        default_arr
                                            .as_ref()
                                            .map(|a| a.key.as_str())
                                            .unwrap_or("C")
                                    );
                                    if ui
                                        .add_sized([430.0, 32.0], egui::Button::new(btn_txt))
                                        .clicked()
                                    {
                                        self.add_item_form.selected_song_id = song.id.clone();
                                        self.add_item_form.title = song.title.clone();
                                        self.add_item_form.subtitle = song.artist.clone();
                                        if let Some(arr) = default_arr {
                                            self.add_item_form.selected_arrangement_id = arr.id;
                                            self.add_item_form.arrangement_name = arr.name;
                                            self.add_item_form.key = arr.key;
                                            self.add_item_form.duration_mins = arr.duration / 60;
                                            self.add_item_form.duration_secs = arr.duration % 60;
                                        }
                                    }
                                }
                            });
                        } else {
                            Frame::group(ui.style()).show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label(
                                            RichText::new(&self.add_item_form.title)
                                                .size(15.0)
                                                .strong(),
                                        );
                                        if !self.add_item_form.subtitle.is_empty() {
                                            ui.label(
                                                RichText::new(&self.add_item_form.subtitle)
                                                    .size(12.0)
                                                    .color(Color32::GRAY),
                                            );
                                        }
                                    });
                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        if ui.button(i18n.t("change")).clicked() {
                                            self.add_item_form.selected_song_id.clear();
                                        }
                                    });
                                });
                            });

                            if let Some(sel_song) = self
                                .songs
                                .iter()
                                .find(|s| s.id == self.add_item_form.selected_song_id)
                                .cloned()
                            {
                                if sel_song.arrangements.len() > 1 {
                                    ui.add_space(6.0);
                                    ui.label(RichText::new(i18n.t("arrangement")).strong());
                                    egui::ComboBox::from_id_salt("add_item_arr_select")
                                        .width(440.0)
                                        .selected_text(format!(
                                            "{} ({})",
                                            self.add_item_form.arrangement_name,
                                            self.add_item_form.key
                                        ))
                                        .show_ui(ui, |ui| {
                                            for arr in &sel_song.arrangements {
                                                let lbl = format!(
                                                    "{} ({}) — {}",
                                                    arr.name,
                                                    arr.key,
                                                    arr.format_duration()
                                                );
                                                if ui
                                                    .selectable_label(
                                                        self.add_item_form.selected_arrangement_id
                                                            == arr.id,
                                                        lbl,
                                                    )
                                                    .clicked()
                                                {
                                                    self.add_item_form.selected_arrangement_id =
                                                        arr.id.clone();
                                                    self.add_item_form.arrangement_name =
                                                        arr.name.clone();
                                                    self.add_item_form.key = arr.key.clone();
                                                    self.add_item_form.duration_mins =
                                                        arr.duration / 60;
                                                    self.add_item_form.duration_secs =
                                                        arr.duration % 60;
                                                }
                                            }
                                        });
                                }
                            }

                            ui.add_space(6.0);
                            ui.label(RichText::new(i18n.t("mainVocalistLead")).strong());
                            ui.add_sized(
                                [440.0, 28.0],
                                egui::TextEdit::singleline(&mut self.add_item_form.lead_vocalist),
                            );

                            ui.add_space(6.0);
                            ui.label(RichText::new(i18n.t("extraNotesTechnicalCues")).strong());
                            ui.add_sized(
                                [440.0, 56.0],
                                egui::TextEdit::multiline(&mut self.add_item_form.notes),
                            );
                        }
                    } else {
                        ui.label(RichText::new(i18n.t("titleName")).strong());
                        ui.add_sized(
                            [440.0, 28.0],
                            egui::TextEdit::singleline(&mut self.add_item_form.title),
                        );

                        if self.add_item_form.item_type == ItemType::Conference {
                            ui.add_space(6.0);
                            ui.label(RichText::new(i18n.t("speaker")).strong());
                            ui.add_sized(
                                [440.0, 28.0],
                                egui::TextEdit::singleline(&mut self.add_item_form.lead_vocalist),
                            );
                        }

                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new(i18n.t("durationMinutes")).strong());
                                ui.add(
                                    egui::DragValue::new(&mut self.add_item_form.duration_mins)
                                        .range(0..=240),
                                );
                            });
                            ui.add_space(24.0);
                            ui.vertical(|ui| {
                                ui.label(RichText::new(i18n.t("durationSeconds")).strong());
                                ui.add(
                                    egui::DragValue::new(&mut self.add_item_form.duration_secs)
                                        .range(0..=59),
                                );
                            });
                        });

                        ui.add_space(6.0);
                        ui.label(RichText::new(i18n.t("extraNotesTechnicalCues")).strong());
                        ui.add_sized(
                            [440.0, 64.0],
                            egui::TextEdit::multiline(&mut self.add_item_form.notes),
                        );
                    }

                    ui.add_space(14.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(i18n.t("cancel")).clicked() {
                            self.show_add_item_modal = false;
                        }
                        let can_submit = if self.add_item_form.item_type == ItemType::Song {
                            !self.add_item_form.selected_song_id.is_empty()
                        } else {
                            !self.add_item_form.title.trim().is_empty()
                        };
                        let btn = egui::Button::new(
                            RichText::new(i18n.t("addToSheet"))
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(Color32::from_rgb(0, 120, 212));
                        if ui.add_enabled(can_submit, btn).clicked() {
                            if let Some(ref svc_id) = self.active_service_id {
                                let new_item = ServiceItem {
                                    id: Uuid::new_v4().to_string(),
                                    service_id: svc_id.clone(),
                                    item_type: self.add_item_form.item_type,
                                    title: self.add_item_form.title.trim().to_string(),
                                    subtitle: self.add_item_form.subtitle.trim().to_string(),
                                    arrangement_name: if self.add_item_form.item_type
                                        == ItemType::Song
                                    {
                                        self.add_item_form.arrangement_name.clone()
                                    } else {
                                        String::new()
                                    },
                                    key: if self.add_item_form.item_type == ItemType::Song {
                                        self.add_item_form.key.clone()
                                    } else {
                                        String::new()
                                    },
                                    lead_vocalist: self
                                        .add_item_form
                                        .lead_vocalist
                                        .trim()
                                        .to_string(),
                                    duration: self.add_item_form.duration_mins * 60
                                        + self.add_item_form.duration_secs,
                                    order: self.builder_items.len() as i32,
                                    notes: self.add_item_form.notes.trim().to_string(),
                                };
                                let _ = self.db.save_item(new_item);
                                self.show_add_item_modal = false;
                                self.reload_data();
                            }
                        }
                    });
                });
            if !open {
                self.show_add_item_modal = false;
            }
        }

        // 4. New Song Modal
        if self.show_new_song_modal {
            let mut open = self.show_new_song_modal;
            egui::Window::new(i18n.t("newSong"))
                .id(Id::new("new_song_modal"))
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .fixed_size(Vec2::new(430.0, 360.0))
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label(RichText::new(i18n.t("songTitle")).strong());
                    ui.add_sized(
                        [400.0, 28.0],
                        egui::TextEdit::singleline(&mut self.new_song_form.title),
                    );

                    ui.add_space(6.0);
                    ui.label(RichText::new(i18n.t("artistAuthor")).strong());
                    ui.add_sized(
                        [400.0, 28.0],
                        egui::TextEdit::singleline(&mut self.new_song_form.artist),
                    );

                    ui.add_space(8.0);
                    Frame::group(ui.style()).show(ui, |ui| {
                        ui.label(RichText::new(i18n.t("defaultArrangementName")).strong());
                        ui.text_edit_singleline(&mut self.new_song_form.default_arr_name);

                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(i18n.t("key")).strong());
                            egui::ComboBox::from_id_salt("new_song_key_select")
                                .selected_text(&self.new_song_form.key)
                                .show_ui(ui, |ui| {
                                    for &k in DEFAULT_KEYS {
                                        ui.selectable_value(
                                            &mut self.new_song_form.key,
                                            k.to_string(),
                                            k,
                                        );
                                    }
                                });

                            ui.add_space(16.0);
                            ui.label(RichText::new(i18n.t("bpm")).strong());
                            ui.add(egui::DragValue::new(&mut self.new_song_form.bpm).range(30..=260));
                        });

                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(i18n.t("durationMinutes"));
                            ui.add(
                                egui::DragValue::new(&mut self.new_song_form.duration_mins)
                                    .range(0..=60),
                            );
                            ui.add_space(12.0);
                            ui.label(i18n.t("durationSeconds"));
                            ui.add(
                                egui::DragValue::new(&mut self.new_song_form.duration_secs)
                                    .range(0..=59),
                            );
                        });
                    });

                    ui.add_space(12.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(i18n.t("cancel")).clicked() {
                            self.show_new_song_modal = false;
                        }
                        let btn = egui::Button::new(
                            RichText::new(i18n.t("addSongToBank"))
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(Color32::from_rgb(0, 120, 212));
                        if ui.add(btn).clicked() && !self.new_song_form.title.trim().is_empty() {
                            let arr = Arrangement {
                                id: Uuid::new_v4().to_string(),
                                name: if self.new_song_form.default_arr_name.trim().is_empty() {
                                    "Standard".to_string()
                                } else {
                                    self.new_song_form.default_arr_name.trim().to_string()
                                },
                                key: self.new_song_form.key.clone(),
                                bpm: self.new_song_form.bpm,
                                duration: self.new_song_form.duration_mins * 60
                                    + self.new_song_form.duration_secs,
                            };
                            let song = Song {
                                id: Uuid::new_v4().to_string(),
                                title: self.new_song_form.title.trim().to_string(),
                                artist: self.new_song_form.artist.trim().to_string(),
                                arrangements: vec![arr],
                            };
                            let _ = self.db.save_song(song);
                            self.show_new_song_modal = false;
                            self.reload_data();
                        }
                    });
                });
            if !open {
                self.show_new_song_modal = false;
            }
        }

        // 5. Add Arrangement Modal
        if self.show_new_arr_modal {
            let mut open = self.show_new_arr_modal;
            egui::Window::new(format!(
                "{}: {}",
                i18n.t("addArrangement"),
                self.new_arr_form.target_song_title
            ))
            .id(Id::new("new_arr_modal"))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .fixed_size(Vec2::new(400.0, 260.0))
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.label(RichText::new(i18n.t("arrangementName")).strong());
                ui.text_edit_singleline(&mut self.new_arr_form.name);

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new(i18n.t("key")).strong());
                    egui::ComboBox::from_id_salt("new_arr_key_select")
                        .selected_text(&self.new_arr_form.key)
                        .show_ui(ui, |ui| {
                            for &k in DEFAULT_KEYS {
                                ui.selectable_value(&mut self.new_arr_form.key, k.to_string(), k);
                            }
                        });
                    ui.add_space(16.0);
                    ui.label(RichText::new(i18n.t("bpm")).strong());
                    ui.add(egui::DragValue::new(&mut self.new_arr_form.bpm).range(30..=260));
                });

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(i18n.t("durationMinutes"));
                    ui.add(
                        egui::DragValue::new(&mut self.new_arr_form.duration_mins).range(0..=60),
                    );
                    ui.add_space(12.0);
                    ui.label(i18n.t("durationSeconds"));
                    ui.add(
                        egui::DragValue::new(&mut self.new_arr_form.duration_secs).range(0..=59),
                    );
                });

                ui.add_space(12.0);
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button(i18n.t("cancel")).clicked() {
                        self.show_new_arr_modal = false;
                    }
                    let btn = egui::Button::new(
                        RichText::new(i18n.t("addArrangement"))
                            .color(Color32::WHITE)
                            .strong(),
                    )
                    .fill(Color32::from_rgb(0, 120, 212));
                    if ui.add(btn).clicked() && !self.new_arr_form.name.trim().is_empty() {
                        if let Some(mut song) = self
                            .songs
                            .iter()
                            .find(|s| s.id == self.new_arr_form.target_song_id)
                            .cloned()
                        {
                            song.arrangements.push(Arrangement {
                                id: Uuid::new_v4().to_string(),
                                name: self.new_arr_form.name.trim().to_string(),
                                key: self.new_arr_form.key.clone(),
                                bpm: self.new_arr_form.bpm,
                                duration: self.new_arr_form.duration_mins * 60
                                    + self.new_arr_form.duration_secs,
                            });
                            let _ = self.db.save_song(song);
                            self.show_new_arr_modal = false;
                            self.reload_data();
                        }
                    }
                });
            });
            if !open {
                self.show_new_arr_modal = false;
            }
        }

        // 6. Confirmation Modal
        if let Some((prompt, action)) = self.confirm_modal.clone() {
            egui::Window::new("Confirm")
                .id(Id::new("confirm_dialog"))
                .collapsible(false)
                .resizable(false)
                .fixed_size(Vec2::new(380.0, 140.0))
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.label(RichText::new(prompt).size(14.0));
                    ui.add_space(16.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(i18n.t("cancel")).clicked() {
                            self.confirm_modal = None;
                        }
                        let del_btn = egui::Button::new(
                            RichText::new(i18n.t("delete"))
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(Color32::from_rgb(220, 38, 38));
                        if ui.add(del_btn).clicked() {
                            match action {
                                ConfirmAction::DeleteService(id) => {
                                    let _ = self.db.delete_service(&id);
                                }
                                ConfirmAction::DeleteSeries(id) => {
                                    let _ = self.db.delete_sermon_series(&id);
                                }
                                ConfirmAction::DeleteSong(id) => {
                                    let _ = self.db.delete_song(&id);
                                }
                            }
                            self.confirm_modal = None;
                            self.reload_data();
                        }
                    });
                });
        }
    }
}

impl eframe::App for GrancoPlannerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.handle_dropped_files(ctx);
        self.handle_screenshot_events(ctx);

        self.render_sidebar(ctx);

        egui::CentralPanel::default()
            .frame(Frame::NONE.fill(ctx.style().visuals.panel_fill))
            .show(ctx, |ui| match self.current_view {
                ActiveView::Services => self.render_services_view(ctx, ui),
                ActiveView::Calendar => self.render_calendar_view(ctx, ui),
                ActiveView::Series => self.render_series_view(ctx, ui),
                ActiveView::Builder => self.render_builder_view(ctx, ui),
                ActiveView::SongBank => self.render_song_bank_view(ctx, ui),
                ActiveView::Stats => self.render_analytics_view(ctx, ui),
                ActiveView::Settings => self.render_settings_view(ctx, ui),
            });

        self.render_modals(ctx);

        if let Some((ref msg, exp)) = self.status_toast.clone() {
            if ctx.input(|i| i.time) < exp {
                egui::Window::new("toast")
                    .title_bar(false)
                    .resizable(false)
                    .anchor(egui::Align2::RIGHT_BOTTOM, Vec2::new(-20.0, -20.0))
                    .show(ctx, |ui| {
                        ui.label(
                            RichText::new(format!("✔ {}", msg))
                                .color(Color32::from_rgb(34, 197, 94))
                                .strong(),
                        );
                    });
            } else {
                self.status_toast = None;
            }
        }
    }
}
