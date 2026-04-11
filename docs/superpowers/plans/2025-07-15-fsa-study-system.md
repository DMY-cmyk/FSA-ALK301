# FSA Study System Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a complete graduate-level study system for ALK301 with Rust-powered HTML assembly pipeline and 14 weeks of academic content.

**Architecture:** Hybrid Pipeline — a Rust CLI reads `course-map.json` + handcrafted HTML fragments → assembles full pages with navigation, print CSS, and metadata. Visual Companion and Print Bundle are generated from the same course map.

**Tech Stack:** Rust (serde, tera, clap, anyhow), HTML/CSS (vanilla, no frameworks), JSON config.

**Spec:** `docs/superpowers/specs/2025-07-15-fsa-study-system-design.md`

---

## Chunk 1: Rust Project Foundation

### Task 1: Cargo Project Scaffolding

**Files:**
- Create: `Dev Assistant/Cargo.toml`
- Create: `Dev Assistant/src/main.rs`
- Create: `Dev Assistant/src/config/mod.rs`
- Create: `Dev Assistant/src/assembler/mod.rs`
- Create: `Dev Assistant/src/companion/mod.rs`
- Create: `Dev Assistant/src/print_bundle/mod.rs`
- Create: `Dev Assistant/src/validator/mod.rs`
- Create: `Dev Assistant/src/utils/mod.rs`
- Create: `Dev Assistant/README.md`

- [ ] **Step 1: Create Cargo.toml**

```toml
[package]
name = "fsa-dev-assistant"
version = "0.1.0"
edition = "2021"
description = "Build pipeline for FSA ALK301 study system"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tera = "1"
clap = { version = "4", features = ["derive"] }
walkdir = "2"
anyhow = "1"
```

- [ ] **Step 2: Create stub main.rs with clap CLI**

```rust
mod config;
mod assembler;
mod companion;
mod print_bundle;
mod validator;
mod utils;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "fsa-dev-assistant")]
#[command(about = "Build pipeline for FSA ALK301 study system")]
struct Cli {
    /// Path to project root (default: parent of Dev Assistant/)
    #[arg(long, default_value = "..")]
    project_root: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Assemble HTML fragments into full pages
    Build,
    /// Validate that all expected fragment files exist
    Validate,
    /// Generate Visual Companion HTML
    Companion,
    /// Generate Master Print Bundle
    PrintBundle,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let project_root = cli.project_root.canonicalize()?;

    match cli.command {
        Commands::Build => {
            println!("Building assembled pages...");
            assembler::run(&project_root)?;
        }
        Commands::Validate => {
            validator::run(&project_root)?;
        }
        Commands::Companion => {
            println!("Generating Visual Companion...");
            companion::run(&project_root)?;
        }
        Commands::PrintBundle => {
            println!("Generating Print Bundle...");
            print_bundle::run(&project_root)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 3: Create stub modules**

Each module file gets a placeholder `run` function:

`src/config/mod.rs`:
```rust
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
        Ok(map)
    }
}
```

`src/utils/mod.rs`:
```rust
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
```

`src/validator/mod.rs`:
```rust
use crate::config::CourseMap;
use crate::utils::{self, ContentType};
use std::path::Path;

pub fn run(project_root: &Path) -> anyhow::Result<()> {
    let config_path = utils::configs_dir(project_root).join("course-map.json");
    let course_map = CourseMap::load(&config_path)?;
    let output_root = utils::outputs_dir(project_root);

    let mut missing: Vec<String> = Vec::new();
    let mut found: usize = 0;

    for week in &course_map.weeks {
        // Check required content types
        for ct in ContentType::required() {
            let path = utils::fragment_path(&output_root, week.week, &ct);
            if path.exists() {
                found += 1;
            } else {
                missing.push(format!(
                    "Week {:02}/{}/{}",
                    week.week,
                    ct.directory(),
                    ct.filename()
                ));
            }
        }

        // Check article analysis only if week has articles
        if !week.articles.is_empty() {
            let ct = ContentType::ArticleAnalysis;
            let path = utils::fragment_path(&output_root, week.week, &ct);
            if path.exists() {
                found += 1;
            } else {
                missing.push(format!(
                    "Week {:02}/{}/{}",
                    week.week,
                    ct.directory(),
                    ct.filename()
                ));
            }
        }
    }

    if missing.is_empty() {
        println!("✅ All {} fragments found.", found);
        Ok(())
    } else {
        eprintln!("❌ {} fragments found, {} missing:", found, missing.len());
        for m in &missing {
            eprintln!("  - {}", m);
        }
        std::process::exit(1);
    }
}
```

`src/assembler/mod.rs`:
```rust
use std::path::Path;

pub fn run(project_root: &Path) -> anyhow::Result<()> {
    let config_path = crate::utils::configs_dir(project_root).join("course-map.json");
    let course_map = crate::config::CourseMap::load(&config_path)?;
    let output_root = crate::utils::outputs_dir(project_root);
    let templates_dir = crate::utils::templates_dir(project_root);

    // Initialize Tera templates (use forward-slash glob, include *.css for print.css)
    let glob_pattern = format!("{}/{}", templates_dir.display(), "**/*.*").replace('\\', "/");
    let tera = tera::Tera::new(&glob_pattern)?;

    let mut assembled_count = 0;

    for week in &course_map.weeks {
        let content_types = build_content_type_list(week);

        for ct in &content_types {
            let fragment_path = crate::utils::fragment_path(&output_root, week.week, ct);
            if !fragment_path.exists() {
                eprintln!("⚠ Skipping Week {:02}/{} (fragment not found)", week.week, ct.filename());
                continue;
            }

            let body_content = std::fs::read_to_string(&fragment_path)?;
            let out_path = crate::utils::assembled_path(&output_root, week.week, ct);

            // Create output directory
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Build Tera context
            let mut context = tera::Context::new();
            context.insert("page_title", &format!("Week {:02} — {}", week.week, ct.display_name()));
            context.insert("week_number", &week.week);
            context.insert("week_nn", &format!("{:02}", week.week));
            context.insert("week_topic", &week.topic);
            context.insert("week_topic_en", &week.topic_en);
            context.insert("content_type", ct.slug());
            context.insert("subramanyam_chapters", &week.subramanyam_chapters);
            context.insert("palepu_chapters", &week.palepu_chapters);
            context.insert("learning_focus", &week.learning_focus);
            context.insert("body_content", &body_content);
            context.insert("id_company_ticker", &week.id_company.primary);
            context.insert("id_company_name", &week.id_company.name);

            // Build nav items (week_nn = zero-padded string)
            let nav_items: Vec<serde_json::Value> = course_map.weeks.iter().map(|w| {
                serde_json::json!({
                    "week": w.week,
                    "week_nn": format!("{:02}", w.week),
                    "topic": w.topic,
                    "active": w.week == week.week
                })
            }).collect();
            context.insert("nav_items", &nav_items);

            // Prev/next (pass both numeric and zero-padded)
            if week.week > 1 {
                context.insert("prev_week", &(week.week - 1));
                context.insert("prev_week_nn", &format!("{:02}", week.week - 1));
            }
            if week.week < 14 {
                context.insert("next_week", &(week.week + 1));
                context.insert("next_week_nn", &format!("{:02}", week.week + 1));
            }

            // Article metadata
            let articles: Vec<serde_json::Value> = week.articles.iter().map(|a| {
                serde_json::json!({
                    "title": a.title,
                    "author": a.author,
                    "year": a.year
                })
            }).collect();
            context.insert("articles", &articles);

            let rendered = tera.render("base.html", &context)?;
            std::fs::write(&out_path, rendered)?;
            assembled_count += 1;
        }
    }

    println!("✅ Assembled {} pages.", assembled_count);
    Ok(())
}

fn build_content_type_list(week: &crate::config::WeekEntry) -> Vec<crate::utils::ContentType> {
    let mut types = crate::utils::ContentType::required();
    if !week.articles.is_empty() {
        types.push(crate::utils::ContentType::ArticleAnalysis);
    }
    types
}
```

`src/companion/mod.rs`:
```rust
use std::path::Path;

