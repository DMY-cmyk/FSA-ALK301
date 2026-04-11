use std::path::{Path, PathBuf};

/// Content types and their directory/filename mappings
pub enum ContentType {
    StudyGuide,
    MainSummary,
    ArticleAnalysis,
    IdExamples,
}

impl ContentType {
    pub fn directory(&self) -> &str {
        match self {
            ContentType::StudyGuide => "Study Guide - Aid",
            ContentType::MainSummary => "Main Summary - Ebook",
            ContentType::ArticleAnalysis => "Articles",
            ContentType::IdExamples => "Indonesian Company Examples",
        }
    }

    pub fn filename(&self) -> &str {
        match self {
            ContentType::StudyGuide => "study-guide.html",
            ContentType::MainSummary => "main-summary.html",
            ContentType::ArticleAnalysis => "article-analysis.html",
            ContentType::IdExamples => "id-examples.html",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            ContentType::StudyGuide => "Study Guide",
            ContentType::MainSummary => "Main Summary",
            ContentType::ArticleAnalysis => "Article Analysis",
            ContentType::IdExamples => "Indonesian Company Examples",
        }
    }

    pub fn slug(&self) -> &str {
        match self {
            ContentType::StudyGuide => "study-guide",
            ContentType::MainSummary => "main-summary",
            ContentType::ArticleAnalysis => "article-analysis",
            ContentType::IdExamples => "id-examples",
        }
    }

    /// Returns all content types that are always expected
    pub fn required() -> Vec<ContentType> {
        vec![
            ContentType::StudyGuide,
            ContentType::MainSummary,
            ContentType::IdExamples,
        ]
    }
}

/// Build the fragment path for a given week and content type
pub fn fragment_path(output_root: &Path, week: u8, content_type: &ContentType) -> PathBuf {
    output_root
        .join(content_type.directory())
        .join(format!("Week {:02}", week))
        .join(content_type.filename())
}

/// Build the assembled output path for a given week and content type
pub fn assembled_path(output_root: &Path, week: u8, content_type: &ContentType) -> PathBuf {
    output_root
        .join("assembled")
        .join(format!("week-{:02}", week))
        .join(content_type.filename())
}

/// Get the outputs directory from project root
pub fn outputs_dir(project_root: &Path) -> PathBuf {
    project_root.join("course-materials").join("outputs")
}

/// Get the configs directory from project root
pub fn configs_dir(project_root: &Path) -> PathBuf {
    project_root.join("Dev Assistant").join("configs")
}

/// Get the templates directory from project root
pub fn templates_dir(project_root: &Path) -> PathBuf {
    project_root.join("Dev Assistant").join("templates")
}
