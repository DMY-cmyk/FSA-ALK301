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