pub fn run(project_root: &Path) -> anyhow::Result<()> {
    let config_path = crate::utils::configs_dir(project_root).join("course-map.json");
    let course_map = crate::config::CourseMap::load(&config_path)?;
    let output_root = crate::utils::outputs_dir(project_root);
    let templates_dir = crate::utils::templates_dir(project_root);

    let glob_pattern = format!("{}/{}", templates_dir.display(), "**/*.*").replace('\\', "/");
    let tera = tera::Tera::new(&glob_pattern)?;

    let mut context = tera::Context::new();
    context.insert("course", &serde_json::to_value(&course_map.course)?);

    let weeks_json: Vec<serde_json::Value> = course_map.weeks.iter().map(|w| {
        serde_json::json!({
            "week": w.week,
            "week_nn": format!("{:02}", w.week),
            "topic": w.topic,
            "topic_en": w.topic_en,
            "subramanyam_chapters": w.subramanyam_chapters,
            "palepu_chapters": w.palepu_chapters,
            "articles": w.articles.iter().map(|a| {
                serde_json::json!({
                    "title": a.title,
                    "author": a.author,
                    "year": a.year,
                    "degraded": a.degraded
                })
            }).collect::<Vec<_>>(),
            "id_company": {
                "primary": &w.id_company.primary,
                "name": &w.id_company.name,
            },
            "learning_focus": w.learning_focus,
            "cluster_color": cluster_color(w.week),
        })
    }).collect();
    context.insert("weeks", &weeks_json);

    let rendered = tera.render("companion.html", &context)?;

    let companion_dir = output_root.join("Visual Companion");
    std::fs::create_dir_all(&companion_dir)?;
    std::fs::write(companion_dir.join("index.html"), rendered)?;

    println!("✅ Visual Companion generated.");
    Ok(())
}

fn cluster_color(week: u8) -> &'static str {
    match week {
        1 | 2 => "#1B3A6B",       // Foundation — navy
        3 => "#2196F3",            // Financing — blue
        4 | 5 => "#4CAF50",        // Investing — green
        6 => "#FF9800",            // Operations — orange
        7 => "#607D8B",            // Review — gray
        8 => "#2196F3",            // Cash flow — blue
        9 => "#FF9800",            // Profitability — orange
        10 => "#FF9800",           // Prospective — orange
        11 => "#2196F3",           // Credit — blue
        12 => "#9C27B0",           // Valuation — purple
        13 => "#9C27B0",           // Comprehensive — purple
        14 => "#607D8B",           // Review — gray
        _ => "#607D8B",
    }
}
```

`src/print_bundle/mod.rs`:
```rust
use std::path::Path;

pub fn run(project_root: &Path) -> anyhow::Result<()> {
    let config_path = crate::utils::configs_dir(project_root).join("course-map.json");
    let course_map = crate::config::CourseMap::load(&config_path)?;
    let output_root = crate::utils::outputs_dir(project_root);
    let templates_dir = crate::utils::templates_dir(project_root);

    let print_css = std::fs::read_to_string(templates_dir.join("print.css"))
        .unwrap_or_default();

    let mut all_content = String::new();

    // Table of contents
    all_content.push_str("<div class=\"toc\">\n<h1>Table of Contents</h1>\n<ul>\n");
    for week in &course_map.weeks {
        all_content.push_str(&format!(
            "<li><a href=\"#week-{:02}\">Week {:02}: {}</a></li>\n",
            week.week, week.week, week.topic_en
        ));
    }
    all_content.push_str("</ul>\n</div>\n<div class=\"page-break\"></div>\n");

    // Concatenate assembled pages
    for week in &course_map.weeks {
        all_content.push_str(&format!(
            "<div id=\"week-{:02}\" class=\"week-section\">\n<h1>Week {:02}: {}</h1>\n<p class=\"topic-id\">{}</p>\n",
            week.week, week.week, week.topic_en, week.topic
        ));

        let content_types = vec![
            crate::utils::ContentType::StudyGuide,
            crate::utils::ContentType::MainSummary,
            crate::utils::ContentType::ArticleAnalysis,
            crate::utils::ContentType::IdExamples,
        ];

        for ct in &content_types {
            let assembled = crate::utils::assembled_path(&output_root, week.week, ct);
            if assembled.exists() {
                // Extract body content from assembled HTML (between <main> tags)
                let html = std::fs::read_to_string(&assembled)?;
                if let (Some(start), Some(end)) = (html.find("<main>"), html.find("</main>")) {
                    let body = &html[start + 6..end];
                    all_content.push_str(&format!(
                        "<div class=\"content-section {}\">\n<h2>{}</h2>\n{}\n</div>\n",
                        ct.slug(), ct.display_name(), body
                    ));
                }
            }
        }

        all_content.push_str("</div>\n<div class=\"page-break\"></div>\n");
    }

    // Write final bundle
    let bundle = format!(
        "<!DOCTYPE html>\n<html lang=\"id\">\n<head>\n<meta charset=\"UTF-8\">\n<title>ALK301 Course Companion — Master Print Bundle</title>\n<style>\n{}\n</style>\n</head>\n<body>\n{}\n</body>\n</html>",
        print_css, all_content
    );

    let bundle_dir = output_root.join("Master Print Bundle");
    std::fs::create_dir_all(&bundle_dir)?;
    std::fs::write(bundle_dir.join("course-companion.html"), bundle)?;

    println!("✅ Master Print Bundle generated.");
    Ok(())
}
```

- [ ] **Step 4: Add display_name() and slug() to ContentType**

In `src/utils/mod.rs`, add these methods to the `ContentType` impl:

```rust
impl ContentType {
    // ... existing methods ...

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
}
```

- [ ] **Step 5: Verify the project compiles**

Run from `Dev Assistant/`:
```
cargo build
```
Expected: Successful compilation with no errors (warnings about unused variables are OK at this stage).

- [ ] **Step 6: Commit**

```
git add "Dev Assistant/"
git commit -m "feat: scaffold Rust Dev Assistant project with all modules

- CLI with 4 subcommands: build, validate, companion, print-bundle
- Config module with CourseMap serde structs
- Validator checks fragment existence
- Assembler renders fragments through Tera templates
- Companion and PrintBundle generators
- Utils with path resolution helpers

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 2: Create course-map.json

**Files:**
- Create: `Dev Assistant/configs/course-map.json`

- [ ] **Step 1: Write the complete 14-week course map**

