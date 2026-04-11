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
