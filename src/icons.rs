use egui::{
    Align, Color32, ColorImage, Context, CornerRadius, Layout, RichText, Sense, Stroke,
    TextureHandle, TextureOptions, Ui, Vec2,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    // App & Sidebar Navigation
    AppLogo,
    Services,
    Calendar,
    Series,
    SongBank,
    Analytics,
    Settings,

    // Item Types
    Song,
    Conference,
    Video,
    Loop,
    Announcements,
    Countdown,

    // Actions & Controls
    Add,
    Edit,
    Delete,
    Back,
    Print,
    Camera,
    Search,
    Play,
    ArrowUp,
    ArrowDown,
    ArrowRight,
    DropDown,
    Check,

    // Metadata, Placeholders, Analytics KPIs & Settings
    EmptyClipboard,
    LandscapeView,
    CalendarCheck,
    Clock,
    StarMedal,
    Keyboard,
    Hourglass,
    PaintPalette,
    MoonStars,
    Sun,
    ComputerDesktop,
    Earth,
}

impl Icon {
    pub const ALL: &'static [Icon] = &[
        Icon::AppLogo,
        Icon::Services,
        Icon::Calendar,
        Icon::Series,
        Icon::SongBank,
        Icon::Analytics,
        Icon::Settings,
        Icon::Song,
        Icon::Conference,
        Icon::Video,
        Icon::Loop,
        Icon::Announcements,
        Icon::Countdown,
        Icon::Add,
        Icon::Edit,
        Icon::Delete,
        Icon::Back,
        Icon::Print,
        Icon::Camera,
        Icon::Search,
        Icon::Play,
        Icon::ArrowUp,
        Icon::ArrowDown,
        Icon::ArrowRight,
        Icon::DropDown,
        Icon::Check,
        Icon::EmptyClipboard,
        Icon::LandscapeView,
        Icon::CalendarCheck,
        Icon::Clock,
        Icon::StarMedal,
        Icon::Keyboard,
        Icon::Hourglass,
        Icon::PaintPalette,
        Icon::MoonStars,
        Icon::Sun,
        Icon::ComputerDesktop,
        Icon::Earth,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Icon::AppLogo => "music-note-trebble-clef",
            Icon::Services => "calendar-mark",
            Icon::Calendar => "calendar-heart",
            Icon::Series => "gallery-2",
            Icon::SongBank => "music-note-2",
            Icon::Analytics => "graph-bar-increase",
            Icon::Settings => "cog",
            Icon::Song => "music-note-2",
            Icon::Conference => "user-podcast",
            Icon::Video => "camera-video",
            Icon::Loop => "synchronize",
            Icon::Announcements => "announcement-megaphone",
            Icon::Countdown => "stopwatch-half",
            Icon::Add => "application-add",
            Icon::Edit => "pencil-square",
            Icon::Delete => "recycle-bin-2",
            Icon::Back => "return-3",
            Icon::Print => "printer",
            Icon::Camera => "camera-1",
            Icon::Search => "search-visual",
            Icon::Play => "button-play-circle",
            Icon::ArrowUp => "arrow-up-4",
            Icon::ArrowDown => "arrow-turn-down-large",
            Icon::ArrowRight => "arrow-right-circle-1",
            Icon::DropDown => "drop-down-menu",
            Icon::Check => "check-thick",
            Icon::EmptyClipboard => "empty-clipboard",
            Icon::LandscapeView => "landscape-view",
            Icon::CalendarCheck => "calendar-check",
            Icon::Clock => "circle-clock",
            Icon::StarMedal => "star-medal",
            Icon::Keyboard => "keyboard",
            Icon::Hourglass => "hourglass",
            Icon::PaintPalette => "paint-palette",
            Icon::MoonStars => "moon-stars",
            Icon::Sun => "sun",
            Icon::ComputerDesktop => "computer-pc-desktop",
            Icon::Earth => "earth-1",
        }
    }

    pub fn svg_str(&self) -> &'static str {
        match self {
            Icon::AppLogo => include_str!("../assets/icons/music-note-trebble-clef.svg"),
            Icon::Services => include_str!("../assets/icons/calendar-mark.svg"),
            Icon::Calendar => include_str!("../assets/icons/calendar-heart.svg"),
            Icon::Series => include_str!("../assets/icons/gallery-2.svg"),
            Icon::SongBank | Icon::Song => include_str!("../assets/icons/music-note-2.svg"),
            Icon::Analytics => include_str!("../assets/icons/graph-bar-increase.svg"),
            Icon::Settings => include_str!("../assets/icons/cog.svg"),
            Icon::Conference => include_str!("../assets/icons/user-podcast.svg"),
            Icon::Video => include_str!("../assets/icons/camera-video.svg"),
            Icon::Loop => include_str!("../assets/icons/synchronize.svg"),
            Icon::Announcements => include_str!("../assets/icons/announcement-megaphone.svg"),
            Icon::Countdown => include_str!("../assets/icons/stopwatch-half.svg"),
            Icon::Add => include_str!("../assets/icons/application-add.svg"),
            Icon::Edit => include_str!("../assets/icons/pencil-square.svg"),
            Icon::Delete => include_str!("../assets/icons/recycle-bin-2.svg"),
            Icon::Back => include_str!("../assets/icons/return-3.svg"),
            Icon::Print => include_str!("../assets/icons/printer.svg"),
            Icon::Camera => include_str!("../assets/icons/camera-1.svg"),
            Icon::Search => include_str!("../assets/icons/search-visual.svg"),
            Icon::Play => include_str!("../assets/icons/button-play-circle.svg"),
            Icon::ArrowUp => include_str!("../assets/icons/arrow-up-4.svg"),
            Icon::ArrowDown => include_str!("../assets/icons/arrow-turn-down-large.svg"),
            Icon::ArrowRight => include_str!("../assets/icons/arrow-right-circle-1.svg"),
            Icon::DropDown => include_str!("../assets/icons/drop-down-menu.svg"),
            Icon::Check => include_str!("../assets/icons/check-thick.svg"),
            Icon::EmptyClipboard => include_str!("../assets/icons/empty-clipboard.svg"),
            Icon::LandscapeView => include_str!("../assets/icons/landscape-view.svg"),
            Icon::CalendarCheck => include_str!("../assets/icons/calendar-check.svg"),
            Icon::Clock => include_str!("../assets/icons/circle-clock.svg"),
            Icon::StarMedal => include_str!("../assets/icons/star-medal.svg"),
            Icon::Keyboard => include_str!("../assets/icons/keyboard.svg"),
            Icon::Hourglass => include_str!("../assets/icons/hourglass.svg"),
            Icon::PaintPalette => include_str!("../assets/icons/paint-palette.svg"),
            Icon::MoonStars => include_str!("../assets/icons/moon-stars.svg"),
            Icon::Sun => include_str!("../assets/icons/sun.svg"),
            Icon::ComputerDesktop => include_str!("../assets/icons/computer-pc-desktop.svg"),
            Icon::Earth => include_str!("../assets/icons/earth-1.svg"),
        }
    }

    /// Rasterizes the Streamline Plump Color SVG at `target_px x target_px` into an `egui::ColorImage`.
    pub fn rasterize(&self, target_px: u32) -> Option<ColorImage> {
        use usvg::TreeParsing;
        let opt = usvg::Options::default();
        let utree = usvg::Tree::from_str(self.svg_str(), &opt).ok()?;
        let rtree = resvg::Tree::from_usvg(&utree);
        let mut pixmap = tiny_skia::Pixmap::new(target_px, target_px)?;
        let sx = (target_px as f32) / rtree.size.width();
        let sy = (target_px as f32) / rtree.size.height();
        let transform = tiny_skia::Transform::from_scale(sx, sy);
        rtree.render(transform, &mut pixmap.as_mut());

        // tiny-skia stores premultiplied RGBA; convert to unmultiplied RGBA for ColorImage
        let mut rgba = Vec::with_capacity((target_px * target_px * 4) as usize);
        for pixel in pixmap.pixels() {
            let c = pixel.demultiply();
            rgba.push(c.red());
            rgba.push(c.green());
            rgba.push(c.blue());
            rgba.push(c.alpha());
        }
        Some(ColorImage::from_rgba_unmultiplied(
            [target_px as usize, target_px as usize],
            &rgba,
        ))
    }
}