```json
{
  "course": {
    "name": "Analisis Laporan Keuangan dan Valuasi Sekuritas",
    "code": "ALK301",
    "semester": "1 / 2025-2026",
    "primary_text": "Subramanyam, K.R. Financial Statement Analysis, 11th ed. McGraw-Hill.",
    "supplementary_text": "Palepu, Healy, Peek. Business Analysis and Valuation: IFRS Edition."
  },
  "weeks": [
    {
      "week": 1,
      "topic": "Silabus dan pengenalan ALK",
      "topic_en": "Syllabus and Introduction to FSA",
      "subramanyam_chapters": [1],
      "palepu_chapters": [1],
      "articles": [],
      "id_company": {
        "primary": "UNVR",
        "name": "Unilever Indonesia Tbk",
        "rationale": "Clean financials, continues Ch.1 baseline"
      },
      "learning_focus": "Framework for financial statement analysis; role of accounting information in capital markets",
      "connections": { "builds_on": [], "leads_to": [2] }
    },
    {
      "week": 2,
      "topic": "Analisis dan pelaporan keuangan",
      "topic_en": "Financial Reporting and Analysis Environment",
      "subramanyam_chapters": [1, 2],
      "palepu_chapters": [1, 2],
      "articles": [],
      "id_company": {
        "primary": "UNVR",
        "name": "Unilever Indonesia Tbk",
        "rationale": "Consistent with Week 1; shows reporting environment"
      },
      "learning_focus": "Financial reporting environment; accrual accounting; earnings quality; regulatory framework",
      "connections": { "builds_on": [1], "leads_to": [3] }
    },
    {
      "week": 3,
      "topic": "Analisis pendanaan (financing)",
      "topic_en": "Financing Activities Analysis",
      "subramanyam_chapters": [3],
      "palepu_chapters": [],
      "articles": [
        {
          "title": "Financial Ratios, Discriminant Analysis and the Prediction of Corporate Bankruptcy",
          "author": "Altman, E.I.",
          "year": 1968,
          "filename": "Altman_1968_Financial_Ratios_Discriminant_Analysis_Bankruptcy.pdf",
          "degraded": false
        }
      ],
      "id_company": {
        "primary": "BBCA",
        "name": "Bank Central Asia Tbk",
        "rationale": "Rich debt/equity structure, bond issuance, complex financing"
      },
      "learning_focus": "Debt vs equity financing; leases; off-balance-sheet items; capital structure analysis",
      "connections": { "builds_on": [2], "leads_to": [4] }
    },
    {
      "week": 4,
      "topic": "Analisis investasi",
      "topic_en": "Investing Activities Analysis",
      "subramanyam_chapters": [4],
      "palepu_chapters": [],
      "articles": [],
      "id_company": {
        "primary": "ASII",
        "name": "Astra International Tbk",
        "rationale": "Major capex, diverse asset acquisition patterns"
      },
      "learning_focus": "PP&E analysis; intangible assets; depreciation methods; asset impairment; capitalization vs expensing",
      "connections": { "builds_on": [3], "leads_to": [5] }
    },
    {
      "week": 5,
      "topic": "Analisis investasi: Intercorporate investment",
      "topic_en": "Intercorporate Investments Analysis",
      "subramanyam_chapters": [5],
      "palepu_chapters": [],
      "articles": [
        {
          "title": "Sector-wise Analysis of Working Capital Management and Firm Performance in Manufacturing Sector of Pakistan",
          "author": "Raheman, A.; Qayyum, A.; Afza, T.",
          "year": 2010,
          "filename": null,
          "degraded": true
        }
      ],
      "id_company": {
        "primary": "ASII",
        "name": "Astra International Tbk + United Tractors (UNTR)",
        "rationale": "Holding/subsidiary consolidation structure"
      },
      "learning_focus": "Equity method; consolidation; minority interests; goodwill; intercorporate investment accounting",
      "connections": { "builds_on": [4], "leads_to": [6] }
    },
    {
      "week": 6,
      "topic": "Analisis aktivitas operasi",
      "topic_en": "Operating Activities Analysis",
      "subramanyam_chapters": [6],
      "palepu_chapters": [4],
      "articles": [],
      "id_company": {
        "primary": "TLKM",
        "name": "Telkom Indonesia Tbk",
        "rationale": "Revenue recognition complexity, operating expense structure"
      },
      "learning_focus": "Revenue recognition; operating expenses; income measurement; non-recurring items; earnings persistence",
      "connections": { "builds_on": [5], "leads_to": [7] }
    },
    {
      "week": 7,
      "topic": "Review bahan",
      "topic_en": "Pre-Midterm Review: Synthesis and Article Discussion",
      "subramanyam_chapters": [],
      "palepu_chapters": [],
      "articles": [
        {
          "title": "Earnings Management During Import Relief Investigations",
          "author": "Jones, J.J.",
          "year": 1991,
          "filename": "Jones_1991_Earnings_Management_Import_Relief.pdf",
          "degraded": false
        },
        {
          "title": "The Existence of Inter-industry Convergence in Financial Ratios: Evidence from Turkey",
          "author": "Acaravci, S.K.",
          "year": 2007,
          "filename": "Acaravci_2007_Interindustry_Convergence_Financial_Ratios_Turkey.pdf",
          "degraded": false
        }
      ],
      "id_company": {
        "primary": "Various",
        "name": "Various IDX Cases",
        "rationale": "Earnings management examples in Indonesian context"
      },
      "learning_focus": "Synthesis of Weeks 1-6; earnings management detection; cross-industry ratio analysis; exam preparation",
      "connections": { "builds_on": [1, 2, 3, 4, 5, 6], "leads_to": [8] }
    },
    {
      "week": 8,
      "topic": "Analisis aliran kas",
      "topic_en": "Cash Flow Analysis",
      "subramanyam_chapters": [7],
      "palepu_chapters": [],
      "articles": [],
      "id_company": {
        "primary": "ICBP",
        "name": "Indofood CBP Sukses Makmur Tbk",
        "rationale": "Complex cash flow from operations/investing patterns"
      },
      "learning_focus": "Statement of cash flows; direct vs indirect method; free cash flow; cash flow quality analysis",
      "connections": { "builds_on": [7], "leads_to": [9] }
    },
    {
      "week": 9,
      "topic": "Analisis profitabilitas dan return on invested capital",
      "topic_en": "Profitability Analysis and ROIC",
      "subramanyam_chapters": [8],
      "palepu_chapters": [5, 6],
      "articles": [],
      "id_company": {
        "primary": "BBRI",
        "name": "Bank Rakyat Indonesia Tbk",
        "rationale": "ROE/ROIC decomposition, DuPont analysis application"
      },
      "learning_focus": "DuPont decomposition; ROE; ROIC; profit margins; asset turnover; financial leverage analysis",
      "connections": { "builds_on": [8], "leads_to": [10] }
    },
    {
      "week": 10,
      "topic": "Analisis prospektif",
      "topic_en": "Prospective Analysis and Forecasting",
      "subramanyam_chapters": [9],
      "palepu_chapters": [7],
      "articles": [
        {
          "title": "Cash Flow Ratios as Indicators of Corporate Failure",
          "author": "Murty, A.V.N.; Misra, D.P.",
          "year": 2004,
          "filename": null,
          "degraded": true
        }
      ],
      "id_company": {
        "primary": "GOTO",
        "name": "GoTo Gojek Tokopedia Tbk",
        "rationale": "Growth projections, prospective valuation challenges"
      },
      "learning_focus": "Earnings forecasting; pro forma analysis; scenario modeling; forecasting cash flows and growth",
      "connections": { "builds_on": [9], "leads_to": [11] }
    },
    {
      "week": 11,
      "topic": "Analisis kredit",
      "topic_en": "Credit Analysis",
      "subramanyam_chapters": [10],
      "palepu_chapters": [],
      "articles": [],
      "id_company": {
        "primary": "BUMI",
        "name": "Bumi Resources Tbk",
        "rationale": "Financial distress history, credit risk indicators"
      },
      "learning_focus": "Liquidity analysis; solvency ratios; credit scoring; bankruptcy prediction; bond ratings",
      "connections": { "builds_on": [10], "leads_to": [12] }
    },
    {
      "week": 12,
      "topic": "Analisis ekuitas dan penilaian",
      "topic_en": "Equity Analysis and Valuation",
      "subramanyam_chapters": [11],
      "palepu_chapters": [8, 9],
      "articles": [
        {
          "title": "The Effect of Financial Ratios on Returns from Initial Public Offerings: An Application of Principal Component Analysis",
          "author": "Cheng, Min-Tsung",
          "year": 2006,
          "filename": null,
          "degraded": true
        }
      ],
      "id_company": {
        "primary": "BBCA",
        "name": "Bank Central Asia Tbk",
        "rationale": "P/E, P/B, residual income model application"
      },
      "learning_focus": "Residual income valuation; DDM; DCF; P/E and P/B multiples; equity risk premium; intrinsic value",
      "connections": { "builds_on": [11], "leads_to": [13] }
    },
    {
      "week": 13,
      "topic": "Comprehensive case: Applying financial statement analysis",
      "topic_en": "Comprehensive Case Study",
      "subramanyam_chapters": [],
      "palepu_chapters": [10],
      "articles": [],
      "id_company": {
        "primary": "HMSP",
        "name": "HM Sampoerna Tbk",
        "rationale": "Full FSA case study — well-documented, diversified operations"
      },
      "learning_focus": "Integrating all FSA techniques into a comprehensive company analysis; case methodology",
      "connections": { "builds_on": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], "leads_to": [14] }
    },
    {
      "week": 14,
      "topic": "Review bahan",
      "topic_en": "Pre-Final Review: Bankruptcy Prediction and Synthesis",
      "subramanyam_chapters": [],
      "palepu_chapters": [],
      "articles": [
        {
          "title": "A Discriminant Analysis of Predictors of Business Failure",
          "author": "Deakin, E.B.",
          "year": 1972,
          "filename": "Deakin_1972_Discriminant_Analysis_Predictors_Business_Failure.pdf",
          "degraded": false
        },
        {
          "title": "Financial Ratios and the Probabilistic Prediction of Bankruptcy",
          "author": "Ohlson, J.A.",
          "year": 1980,
          "filename": "Ohlson_1980_Financial_Ratios_Probabilistic_Prediction_Bankruptcy.pdf",
          "degraded": false
        }
      ],
      "id_company": {
        "primary": "Various",
        "name": "IDX Delisted Companies",
        "rationale": "Bankruptcy prediction in Indonesian corporate context"
      },
      "learning_focus": "Bankruptcy prediction models (Altman, Ohlson, Deakin); synthesis of full course; final exam preparation",
      "connections": { "builds_on": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13], "leads_to": [] }
    }
  ]
}
```

- [ ] **Step 2: Verify JSON parses correctly**

Run from `Dev Assistant/`:
```
cargo run -- validate
```
Expected: Should attempt to load course-map.json and report missing fragments (since no content exists yet). The important thing is it doesn't crash on JSON parsing.

- [ ] **Step 3: Commit**

```
git add "Dev Assistant/configs/course-map.json"
git commit -m "feat: add complete 14-week course-map.json

Maps all weeks to Subramanyam chapters, Palepu chapters, articles,
Indonesian company examples, learning focus, and week connections.

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 3: Create HTML Templates and Print CSS

**Files:**
- Create: `Dev Assistant/templates/base.html`
- Create: `Dev Assistant/templates/nav.html`
- Create: `Dev Assistant/templates/print.css`

- [ ] **Step 1: Create print.css**

Extract and extend from existing Chapter 1 guide. File: `Dev Assistant/templates/print.css`

```css
/* === FSA ALK301 Study System — Base + Print Stylesheet === */

