pub mod crud;
pub mod feature;
pub mod handler;
pub mod middleware;
pub mod migration;
pub mod model;
pub mod worker;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FieldInfo {
    pub name: String,
    pub rust_type: String,
    pub sql_type: String,
    pub nullable: bool,
    pub default: Option<String>,
    pub validation: Option<String>,
}

impl FieldInfo {
    pub fn parse(field_str: &str) -> Option<Self> {
        let parts: Vec<&str> = field_str.split(':').collect();
        if parts.len() < 2 {
            return None;
        }

        let name = parts[0].to_string();
        let type_str = parts[1];

        let (rust_type, sql_type, validation) = match type_str.to_lowercase().as_str() {
            "string" | "str" => (
                "String".to_string(),
                "VARCHAR(255)".to_string(),
                Some("length(min = 1, max = 255)".to_string()),
            ),
            "text" => ("String".to_string(), "TEXT".to_string(), None),
            "i32" | "int" | "integer" => ("i32".to_string(), "INTEGER".to_string(), None),
            "i64" | "bigint" => ("i64".to_string(), "BIGINT".to_string(), None),
            "f32" | "float" => ("f32".to_string(), "REAL".to_string(), None),
            "f64" | "double" => ("f64".to_string(), "DOUBLE PRECISION".to_string(), None),
            "bool" | "boolean" => ("bool".to_string(), "BOOLEAN".to_string(), None),
            "uuid" => ("Uuid".to_string(), "UUID".to_string(), None),
            "date" => ("chrono::NaiveDate".to_string(), "DATE".to_string(), None),
            "datetime" | "timestamp" => {
                ("DateTime<Utc>".to_string(), "TIMESTAMPTZ".to_string(), None)
            }
            "json" | "jsonb" => ("serde_json::Value".to_string(), "JSONB".to_string(), None),
            "email" => (
                "String".to_string(),
                "VARCHAR(255)".to_string(),
                Some("email".to_string()),
            ),
            "url" => (
                "String".to_string(),
                "VARCHAR(2048)".to_string(),
                Some("url".to_string()),
            ),
            _ => (type_str.to_string(), "VARCHAR(255)".to_string(), None),
        };

        Some(Self {
            name,
            rust_type,
            sql_type,
            nullable: false,
            default: None,
            validation,
        })
    }

    pub fn parse_many(fields: &[String]) -> Vec<Self> {
        fields.iter().filter_map(|f| Self::parse(f)).collect()
    }
}
