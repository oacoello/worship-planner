use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const DEFAULT_KEYS: &[&str] = &[
    "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B", "Am", "Bm", "Cm", "Dm",
    "Em", "F#m", "Gm",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Es,
    En,
}

impl Language {
    pub fn as_code(&self) -> &'static str {
        match self {
            Language::Es => "es",
            Language::En => "en",
        }
    }

    pub fn from_code(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "en" => Language::En,
            _ => Language::Es,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}

impl ThemeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
            ThemeMode::System => "system",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "light" => ThemeMode::Light,
            "system" => ThemeMode::System,
            _ => ThemeMode::Dark,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub language: Language,
    pub theme_mode: ThemeMode,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: Language::Es,
            theme_mode: ThemeMode::Dark,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SermonSeries {
    pub id: String,
    pub title: String,
    pub description: String,
    /// Optional base64-encoded PNG/JPEG image or data URI
    pub artwork: Option<String>,
    pub created_at: String,
}

impl SermonSeries {
    pub fn new(title: String, description: String, artwork: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            artwork,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePlan {
    pub id: String,
    pub title: String,
    pub series_id: String,
    pub series: String,
    pub sermon_title: String,
    pub week_number: String,
    pub sermon_art: Option<String>,
    /// YYYY-MM-DD
    pub date: String,
    /// HH:MM (24-hour)
    pub time: String,
}

impl Default for ServicePlan {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: "Sunday Service".to_string(),
            series_id: String::new(),
            series: String::new(),
            sermon_title: String::new(),
            week_number: "Week 1".to_string(),
            sermon_art: None,
            date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            time: "10:00".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Song,
    Conference,
    Video,
    Loop,
    Announcements,
    Countdown,
}

impl ItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemType::Song => "song",
            ItemType::Conference => "conference",
            ItemType::Video => "video",
            ItemType::Loop => "loop",
            ItemType::Announcements => "announcements",
            ItemType::Countdown => "countdown",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "conference" => ItemType::Conference,
            "video" => ItemType::Video,
            "loop" => ItemType::Loop,
            "announcements" => ItemType::Announcements,
            "countdown" => ItemType::Countdown,
            _ => ItemType::Song,
        }
    }

    pub fn default_duration_mins(&self) -> u32 {
        match self {
            ItemType::Song => 5,
            ItemType::Video => 3,
            ItemType::Loop => 4,
            ItemType::Announcements => 4,
            ItemType::Conference => 35,
            ItemType::Countdown => 5,
        }
    }

    pub fn icon(&self) -> crate::icons::Icon {
        match self {
            ItemType::Song => crate::icons::Icon::Song,
            ItemType::Conference => crate::icons::Icon::Conference,
            ItemType::Video => crate::icons::Icon::Video,
            ItemType::Loop => crate::icons::Icon::Loop,
            ItemType::Announcements => crate::icons::Icon::Announcements,
            ItemType::Countdown => crate::icons::Icon::Countdown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceItem {
    pub id: String,
    pub service_id: String,
    pub item_type: ItemType,
    pub title: String,
    pub subtitle: String,
    pub arrangement_name: String,
    pub key: String,
    pub lead_vocalist: String,
    /// Duration in seconds
    pub duration: u32,
    pub order: i32,
    pub notes: String,
}

impl ServiceItem {
    pub fn format_duration(&self) -> String {
        let mins = self.duration / 60;
        let secs = self.duration % 60;
        format!("{}:{:02}", mins, secs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arrangement {
    pub id: String,
    pub name: String,
    pub key: String,
    pub bpm: u32,
    /// Duration in seconds
    pub duration: u32,
}

impl Default for Arrangement {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "Standard".to_string(),
            key: "C".to_string(),
            bpm: 120,
            duration: 300,
        }
    }
}

impl Arrangement {
    pub fn format_duration(&self) -> String {
        let mins = self.duration / 60;
        let secs = self.duration % 60;
        format!("{}:{:02}", mins, secs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub arrangements: Vec<Arrangement>,
}

impl Song {
    pub fn normalized(mut self) -> Self {
        if self.arrangements.is_empty() {
            self.arrangements.push(Arrangement::default());
        }
        self
    }

    pub fn key_summary(&self) -> String {
        let mut keys = Vec::new();
        for arr in &self.arrangements {
            if !arr.key.is_empty() && !keys.contains(&arr.key) {
                keys.push(arr.key.clone());
            }
        }
        if keys.is_empty() {
            "-".to_string()
        } else {
            keys.join(", ")
        }
    }

    pub fn bpm_summary(&self) -> String {
        let bpms: Vec<u32> = self
            .arrangements
            .iter()
            .map(|a| a.bpm)
            .filter(|&b| b > 0)
            .collect();
        if bpms.is_empty() {
            "-".to_string()
        } else {
            let min = *bpms.iter().min().unwrap();
            let max = *bpms.iter().max().unwrap();
            if min == max {
                format!("{}", min)
            } else {
                format!("{}-{}", min, max)
            }
        }
    }
}