/* --- CSS Variables --- */
:root {
  --navy: #1B3A6B;
  --gold: #C9A84C;
  --gray: #F5F5F5;
  --lblue: #E8EEF7;
  --colgbg: #FFF8E8;
  --idbg: #FFF5F0;
  --idborder: #E8A0A0;
  --idaccent: #C41E3A;
}

/* --- Reset --- */
*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
body {
  font-family: 'Segoe UI', Arial, sans-serif;
  font-size: 11pt;
  color: #222;
  line-height: 1.45;
  background: #fff;
}

/* --- Layout --- */
.page-wrapper { max-width: 900px; margin: 0 auto; padding: 20px 30px; }
nav.sidebar { display: none; } /* Hidden by default in print; shown via JS in screen */

/* --- Metadata Banner --- */
.week-banner {
  background: var(--navy);
  color: #fff;
  padding: 12px 18px;
  border-radius: 6px;
  margin-bottom: 16px;
}
.week-banner h1 { font-size: 16pt; margin-bottom: 4px; }
.week-banner .meta { font-size: 9.5pt; opacity: 0.85; }
.week-banner .meta span { margin-right: 16px; }

/* --- Headings --- */
h2 { color: var(--navy); font-size: 14pt; border-bottom: 2.5px solid var(--navy); padding-bottom: 5px; margin: 14px 0 8px; }
h3 { color: var(--navy); font-size: 12pt; margin: 10px 0 5px; }
h4 { color: var(--navy); font-size: 11pt; margin: 8px 0 4px; }
p { margin: 4px 0; }
ul, ol { padding-left: 22px; margin: 4px 0; }
li { margin: 3px 0; }
strong { color: var(--navy); }

