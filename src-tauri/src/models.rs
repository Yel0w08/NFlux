use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a single note with all its properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub color: NoteColor,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Available colors for notes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NoteColor {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "teal")]
    Teal,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "purple")]
    Purple,
}

impl Default for NoteColor {
    fn default() -> Self {
        NoteColor::Default
    }
}

impl NoteColor {
    pub fn from_string(s: &str) -> Self {
        match s {
            "red" => NoteColor::Red,
            "orange" => NoteColor::Orange,
            "yellow" => NoteColor::Yellow,
            "green" => NoteColor::Green,
            "teal" => NoteColor::Teal,
            "blue" => NoteColor::Blue,
            "purple" => NoteColor::Purple,
            _ => NoteColor::Default,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            NoteColor::Default => "default",
            NoteColor::Red => "red",
            NoteColor::Orange => "orange",
            NoteColor::Yellow => "yellow",
            NoteColor::Green => "green",
            NoteColor::Teal => "teal",
            NoteColor::Blue => "blue",
            NoteColor::Purple => "purple",
        }
    }
}

/// Request payload for creating a new note
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNoteRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}

/// Request payload for updating a note
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNoteRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub color: Option<String>,
    pub archived: Option<bool>,
}

/// Response wrapper for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

impl Note {
    /// Create a new note with default values
    pub fn new(title: String, content: String) -> Self {
        let now = Utc::now();
        Note {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            color: NoteColor::default(),
            archived: false,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get a preview of the note content (first 50 chars without markdown)
    pub fn preview(&self, length: usize) -> String {
        self.content
            .chars()
            .filter(|c| !matches!(c, '#' | '*' | '`' | '>' | '-' | '_' | '[' | ']' | '(' | ')' | '{' | '}'))
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
            .take(length)
            .collect()
    }

    /// Update the modification timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Get word count
    pub fn word_count(&self) -> usize {
        self.content
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .count()
    }

    /// Get character count
    pub fn char_count(&self) -> usize {
        self.content.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_note() {
        let note = Note::new("Test".to_string(), "Content".to_string());
        assert_eq!(note.title, "Test");
        assert_eq!(note.content, "Content");
        assert!(!note.archived);
    }

    #[test]
    fn test_preview() {
        let note = Note::new("Test".to_string(), "# Hello **world** test".to_string());
        let preview = note.preview(20);
        assert_eq!(preview, "Hello world test");
    }
}
