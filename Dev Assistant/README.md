# FSA Dev Assistant

Build pipeline for the ALK301 (Analisis Laporan Keuangan dan Valuasi Sekuritas) study system.

## Commands

```bash
cargo run -- build          # Assemble HTML fragments into full pages
cargo run -- validate       # Check all expected fragment files exist
cargo run -- companion      # Generate Visual Companion HTML
cargo run -- print-bundle   # Generate Master Print Bundle
```

## Architecture

Reads `configs/course-map.json` + handcrafted HTML fragments from `course-materials/outputs/` → assembles full pages with navigation, print CSS, and metadata via Tera templates.