#[derive(Default)]
pub struct IconCache {
    textures: HashMap<Icon, TextureHandle>,
}

impl IconCache {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn get(&mut self, ctx: &Context, icon: Icon) -> Option<TextureHandle> {
        if let Some(tex) = self.textures.get(&icon) {
            return Some(tex.clone());
        }
        let img = icon.rasterize(64)?;
        let tex = ctx.load_texture(
            format!("streamline_plump_{:?}", icon),
            img,
            TextureOptions::LINEAR,
        );
        self.textures.insert(icon, tex.clone());
        Some(tex)
    }

    /// Draws an icon of `size x size` logical points inside `ui`.
    pub fn draw(&mut self, ctx: &Context, ui: &mut Ui, icon: Icon, size: f32) {
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
        self.paint_at(ctx, ui, icon, rect, Color32::WHITE);
    }

    /// Paints an icon inside an existing `egui::Rect` with optional tint.
    pub fn paint_at(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        icon: Icon,
        rect: egui::Rect,
        tint: Color32,
    ) {
        if let Some(tex) = self.get(ctx, icon) {
            ui.painter().image(
                tex.id(),
                rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                tint,
            );
        }
    }

    /// Renders a full-width sidebar navigation button with a Streamline Plump Color icon and text label.
    pub fn sidebar_tab(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        icon: Icon,
        label: &str,
        selected: bool,
        is_dark: bool,
    ) -> bool {
        let desired_size = Vec2::new(ui.available_width(), 38.0);
        let (rect, resp) = ui.allocate_exact_size(desired_size, Sense::click());

        let bg_fill = if selected {
            if is_dark {
                Color32::from_rgb(0, 102, 180)
            } else {
                Color32::from_rgb(224, 242, 254)
            }
        } else if resp.hovered() {
            if is_dark {
                Color32::from_rgb(38, 38, 45)
            } else {
                Color32::from_rgb(241, 245, 249)
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

        ui.painter()
            .rect_filled(rect, CornerRadius::same(6), bg_fill);

        let icon_rect = egui::Rect::from_center_size(
            egui::pos2(rect.left() + 20.0, rect.center().y),
            Vec2::splat(20.0),
        );
        self.paint_at(ctx, ui, icon, icon_rect, Color32::WHITE);

        ui.painter().text(
            egui::pos2(rect.left() + 38.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::proportional(14.0),
            text_color,
        );

        resp.clicked()
    }

    /// Renders a primary or standard button with a Streamline Plump Color icon on the left and a label.
    pub fn button(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        icon: Icon,
        label: &str,
        primary: bool,
        min_size: Vec2,
    ) -> egui::Response {
        let text_color = if primary {
            Color32::WHITE
        } else {
            ui.style().visuals.text_color()
        };
        self.button_styled(ctx, ui, icon, label, primary, text_color, min_size)
    }

    pub fn button_danger(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        icon: Icon,
        label: &str,
        min_size: Vec2,
    ) -> egui::Response {
        self.button_styled(
            ctx,
            ui,
            icon,
            label,
            false,
            Color32::from_rgb(239, 68, 68),
            min_size,
        )
    }

    pub fn button_styled(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        icon: Icon,
        label: &str,
        primary: bool,
        text_color: Color32,
        min_size: Vec2,
    ) -> egui::Response {
        let font_id = egui::FontId::proportional(13.5);
        let galley =
            ui.painter()
                .layout_no_wrap(label.to_string(), font_id.clone(), text_color);
        let icon_sz = 18.0;
        let gap = if label.is_empty() { 0.0 } else { 8.0 };
        let content_w = icon_sz + gap + galley.size().x;
        let width = (content_w + 24.0).max(min_size.x);
        let height = min_size.y.max(32.0);

        let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());

        let (bg_fill, stroke) = if primary {
            let fill = if resp.is_pointer_button_down_on() {
                Color32::from_rgb(0, 96, 175)
            } else if resp.hovered() {
                Color32::from_rgb(14, 134, 228)
            } else {
                Color32::from_rgb(0, 120, 212)
            };
            (fill, Stroke::NONE)
        } else {
            let visuals = ui.style().interact(&resp);
            (visuals.bg_fill, visuals.bg_stroke)
        };

        ui.painter()
            .rect(rect, CornerRadius::same(6), bg_fill, stroke, egui::StrokeKind::Inside);

        let start_x = rect.center().x - content_w * 0.5;
        let icon_rect = egui::Rect::from_center_size(
            egui::pos2(start_x + icon_sz * 0.5, rect.center().y),
            Vec2::splat(icon_sz),
        );
        self.paint_at(ctx, ui, icon, icon_rect, Color32::WHITE);

        if !label.is_empty() {
            let text_pos = egui::pos2(
                start_x + icon_sz + gap,
                rect.center().y - galley.size().y * 0.5,
            );
            ui.painter().galley(text_pos, galley, text_color);
        }

        resp
    }

    /// Renders a compact icon-only button (`24x24` or custom size), with enabled state.
    pub fn icon_only_button(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        icon: Icon,
        size: f32,
        enabled: bool,
    ) -> egui::Response {
        let sense = if enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, resp) = ui.allocate_exact_size(Vec2::splat(size), sense);
        if enabled {
            let visuals = ui.style().interact(&resp);
            if resp.hovered() || resp.is_pointer_button_down_on() {
                ui.painter().rect(
                    rect,
                    CornerRadius::same(4),
                    visuals.bg_fill,
                    visuals.bg_stroke,
                    egui::StrokeKind::Inside,
                );
            }
        }
        let icon_sz = (size - 6.0).max(12.0);
        let icon_rect = egui::Rect::from_center_size(rect.center(), Vec2::splat(icon_sz));
        let tint = if enabled {
            Color32::WHITE
        } else {
            Color32::from_rgba_unmultiplied(150, 150, 150, 95)
        };
        self.paint_at(ctx, ui, icon, icon_rect, tint);
        resp
    }

    /// Renders an inline horizontal icon + RichText label.
    pub fn label_with_icon(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        icon: Icon,
        icon_size: f32,
        text: RichText,
    ) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            self.draw(ctx, ui, icon, icon_size);
            ui.label(text);
        });
    }

    /// Selectable card/pill button with a Streamline Plump Color icon (used in Settings for theme/language).
    pub fn selectable_button(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        icon: Icon,
        label: &str,
        selected: bool,
        min_size: Vec2,
    ) -> bool {
        let (rect, resp) = ui.allocate_exact_size(min_size, Sense::click());
        let is_dark = ui.visuals().dark_mode;

        let (bg_fill, stroke, text_col) = if selected {
            let fill = if is_dark {
                Color32::from_rgb(12, 74, 110)
            } else {
                Color32::from_rgb(224, 242, 254)
            };
            let border = Stroke::new(1.5_f32, Color32::from_rgb(56, 189, 248));
            let tc = if is_dark {
                Color32::WHITE
            } else {
                Color32::from_rgb(3, 105, 161)
            };
            (fill, border, tc)
        } else {
            let visuals = ui.style().interact(&resp);
            (visuals.bg_fill, visuals.bg_stroke, ui.visuals().text_color())
        };

        ui.painter()
            .rect(rect, CornerRadius::same(8), bg_fill, stroke, egui::StrokeKind::Inside);

        let icon_rect = egui::Rect::from_center_size(
            egui::pos2(rect.left() + 24.0, rect.center().y),
            Vec2::splat(22.0),
        );
        self.paint_at(ctx, ui, icon, icon_rect, Color32::WHITE);

        ui.painter().text(
            egui::pos2(rect.left() + 44.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::proportional(14.0),
            text_col,
        );

        let _ = Layout::left_to_right(Align::Center);
        resp.clicked()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_streamline_plump_icons_rasterize() {
        for &icon in Icon::ALL {
            let img = icon
                .rasterize(64)
                .unwrap_or_else(|| panic!("Failed to rasterize icon {:?} ({})", icon, icon.name()));
            assert_eq!(img.size, [64, 64]);
        }
    }
}
