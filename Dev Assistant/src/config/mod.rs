use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CourseMap {
    pub course: CourseInfo,
    pub weeks: Vec<WeekEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CourseInfo {
    pub name: String,
    pub code: String,
    pub semester: String,
    pub primary_text: String,
    pub supplementary_text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Article {
    pub title: String,
    pub author: String,
    pub year: u16,
    pub filename: Option<String>,
    pub degraded: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IdCompany {
    pub primary: String,
    pub name: String,
    pub rationale: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Connections {
    pub builds_on: Vec<u8>,
    pub leads_to: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeekEntry {
    pub week: u8,
    pub topic: String,
    pub topic_en: String,
    pub subramanyam_chapters: Vec<u8>,
    pub palepu_chapters: Vec<u8>,
    pub articles: Vec<Article>,
    pub id_company: IdCompany,
    pub learning_focus: String,
    pub connections: Connections,
}

impl CourseMap {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let map: CourseMap = serde_json::from_str(&content)?;
        for week in &map.weeks {
            if week.week == 0 || week.week > 14 {
                anyhow::bail!("Invalid week number: {}", week.week);
            }
        }
        Ok(map)
    }
}
