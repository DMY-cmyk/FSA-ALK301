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
        anyhow::bail!("{} fragments found, {} missing", found, missing.len());
    }
}
