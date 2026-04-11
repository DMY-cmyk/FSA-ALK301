use std::path::Path;

pub fn run(project_root: &Path) -> anyhow::Result<()> {
    let config_path = crate::utils::configs_dir(project_root).join("course-map.json");
    let course_map = crate::config::CourseMap::load(&config_path)?;
    let output_root = crate::utils::outputs_dir(project_root);
    let templates_dir = crate::utils::templates_dir(project_root);

    let print_css = std::fs::read_to_string(templates_dir.join("print.css"))?;

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
                let html = std::fs::read_to_string(&assembled)?;
                if let (Some(start), Some(end)) = (html.find("<main>"), html.find("</main>")) {
                    let content_start = start + "<main>".len();
                    if content_start > end || end > html.len() {
                        continue;
                    }
                    let body = &html[content_start..end];
                    all_content.push_str(&format!(
                        "<div class=\"content-section {}\">\n<h2>{}</h2>\n{}\n</div>\n",
                        ct.slug(), ct.display_name(), body
                    ));
                }
            }
        }

        all_content.push_str("</div>\n<div class=\"page-break\"></div>\n");
    }

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