/* --- Component Boxes (matching Ch.1 baseline) --- */
.def { background: var(--gray); border-left: 4px solid var(--navy); padding: 8px 12px; margin: 6px 0; border-radius: 0 5px 5px 0; }
.den { font-weight: bold; color: var(--navy); font-size: 11pt; }
.did { color: #444; margin-top: 3px; font-size: 10.5pt; }

.fbox { background: var(--gray); border: 1px solid #ccc; border-top: 3px solid var(--navy); padding: 8px 12px; margin: 6px 0; border-radius: 0 0 5px 5px; }
.ftitle { font-weight: bold; color: var(--navy); font-size: 10pt; margin-bottom: 5px; text-transform: uppercase; letter-spacing: .5px; }
.ff { font-family: 'Courier New', monospace; font-size: 10pt; margin: 3px 0; }
.fw { font-size: 9.5pt; color: #555; margin-top: 5px; }

.cg { background: var(--colgbg); border: 1px solid #e8d5a0; border-left: 4px solid var(--gold); padding: 8px 12px; margin: 6px 0; border-radius: 0 5px 5px 0; }
.cgt { font-weight: bold; color: #8B6914; font-size: 9.5pt; text-transform: uppercase; letter-spacing: .5px; margin-bottom: 4px; }

.id-box { background: var(--idbg); border: 1px solid var(--idborder); border-left: 4px solid var(--idaccent); padding: 8px 12px; margin: 6px 0; border-radius: 0 5px 5px 0; }
.idt { font-weight: bold; color: #8B1A2B; font-size: 9.5pt; text-transform: uppercase; letter-spacing: .5px; margin-bottom: 4px; }

.ibox { background: var(--lblue); border: 1px solid var(--navy); padding: 8px 12px; margin: 6px 0; border-radius: 5px; }
.ibt { font-weight: bold; color: var(--navy); margin-bottom: 4px; }

.qb { background: var(--gray); border-radius: 5px; padding: 8px 12px; margin: 5px 0; }
.qn { font-weight: bold; color: var(--navy); }

.exb { border: 1px solid #ddd; border-top: 3px solid var(--navy); border-radius: 0 0 5px 5px; padding: 9px 12px; margin: 7px 0; }
.ext { font-weight: bold; color: var(--navy); font-size: 11pt; margin-bottom: 5px; }

.cc { background: var(--gray); border-top: 3px solid var(--navy); border-radius: 0 0 5px 5px; padding: 7px 8px; }
.cc h4 { color: var(--navy); font-size: 10pt; margin-bottom: 5px; border-bottom: 1px solid #ccc; padding-bottom: 2px; }

.cgrid { display: grid; grid-template-columns: 1fr 1fr; gap: 6px; }

/* --- Tables --- */
table { width: 100%; border-collapse: collapse; margin: 6px 0; font-size: 10pt; }
thead tr { background: var(--navy); color: #fff; }
th { padding: 5px 7px; text-align: left; font-weight: 600; }
tbody tr:nth-child(even) { background: var(--gray); }
tbody tr:nth-child(odd) { background: #fff; }
td { padding: 4px 7px; border-bottom: 1px solid #e5e5e5; vertical-align: top; }

/* --- Vocabulary Grid --- */
.vgrid { display: grid; grid-template-columns: 1fr 1fr; font-size: 9.5pt; }
.ven { font-weight: bold; color: var(--navy); padding: 3px 6px; border-bottom: 1px solid #eee; }
.vid { font-style: italic; color: #555; padding: 3px 6px; border-bottom: 1px solid #eee; }

/* --- Degraded Article Warning --- */
.degraded-warning {
  background: #FFF3CD;
  border: 1px solid #FFC107;
  border-left: 4px solid #FF9800;
  padding: 8px 12px;
  margin: 6px 0;
  border-radius: 0 5px 5px 0;
  font-size: 10pt;
}

/* --- Prev/Next Navigation --- */
.page-nav {
  display: flex;
  justify-content: space-between;
  margin-top: 24px;
  padding-top: 12px;
  border-top: 2px solid var(--navy);
  font-size: 10pt;
}
.page-nav a { color: var(--navy); text-decoration: none; font-weight: bold; }
.page-nav a:hover { text-decoration: underline; }

/* === PRINT STYLES === */
@page {
  size: A4;
  margin: 1.8cm 2cm 2cm 2cm;
}

@media print {
  .no-print, nav.sidebar, .page-nav { display: none; }
  .page-break, .pb { page-break-before: always; }
  .nb { page-break-inside: avoid; }
  h2, h3, h4 { page-break-after: avoid; }
  .def, .cg, .id-box, .fbox, .ibox, .exb, .qb, .cc { page-break-inside: avoid; }
  table { page-break-inside: avoid; }
  body { font-size: 10pt; line-height: 1.5; }
  .week-banner { -webkit-print-color-adjust: exact; print-color-adjust: exact; }
  thead tr { -webkit-print-color-adjust: exact; print-color-adjust: exact; }
}

/* === PRINT BUNDLE SPECIFIC === */
.toc { page-break-after: always; }
.toc h1 { color: var(--navy); font-size: 18pt; margin-bottom: 12px; }
.toc ul { list-style: none; padding: 0; }
.toc li { padding: 4px 0; border-bottom: 1px solid #eee; font-size: 11pt; }
.toc a { color: var(--navy); text-decoration: none; }
.week-section { page-break-before: always; }
.content-section { margin-bottom: 16px; }
```

- [ ] **Step 2: Create nav.html template**

File: `Dev Assistant/templates/nav.html`

```html
<ul class="nav-list">
{% for item in nav_items %}
  <li class="{% if item.active %}active{% endif %}">
    <a href="../week-{{ item.week_nn }}/{{ content_type }}.html">
      Week {{ item.week_nn }}: {{ item.topic }}
    </a>
  </li>
{% endfor %}
</ul>
```

- [ ] **Step 3: Create base.html template**

File: `Dev Assistant/templates/base.html`

```html
<!DOCTYPE html>
<html lang="id">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{{ page_title }}</title>
<style>
{% include "print.css" %}
</style>
</head>
<body>
<div class="page-wrapper">

<!-- Week Metadata Banner -->
<div class="week-banner nb">
  <h1>Week {{ week_nn }}: {{ week_topic_en }}</h1>
  <div class="meta">
    <span>📚 {{ week_topic }}</span>
    {% if subramanyam_chapters | length > 0 %}
    <span>📖 Subramanyam Ch. {{ subramanyam_chapters | join(sep=", ") }}</span>
    {% endif %}
    {% if palepu_chapters | length > 0 %}
    <span>📗 Palepu Ch. {{ palepu_chapters | join(sep=", ") }}</span>
    {% endif %}
    {% if articles | length > 0 %}
    <span>📄 {{ articles | length }} article(s)</span>
    {% endif %}
    <span>🏢 {{ id_company_ticker }} — {{ id_company_name }}</span>
  </div>
  <div class="meta" style="margin-top:4px;">
    <span>🎯 {{ learning_focus }}</span>
  </div>
</div>

<!-- Body Content (injected fragment) -->
<main>
{{ body_content | safe }}
</main>

<!-- Prev/Next Navigation -->
<div class="page-nav no-print">
  {% if prev_week %}
  <a href="../week-{{ prev_week_nn }}/{{ content_type }}.html">← Week {{ prev_week_nn }}</a>
  {% else %}
  <span></span>
  {% endif %}
  {% if next_week %}
  <a href="../week-{{ next_week_nn }}/{{ content_type }}.html">Week {{ next_week_nn }} →</a>
  {% else %}
  <span></span>
  {% endif %}
</div>

</div><!-- .page-wrapper -->
</body>
</html>
```

- [ ] **Step 4: Verify templates compile with Tera**

Run from `Dev Assistant/`:
```
cargo build
```
Expected: Compiles successfully. (Template rendering will be tested when we have actual fragments.)

- [ ] **Step 5: Commit**

```
git add "Dev Assistant/templates/"
git commit -m "feat: add HTML templates and print CSS

- base.html: page wrapper with week banner, nav, content injection
- nav.html: sidebar navigation partial
- print.css: complete stylesheet matching Ch.1 baseline + print rules

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 4: Create Visual Companion Template

**Files:**
- Create: `Dev Assistant/templates/companion.html`

- [ ] **Step 1: Write companion.html template**

This is a self-contained interactive HTML page generated by Rust from course-map.json data.

File: `Dev Assistant/templates/companion.html`

```html
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>ALK301 Visual Companion — Course Learning Map</title>
<style>
:root{--navy:#1B3A6B;--gold:#C9A84C;--gray:#F5F5F5;--lblue:#E8EEF7}
*,*::before,*::after{box-sizing:border-box;margin:0;padding:0}
body{font-family:'Segoe UI',Arial,sans-serif;color:#222;background:#fff}
.header{background:var(--navy);color:#fff;padding:20px 30px;text-align:center}
.header h1{font-size:20pt;margin-bottom:4px}
.header p{opacity:.8;font-size:11pt}
.tabs{display:flex;border-bottom:3px solid var(--navy);background:var(--gray)}
.tab{padding:10px 20px;cursor:pointer;font-weight:bold;color:var(--navy);border:none;background:none;font-size:10.5pt}
.tab.active{background:var(--navy);color:#fff;border-radius:5px 5px 0 0}
.tab:hover:not(.active){background:var(--lblue)}
.view{display:none;padding:20px 30px;max-width:1100px;margin:0 auto}
.view.active{display:block}

/* Roadmap */
.timeline{display:flex;flex-direction:column;gap:8px}
.week-card{display:flex;align-items:flex-start;gap:12px;padding:10px 14px;border-radius:6px;border-left:5px solid;background:#fff;box-shadow:0 1px 3px rgba(0,0,0,.08)}
.week-num{font-weight:bold;color:#fff;border-radius:50%;width:32px;height:32px;display:flex;align-items:center;justify-content:center;font-size:10pt;flex-shrink:0}
.week-info{flex:1}
.week-info h3{font-size:11pt;color:var(--navy);margin-bottom:2px}
.week-info .details{font-size:9.5pt;color:#555}
.week-info .details span{margin-right:10px}
.midterm-bar,.final-bar{background:var(--gold);color:#fff;text-align:center;padding:8px;font-weight:bold;border-radius:4px;font-size:11pt}

/* Chapter Map */
.ch-grid{display:grid;grid-template-columns:80px 1fr 1fr;gap:2px;font-size:9.5pt}
.ch-header{background:var(--navy);color:#fff;padding:6px 8px;font-weight:bold}
.ch-cell{padding:5px 8px;border-bottom:1px solid #eee}
.ch-week{background:var(--gray);font-weight:bold;color:var(--navy)}

/* Article Network */
.article-card{background:#fff;border:1px solid #ddd;border-left:4px solid var(--gold);padding:10px 14px;margin:8px 0;border-radius:0 6px 6px 0}
.article-card h4{color:var(--navy);margin-bottom:3px;font-size:10.5pt}
.article-card .meta{font-size:9pt;color:#777}
.article-card .connection{font-size:9.5pt;color:#444;margin-top:4px}
.degraded-badge{background:#FFF3CD;color:#795548;padding:2px 8px;border-radius:10px;font-size:8pt;font-weight:bold}

/* Indonesian Examples */
.id-card{background:#FFF5F0;border:1px solid #E8A0A0;border-left:4px solid #C41E3A;padding:10px 14px;margin:8px 0;border-radius:0 6px 6px 0}
.id-card h4{color:#8B1A2B;margin-bottom:3px;font-size:10.5pt}
.id-card .ticker{font-weight:bold;color:#C41E3A}

/* Collapsible */
.collapsible{cursor:pointer;user-select:none}
.collapsible::after{content:" ▸";font-size:10pt}
.collapsible.open::after{content:" ▾"}
.collapse-content{display:none;padding-left:12px;margin-top:4px}
.collapse-content.show{display:block}

@media print{
  .tabs{display:none}
  .view{display:block!important;page-break-before:always}
  .header{-webkit-print-color-adjust:exact;print-color-adjust:exact}
}
</style>
</head>
<body>

<div class="header">
  <h1>📊 ALK301 — Visual Companion</h1>
  <p>{{ course.name }} · {{ course.semester }}</p>
</div>

<div class="tabs">
  <button class="tab active" onclick="showView('roadmap')">Course Roadmap</button>
  <button class="tab" onclick="showView('chapters')">Chapter Map</button>
  <button class="tab" onclick="showView('articles')">Article Network</button>
  <button class="tab" onclick="showView('indonesian')">Indonesian Examples</button>
</div>

<!-- VIEW 1: Course Roadmap -->
<div id="roadmap" class="view active">
  <h2 style="color:var(--navy);margin-bottom:12px">Course Roadmap — 14 Weeks</h2>
  <div class="timeline">
    {% for week in weeks %}
    {% if week.week == 8 %}
    <div class="midterm-bar">📝 Ujian Tengah Semester (Midterm Exam)</div>
    {% endif %}
    <div class="week-card" style="border-left-color:{{ week.cluster_color }}">
      <div class="week-num" style="background:{{ week.cluster_color }}">{{ week.week }}</div>
      <div class="week-info">
        <h3>{{ week.topic_en }}</h3>
        <div class="details">
          <span>📚 {{ week.topic }}</span>
          {% if week.subramanyam_chapters | length > 0 %}<span>📖 Sub. Ch. {{ week.subramanyam_chapters | join(sep=", ") }}</span>{% endif %}
          {% if week.palepu_chapters | length > 0 %}<span>📗 Palepu Ch. {{ week.palepu_chapters | join(sep=", ") }}</span>{% endif %}
          {% if week.articles | length > 0 %}<span>📄 {{ week.articles | length }} article(s)</span>{% endif %}
          <span>🏢 {{ week.id_company.primary }}</span>
        </div>
        <div class="details" style="margin-top:2px"><span>🎯 {{ week.learning_focus }}</span></div>
      </div>
    </div>
    {% endfor %}
    <div class="final-bar">📝 Ujian Akhir Semester (Final Exam)</div>
  </div>
</div>

<!-- VIEW 2: Chapter Map -->
<div id="chapters" class="view">
  <h2 style="color:var(--navy);margin-bottom:12px">Chapter-to-Week Mapping</h2>
  <div class="ch-grid">
    <div class="ch-header">Week</div>
    <div class="ch-header">Subramanyam</div>
    <div class="ch-header">Palepu-Healy-Peek</div>
    {% for week in weeks %}
    <div class="ch-cell ch-week">{{ week.week }}</div>
    <div class="ch-cell">{% if week.subramanyam_chapters | length > 0 %}Ch. {{ week.subramanyam_chapters | join(sep=", ") }}{% else %}—{% endif %}</div>
    <div class="ch-cell">{% if week.palepu_chapters | length > 0 %}Ch. {{ week.palepu_chapters | join(sep=", ") }}{% else %}—{% endif %}</div>
    {% endfor %}
  </div>
</div>

<!-- VIEW 3: Article Network -->
<div id="articles" class="view">
  <h2 style="color:var(--navy);margin-bottom:12px">Article Assignments</h2>
  {% for week in weeks %}
  {% if week.articles | length > 0 %}
  <h3 style="color:var(--navy);margin:12px 0 4px">Week {{ week.week }}: {{ week.topic_en }}</h3>
  {% for article in week.articles %}
  <div class="article-card">
    <h4>{{ article.title }} {% if article.degraded %}<span class="degraded-badge">⚠ Limited Access</span>{% endif %}</h4>
    <div class="meta">{{ article.author }} ({{ article.year }})</div>
    <div class="connection">
      🔗 Connected to: {{ week.topic_en }}
      {% if week.subramanyam_chapters | length > 0 %} · Subramanyam Ch. {{ week.subramanyam_chapters | join(sep=", ") }}{% endif %}
    </div>
  </div>
  {% endfor %}
  {% endif %}
  {% endfor %}
</div>

<!-- VIEW 4: Indonesian Examples -->
<div id="indonesian" class="view">
  <h2 style="color:var(--navy);margin-bottom:12px">Indonesian Company Examples</h2>
  {% for week in weeks %}
  <div class="id-card">
    <h4>Week {{ week.week }}: {{ week.topic_en }}</h4>
    <div><span class="ticker">{{ week.id_company.primary }}</span> — {{ week.id_company.name }}</div>
  </div>
  {% endfor %}
</div>

<script>
function showView(id) {
  document.querySelectorAll('.view').forEach(v => v.classList.remove('active'));
  document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
  document.getElementById(id).classList.add('active');
  event.target.classList.add('active');
}
document.querySelectorAll('.collapsible').forEach(el => {
  el.addEventListener('click', () => {
    el.classList.toggle('open');
    el.nextElementSibling.classList.toggle('show');
  });
});
</script>
</body>
</html>
```

- [ ] **Step 2: Verify build still compiles**

Run: `cargo build` from `Dev Assistant/`
Expected: Compiles successfully.

- [ ] **Step 3: Commit**

```
git add "Dev Assistant/templates/companion.html"
git commit -m "feat: add Visual Companion HTML template

Interactive single-page with 4 views: Course Roadmap, Chapter Map,
Article Network, Indonesian Examples. Vanilla JS, print-friendly.

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 5: Create Output Directory Scaffold

**Files:**
- Create directory tree for all 14 weeks across all content types

- [ ] **Step 1: Create all output directories**

Run from project root:
```powershell
$base = "course-materials\outputs"
$types = @("Study Guide - Aid", "Main Summary - Ebook", "Articles", "Indonesian Company Examples")
1..14 | ForEach-Object {
    $week = "Week {0:D2}" -f $_
    foreach ($type in $types) {
        New-Item -ItemType Directory -Path "$base\$type\$week" -Force | Out-Null
    }
}
# Also create assembled, Visual Companion, Master Print Bundle
New-Item -ItemType Directory -Path "$base\assembled" -Force | Out-Null
New-Item -ItemType Directory -Path "$base\Visual Companion" -Force | Out-Null
New-Item -ItemType Directory -Path "$base\Master Print Bundle" -Force | Out-Null

# Add .gitkeep files so Git tracks these empty directories
Get-ChildItem "$base" -Recurse -Directory | ForEach-Object {
    $gk = Join-Path $_.FullName ".gitkeep"
    if (-not (Test-Path $gk)) { New-Item -ItemType File -Path $gk -Force | Out-Null }
}
```

- [ ] **Step 2: Verify directory structure**

Run: `Get-ChildItem "course-materials\outputs" -Recurse -Directory | Select-Object FullName`
Expected: 60+ directories covering all weeks and types.

- [ ] **Step 3: Commit**

```
git add "course-materials/outputs/"
git commit -m "feat: scaffold output directory structure for all 14 weeks

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 6: End-to-End Smoke Test

- [ ] **Step 1: Create a minimal test fragment for Week 01**

Create `course-materials/outputs/Study Guide - Aid/Week 01/study-guide.html`:
```html
<h2>Week 01 — Study Guide (Test Fragment)</h2>
<p>This is a placeholder to verify the assembly pipeline works.</p>
```

- [ ] **Step 2: Run validate**

Run from `Dev Assistant/`:
```
cargo run -- validate
```
Expected: Reports Week 01 study guide as found, all other fragments as missing. Exit code 1.

- [ ] **Step 3: Run build**

Run from `Dev Assistant/`:
```
cargo run -- build
```
Expected: Assembles Week 01 study guide into `course-materials/outputs/assembled/week-01/study-guide.html`. Skips all other missing fragments with warnings.

- [ ] **Step 4: Run companion**

Run from `Dev Assistant/`:
```
cargo run -- companion
```
Expected: Generates `course-materials/outputs/Visual Companion/index.html` with all 14 weeks listed.

- [ ] **Step 5: Verify assembled HTML is valid**

Open `course-materials/outputs/assembled/week-01/study-guide.html` in a browser (or check it has proper HTML structure with head/body/style tags).

- [ ] **Step 6: Remove test fragment and commit**

Remove the placeholder and generated artifacts:
```
Remove-Item "course-materials\outputs\Study Guide - Aid\Week 01\study-guide.html"
Remove-Item "course-materials\outputs\assembled\week-01" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item "course-materials\outputs\Visual Companion\index.html" -Force -ErrorAction SilentlyContinue
```

```
git add -A -- "course-materials/outputs/"
git commit -m "test: verify end-to-end assembly pipeline works

Smoke-tested validate, build, and companion commands.
Removed test fragment after verification.

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

## Chunk 2: Content Generation — Weeks 1-7

For each week, the content generation task follows this pattern:
1. Create study-guide.html fragment
2. Create main-summary.html fragment
3. Create article-analysis.html fragment (if articles assigned)
4. Create id-examples.html fragment + sources.md
5. Run `cargo run -- validate` to confirm fragments exist

**Important content guidelines:**
- All fragments are HTML **without** `<html>`, `<head>`, `<body>` wrappers — just content using the CSS classes from print.css
- Use the component classes: `.def`, `.cg`, `.id-box`, `.fbox`, `.ibox`, `.exb`, `.qb`, `.cc`, `.cgrid`, `.vgrid`
- Bilingual: English headings/terms, Indonesian explanations
- Graduate-level depth — analytical, not descriptive
- Refer to the existing Chapter 1 Study Guide (`Chapter1_StudyGuide_FSA_Subramanyam.html`) as the quality benchmark
- Indonesian company examples use real financial data with source citations
- Article analyses for degraded-mode articles include the `<div class="degraded-warning">` banner

### Task 7: Week 01 Content — Introduction to FSA

**Files:**
- Create: `course-materials/outputs/Study Guide - Aid/Week 01/study-guide.html`
- Create: `course-materials/outputs/Main Summary - Ebook/Week 01/main-summary.html`
- Create: `course-materials/outputs/Indonesian Company Examples/Week 01/id-examples.html`
- Create: `course-materials/outputs/Indonesian Company Examples/Week 01/sources.md`

- [ ] **Step 1: Create Week 01 Study Guide fragment**

This is the foundational week — the study guide serves as the framework for ALL subsequent weeks.

Content must cover:
- What FSA is and why it matters at graduate level
- Reading order: Start with Subramanyam Ch.1 (primary), supplement with Palepu Ch.1
- Central concepts to focus on: business analysis framework, role of financial statements, types of analysis (credit, equity, managerial)
- How this week's framework connects to all subsequent weeks
- Key questions to guide reading
- How the Main Summary should be approached for this and future weeks

- [ ] **Step 2: Create Week 01 Main Summary fragment**

Adapt and extend from the existing `Chapter1_StudyGuide_FSA_Subramanyam.html` baseline. This file already contains 986 lines of high-quality bilingual content covering Chapter 1. Extract the content portions (remove the `<html>`, `<head>`, CSS, and `<body>` wrappers) and use them as the fragment. Enrich with Palepu Ch.1 supplementary material where appropriate.

- [ ] **Step 3: Create Week 01 Indonesian Examples fragment**

UNVR (Unilever Indonesia) examples — adapt from the existing `.id-box` sections in the Chapter 1 guide. Add real financial data from UNVR 2023 Annual Report.

- [ ] **Step 4: Create Week 01 sources.md**

```markdown
# Week 01 — Indonesian Company Example Sources

## UNVR — Unilever Indonesia Tbk
- Annual Report 2023: https://www.unilever.co.id/investor-relations/
- Financial Statements 2023 (audited): IDX filing
- Company profile: IDX Fact Book 2024
```

- [ ] **Step 5: Validate**

Run: `cargo run -- validate` from `Dev Assistant/`
Expected: Week 01 shows 3 fragments found (study-guide, main-summary, id-examples). No article-analysis expected for Week 01.

- [ ] **Step 6: Commit**

```
git add "course-materials/outputs/Study Guide - Aid/Week 01/" "course-materials/outputs/Main Summary - Ebook/Week 01/" "course-materials/outputs/Indonesian Company Examples/Week 01/"
git commit -m "content: Week 01 — Introduction to FSA

Study guide, main summary (Subramanyam Ch.1 + Palepu Ch.1),
and Indonesian examples (UNVR).

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 8: Week 02 Content — Financial Reporting and Analysis

**Files:**
- Create: `course-materials/outputs/Study Guide - Aid/Week 02/study-guide.html`
- Create: `course-materials/outputs/Main Summary - Ebook/Week 02/main-summary.html`
- Create: `course-materials/outputs/Indonesian Company Examples/Week 02/id-examples.html`
- Create: `course-materials/outputs/Indonesian Company Examples/Week 02/sources.md`

- [ ] **Step 1: Create Week 02 Study Guide**

Focus on: Subramanyam Ch.1 (deepened) + Ch.2, Palepu Ch.1-2 supplement.
Guide reading order, emphasize accrual accounting quality, earnings management concepts, regulatory environment (Indonesian context: OJK, PSAK vs IFRS).

- [ ] **Step 2: Create Week 02 Main Summary**

Covers: financial reporting environment, accrual accounting framework, earnings quality, accounting choices, regulatory framework. Connect Subramanyam Ch.2 concepts with Palepu Ch.2 (strategy analysis introduction).

- [ ] **Step 3: Create Week 02 ID Examples + sources.md**

UNVR continued — show reporting environment, PSAK compliance, accounting policy choices.

- [ ] **Step 4: Validate and Commit**

Run validate, then commit with message: `content: Week 02 — Financial Reporting and Analysis`

---

### Task 9: Week 03 Content — Financing Activities + Altman 1968

**Files:**
- Create: `course-materials/outputs/Study Guide - Aid/Week 03/study-guide.html`
- Create: `course-materials/outputs/Main Summary - Ebook/Week 03/main-summary.html`
- Create: `course-materials/outputs/Articles/Week 03/article-analysis.html`
- Create: `course-materials/outputs/Indonesian Company Examples/Week 03/id-examples.html`
- Create: `course-materials/outputs/Indonesian Company Examples/Week 03/sources.md`

- [ ] **Step 1: Create Week 03 Study Guide**

Focus on Subramanyam Ch.3 (financing). Guide how to connect Altman 1968 article to the financing chapter (Z-score uses financial ratios from balance sheet/income statement — directly relates to capital structure analysis).

- [ ] **Step 2: Create Week 03 Main Summary**

Covers: debt financing (bonds, notes), equity financing, leases, off-balance-sheet financing, capital structure analysis, financial leverage.

- [ ] **Step 3: Create Week 03 Article Analysis — Altman 1968**

Full analysis of: "Financial Ratios, Discriminant Analysis, and the Prediction of Corporate Bankruptcy" (Journal of Finance, 23, pp.589-609). Cover: research question, MDA methodology, Z-score model, 5 ratios, classification accuracy, contribution to bankruptcy prediction literature. Connect to Week 3's financing analysis (how capital structure ratios predict distress).

- [ ] **Step 4: Create Week 03 ID Examples + sources.md**

BBCA — debt/equity structure, subordinated bonds, capital adequacy ratios.

- [ ] **Step 5: Validate and Commit**

Run validate (should find 4 fragments for Week 03), commit: `content: Week 03 — Financing Activities + Altman 1968`

---

### Task 10: Week 04 Content — Investing Activities

**Files:** Study guide, main summary, ID examples + sources (no article this week)

- [ ] **Step 1-4: Create all Week 04 fragments**

Subramanyam Ch.4: PP&E analysis, intangibles, depreciation methods, impairment, capitalization decisions.
ASII (Astra International) — major capex patterns, asset base analysis, goodwill from acquisitions.

- [ ] **Step 5: Validate and Commit**

`content: Week 04 — Investing Activities`

---

### Task 11: Week 05 Content — Intercorporate Investments + Raheman 2010

**Files:** Study guide, main summary, article analysis (degraded), ID examples + sources

- [ ] **Step 1-4: Create Week 05 fragments**

Subramanyam Ch.5: equity method, consolidation, minority interests, goodwill.
Article: Raheman et al. 2010 — **degraded mode** analysis. Include `<div class="degraded-warning">` banner. Cover: working capital management framework, sector-wise analysis methodology, connection to intercorporate investment (managing subsidiary working capital).
ASII + UNTR — holding/subsidiary structure, consolidation entries, sources.md.

- [ ] **Step 5: Validate and Commit**

`content: Week 05 — Intercorporate Investments + Raheman 2010`

---

### Task 12: Week 06 Content — Operating Activities

**Files:** Study guide, main summary, ID examples + sources

- [ ] **Step 1-4: Create all Week 06 fragments**

Subramanyam Ch.6 + Palepu Ch.4 supplement: revenue recognition, operating expenses, non-recurring items, earnings persistence, income measurement.
TLKM — revenue recognition complexity across segments, operating expense structure.

- [ ] **Step 5: Validate and Commit**

`content: Week 06 — Operating Activities`

---

### Task 13: Week 07 Content — Pre-Midterm Review + Jones 1991 + Acaravci 2007

**Files:** Study guide, main summary (synthesis), article analysis (2 articles), ID examples + sources

- [ ] **Step 1: Create Week 07 Study Guide**

Focus on synthesis of Weeks 1-6. Exam preparation strategy. How the two articles connect to the broader FSA framework.

- [ ] **Step 2: Create Week 07 Main Summary**

Synthesis document — no new chapter, but connect all concepts from Ch.1-6. Key analytical frameworks, ratio categories, cross-chapter connections.

- [ ] **Step 3: Create Week 07 Article Analysis — Jones 1991 + Acaravci 2007**

Jones 1991: "Earnings Management During Import Relief Investigations" — the Jones Model for detecting earnings management through discretionary accruals. Research question, methodology, connection to Week 6 (operating activities / earnings quality).

Acaravci 2007: "Inter-industry Convergence in Financial Ratios: Evidence from Turkey" — tests whether financial ratios converge across industries over time. Methodology, findings, connection to ratio analysis fundamentals (Weeks 1-6).

- [ ] **Step 4: Create Week 07 ID Examples + sources.md**

Various IDX cases — earnings management red flags in Indonesian listed companies.

- [ ] **Step 5: Validate and Commit**

`content: Week 07 — Pre-Midterm Review + Jones 1991 + Acaravci 2007`

---

### Chunk 2 Validation

- [ ] **Run full validation for Weeks 1-7**

```
cargo run -- validate
```
Expected: All fragments for Weeks 1-7 found. Weeks 8-14 still missing.

- [ ] **Run build for assembled pages**

```
cargo run -- build
```
Expected: Assembles all Week 1-7 pages into `assembled/` directory.

- [ ] **Spot-check assembled HTML**

Open 2-3 assembled pages in a browser to verify:
- CSS renders correctly
- Week banner shows correct metadata
- Content fragment is injected properly
- Print preview (Ctrl+P) shows clean A4 layout

---

## Chunk 3: Content Generation — Weeks 8-14

### Task 14: Week 08 Content — Cash Flow Analysis

**Files:** Study guide, main summary, ID examples + sources

- [ ] **Step 1-4: Create all Week 08 fragments**

Subramanyam Ch.7: Statement of cash flows, direct vs indirect method, free cash flow calculation, cash flow quality, operating/investing/financing classification. First post-midterm topic.
ICBP — complex CFO/CFI patterns across food manufacturing segments.

- [ ] **Step 5: Validate and Commit**

`content: Week 08 — Cash Flow Analysis`

---

### Task 15: Week 09 Content — Profitability and ROIC

**Files:** Study guide, main summary, ID examples + sources

- [ ] **Step 1-4: Create all Week 09 fragments**

Subramanyam Ch.8 + Palepu Ch.5-6 supplement: DuPont decomposition (3-step and 5-step), ROE analysis, ROIC calculation, profit margins, asset turnover, financial leverage effect, sustainable growth rate.
BBRI — full DuPont decomposition with real data, ROE/ROIC trend analysis.

- [ ] **Step 5: Validate and Commit**

`content: Week 09 — Profitability and ROIC`

---

### Task 16: Week 10 Content — Prospective Analysis + Murty & Misra 2004

**Files:** Study guide, main summary, article analysis (degraded), ID examples + sources

- [ ] **Step 1-4: Create all Week 10 fragments**

Subramanyam Ch.9 + Palepu Ch.7 supplement: earnings forecasting, pro forma financial statements, scenario analysis, growth rate projection, forecast horizon, terminal value concepts.
Article: Murty & Misra 2004 — **degraded mode** analysis. Cash flow ratios as failure predictors. Connect to prospective analysis (forecasting distress using cash flow metrics).
GOTO — growth-stage company, prospective valuation challenges, burn rate analysis.

- [ ] **Step 5: Validate and Commit**

`content: Week 10 — Prospective Analysis + Murty & Misra 2004`

---

### Task 17: Week 11 Content — Credit Analysis

**Files:** Study guide, main summary, ID examples + sources

- [ ] **Step 1-4: Create all Week 11 fragments**

Subramanyam Ch.10: liquidity ratios, solvency ratios, credit scoring models, Altman Z-score application, bond ratings, financial distress prediction, lending decision framework.
BUMI — financial distress history, debt restructuring, credit risk indicators from actual financial data.

- [ ] **Step 5: Validate and Commit**

`content: Week 11 — Credit Analysis`

---

### Task 18: Week 12 Content — Equity Valuation + Cheng 2006

**Files:** Study guide, main summary, article analysis (degraded), ID examples + sources

- [ ] **Step 1-4: Create all Week 12 fragments**

Subramanyam Ch.11 + Palepu Ch.8-9 supplement: residual income model, DDM, DCF, P/E and P/B multiples, equity risk premium, intrinsic value estimation, market efficiency implications.
Article: Cheng 2006 — **degraded mode** analysis. Financial ratios and IPO returns using PCA. Connect to equity valuation (which ratios drive post-IPO performance).
BBCA — P/E, P/B multiples, residual income calculation with real data.

- [ ] **Step 5: Validate and Commit**

`content: Week 12 — Equity Valuation + Cheng 2006`

---

### Task 19: Week 13 Content — Comprehensive Case

**Files:** Study guide, main summary (case methodology), ID examples + sources

- [ ] **Step 1-4: Create all Week 13 fragments**

Palepu Ch.10 supplement: comprehensive case methodology. Study guide walks through applying ALL prior FSA techniques to a single company.
HMSP (HM Sampoerna) — full comprehensive case: industry analysis, ratio analysis, cash flow analysis, profitability decomposition, prospective analysis, credit assessment, equity valuation. This is the capstone integration week.

- [ ] **Step 5: Validate and Commit**

`content: Week 13 — Comprehensive Case`

---

### Task 20: Week 14 Content — Pre-Final Review + Deakin 1972 + Ohlson 1980

**Files:** Study guide, main summary (synthesis), article analysis (2 articles), ID examples + sources

- [ ] **Step 1: Create Week 14 Study Guide**

Final synthesis of entire course. Exam preparation. How Deakin and Ohlson relate to the full-course bankruptcy prediction thread (Altman Week 3 → Credit Analysis Week 11 → Deakin/Ohlson Week 14).

- [ ] **Step 2: Create Week 14 Main Summary**

Course synthesis — connect all analytical frameworks across Weeks 1-13. Key concepts, formulas, and analytical tools organized by topic cluster.

- [ ] **Step 3: Create Week 14 Article Analysis**

Deakin 1972: "A Discriminant Analysis of Predictors of Business Failure" (JAR, Vol.10, No.1). Extends Beaver's univariate approach, tests discriminant analysis with different ratio sets, predictive accuracy over time.

Ohlson 1980: "Financial Ratios and the Probabilistic Prediction of Bankruptcy" (JAR, 18, pp.109-131). Logit model as alternative to MDA, O-score, probabilistic approach, advantages over Altman's Z-score.

Connect both to the bankruptcy prediction thread: Altman (1968, MDA) → Deakin (1972, extended MDA) → Ohlson (1980, logistic regression). Show evolution of methodology.

- [ ] **Step 4: Create Week 14 ID Examples + sources.md**

IDX delisted companies — apply Altman Z-score, O-score to real Indonesian cases of corporate failure.

- [ ] **Step 5: Validate and Commit**

`content: Week 14 — Pre-Final Review + Deakin 1972 + Ohlson 1980`

---

### Chunk 3 Validation

- [ ] **Run full validation for all 14 weeks**

```
cargo run -- validate
```
Expected: ✅ All fragments found.

- [ ] **Run full build**

```
cargo run -- build
```
Expected: Assembles all 14 weeks across all content types.

---

## Chunk 4: Visual Companion, Print Bundle, and Final Deliverables

### Task 21: Generate Visual Companion

- [ ] **Step 1: Run companion generator**

```
cargo run -- companion
```
Expected: Generates `course-materials/outputs/Visual Companion/index.html`

- [ ] **Step 2: Verify in browser**

Open the Visual Companion and check:
- All 4 views render correctly (Roadmap, Chapters, Articles, Indonesian)
- Tab switching works
- Data matches course-map.json
- Print preview shows all views

- [ ] **Step 3: Commit**

```
git add "course-materials/outputs/Visual Companion/"
git commit -m "feat: generate Visual Companion HTML

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 22: Generate Master Print Bundle

- [ ] **Step 1: Run print-bundle generator**

```
cargo run -- print-bundle
```
Expected: Generates `course-materials/outputs/Master Print Bundle/course-companion.html`

- [ ] **Step 2: Verify print layout**

Open in browser → Print Preview (Ctrl+P):
- Table of contents on first page
- Each week starts on new page
- Page breaks are clean
- CSS prints correctly (colors preserved with `-webkit-print-color-adjust`)

- [ ] **Step 3: Commit**

```
git add "course-materials/outputs/Master Print Bundle/"
git commit -m "feat: generate Master Print Bundle

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 23: Working Notes and Source Documentation

**Files:**
- Create: `course-materials/outputs/working-notes.md`
- Copy: `Dev Assistant/configs/course-map.json` → `course-materials/outputs/course-map.json`
- Verify all `sources.md` files exist in Indonesian Company Examples folders

- [ ] **Step 1: Create working-notes.md**

```markdown
# Working Notes — FSA ALK301 Study System

## Chapter-to-Week Mapping Decisions

1. **Palepu-Healy-Peek integration**: Not explicitly referenced in syllabus (only "S" = Subramanyam). Mapped based on topic overlap:
   - Palepu Ch.1-2 → Weeks 1-2 (business analysis framework)
   - Palepu Ch.4 → Week 6 (operating analysis)
   - Palepu Ch.5-6 → Week 9 (profitability, return analysis)
   - Palepu Ch.7 → Week 10 (forecasting)
   - Palepu Ch.8-9 → Week 12 (valuation)
   - Palepu Ch.10 → Week 13 (comprehensive case)

2. **Article 2 title**: Syllabus says "Raheman, Abdul; Abdul Qayyum, and Talat Afza (2010) Sector-wise Analysis of Working Capital Management..." — different author order and title from the commonly cited version with Bodla as co-author.

3. **Week 7 and 14**: "Review bahan" weeks — no new textbook chapters, focus on synthesis and article discussion.

4. **Week 13**: "Comprehensive case" — treated as capstone integration of all FSA techniques.

## Article Availability

- Articles 2, 5, 6 analyzed in degraded mode (abstract/citation-based) — full texts not freely available online.
- Downloaded articles: Altman 1968, Jones 1991, Acaravci 2007, Deakin 1972, Ohlson 1980.

## Indonesian Company Selection Rationale

See course-map.json for per-week rationale. Selection criteria:
- Topical relevance to chapter concepts
- Data availability (public annual reports)
- Diversity of industries and financial characteristics
- Real, concrete, verifiable data only
```

- [ ] **Step 2: Verify all sources.md files exist**

```powershell
1..14 | ForEach-Object {
    $path = "course-materials\outputs\Indonesian Company Examples\Week $("{0:D2}" -f $_)\sources.md"
    if (Test-Path $path) { Write-Host "✅ Week $_ sources.md exists" }
    else { Write-Host "❌ Week $_ sources.md MISSING" }
}
```

- [ ] **Step 3: Copy course-map.json to outputs**

```powershell
Copy-Item "Dev Assistant\configs\course-map.json" "course-materials\outputs\course-map.json"
```

- [ ] **Step 4: Commit**

```
git add "course-materials/outputs/working-notes.md" "course-materials/outputs/course-map.json"
git commit -m "docs: add working notes, mapping decisions, and course-map reference copy

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 24: Final Validation

- [ ] **Step 1: Run complete validation**

```
cargo run -- validate
```
Expected: ✅ All fragments found.

- [ ] **Step 2: Run complete build**

```
cargo run -- build
```
Expected: All pages assembled without errors.

- [ ] **Step 3: Spot-check 3 random weeks in browser**

Verify assembled HTML for Weeks 3, 9, and 14:
- Content renders with correct CSS
- Week banner shows accurate metadata
- All component boxes display properly
- Print preview is clean

- [ ] **Step 4: Final commit**

```
git add -A
git commit -m "feat: complete FSA ALK301 study system

14 weeks of Study Guides, Main Summaries, Article Analyses,
and Indonesian Company Examples. Visual Companion and Master
Print Bundle generated. All validation passed.

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```
