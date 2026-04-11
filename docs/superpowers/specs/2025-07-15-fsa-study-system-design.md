# FSA Study System — Design Specification

**Date:** 2025-07-15
**Course:** Analisis Laporan Keuangan dan Valuasi Sekuritas (ALK301)
**Institution:** Pascasarjana STIE YKPN Yogyakarta
**Level:** Master's (Magister Akuntansi / Magister Manajemen)

---

## 1. Problem Statement

The course requires a comprehensive, graduate-level study system that integrates:
- A 14-week syllabus with Subramanyam (FSA, 11th ed.) as the primary textbook
- Palepu-Healy-Peek (Business Analysis and Valuation) as supplementary enrichment
- 8 assigned academic articles mapped to specific weeks
- Indonesian company examples using real financial data
- A Visual Companion for navigating the learning structure
- Print-to-PDF fidelity for all HTML outputs

The system must feel like material prepared by a lecturer/researcher, not a generic AI summary.

---

## 2. Approach: Hybrid Pipeline

Three layers work together:

1. **Mapping Layer** — A structured `course-map.json` defines the authoritative week-by-week structure
2. **Content Layer** — Handcrafted HTML fragments per week, matching the quality of the existing Chapter 1 Study Guide baseline
3. **Assembly Layer** — Rust CLI reads mapping + fragments → assembles full HTML pages with consistent styling, navigation, and print layout

---

## 3. System Architecture

### 3.1 Data Flow

```
course-map.json ──┐
                   ├──> Rust Assembler ──> Full HTML Pages (per week)
HTML fragments  ──┘         │
                            ├──> Visual Companion (single interactive HTML)
                            └──> Master Print Bundle (single concatenated HTML)
```

### 3.2 course-map.json Schema

```json
{
  "course": {
    "name": "Analisis Laporan Keuangan dan Valuasi Sekuritas",
    "code": "ALK301",
    "semester": "1 / 2025-2026",
    "primary_text": "Subramanyam, K.R. Financial Statement Analysis, 11th ed.",
    "supplementary_text": "Palepu, Healy, Peek. Business Analysis and Valuation."
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
      "connections": {
        "builds_on": [],
        "leads_to": [2]
      }
    }
  ]
}
```

Each week entry contains: `week`, `topic`, `topic_en`, `subramanyam_chapters[]`, `palepu_chapters[]`, `articles[]`, `id_company{}`, `learning_focus`, `connections{}`.

### 3.3 Fragment Discovery Convention

The assembler locates HTML fragments using a deterministic path convention derived from week number and content type. No file paths are stored in `course-map.json` — the mapping is purely structural.

**Fragment path pattern:**
```
{output_root}/{content_type_dir}/Week {NN}/{filename}
```

Where:
- `{output_root}` = `course-materials/outputs/` (configurable via CLI `--output-dir`)
- `{content_type_dir}` and `{filename}` are fixed per content type:

| Content Type | Directory | Filename |
|---|---|---|
| Study Guide | `Study Guide - Aid` | `study-guide.html` |
| Main Summary | `Main Summary - Ebook` | `main-summary.html` |
| Article Analysis | `Articles` | `article-analysis.html` |
| Indonesian Examples | `Indonesian Company Examples` | `id-examples.html` |

**Example:** Week 3's study guide fragment is at:
```
course-materials/outputs/Study Guide - Aid/Week 03/study-guide.html
```

The `validate` subcommand checks that every week listed in `course-map.json` has all expected fragments present (article analysis only for weeks with `articles.len() > 0`). Validation output is human-readable to stderr with exit code 0 (all valid) or 1 (missing files listed).

### 3.4 Path Configuration

The Rust CLI resolves all paths relative to a **project root**, which defaults to the current working directory. The expected layout from project root:

```
{project_root}/
├─ course-materials/outputs/    # Fragment source + assembled output
└─ Dev Assistant/               # Rust project (CWD when running cargo)
```

The CLI accepts `--project-root <path>` to override (default: `..` when run from `Dev Assistant/`). All internal path resolution uses this single root. No hardcoded absolute paths.

### 3.5 Week-to-Chapter Mapping (Complete)

| Week | Topic | Sub. Ch. | Palepu Ch. | Articles | ID Company |
|------|-------|----------|------------|----------|------------|
| 1 | Pengenalan ALK | 1 | 1 | — | UNVR |
| 2 | Analisis & pelaporan keuangan | 1, 2 | 1, 2 | — | UNVR |
| 3 | Analisis pendanaan (financing) | 3 | — | Art.1: Altman 1968 | BBCA |
| 4 | Analisis investasi | 4 | — | — | ASII |
| 5 | Intercorporate investment | 5 | — | Art.2: Raheman et al. 2010 | ASII, UNTR |
| 6 | Analisis aktivitas operasi | 6 | 4 | — | TLKM |
| 7 | Review bahan | — | — | Art.3: Jones 1991, Art.4: Acaravci 2007 | Various |
| — | **Ujian Tengah Semester** | — | — | — | — |
| 8 | Analisis aliran kas | 7 | — | — | ICBP |
| 9 | Profitabilitas & ROIC | 8 | 5, 6 | — | BBRI |
| 10 | Analisis prospektif | 9 | 7 | Art.5: Murty & Misra 2004 | GOTO |
| 11 | Analisis kredit | 10 | — | — | BUMI |
| 12 | Analisis ekuitas & penilaian | 11 | 8, 9 | Art.6: Cheng 2006 | BBCA |
| 13 | Comprehensive case | — | 10 | — | HMSP |
| 14 | Review bahan | — | — | Art.7: Deakin 1972, Art.8: Ohlson 1980 | IDX delisted |

**Palepu-Healy-Peek Integration Notes:**
- Used as supplementary enrichment where topics overlap with Subramanyam
- Ch.1-2 → Weeks 1-2 (business analysis framework)
- Ch.4 → Week 6 (revenue recognition, operating analysis)
- Ch.5-6 → Week 9 (profitability, return analysis)
- Ch.7 → Week 10 (forecasting, prospective analysis)
- Ch.8-9 → Week 12 (valuation models, equity analysis)
- Ch.10 → Week 13 (comprehensive case methodology)

---

## 4. Output Structure

### 4.1 File Organization

```
course-materials/outputs/
├─ Study Guide - Aid/
│  ├─ Week 01/
│  │  └─ study-guide.html          # Fragment
│  ├─ Week 02/
│  │  └─ study-guide.html
│  └─ ... (Weeks 01-14)
├─ Main Summary - Ebook/
│  ├─ Week 01/
│  │  └─ main-summary.html         # Fragment
│  └─ ... (Weeks 01-14)
├─ Articles/
│  ├─ Week 03/
│  │  └─ article-analysis.html     # Fragment
│  ├─ Week 05/
│  ├─ Week 07/
│  ├─ Week 10/
│  ├─ Week 12/
│  └─ Week 14/
├─ Indonesian Company Examples/
│  ├─ Week 01/
│  │  ├─ id-examples.html          # Fragment
│  │  └─ sources.md                # Data sources documentation
│  └─ ... (Weeks 01-14)
├─ Visual Companion/
│  ├─ index.html                   # Rust-generated interactive page
│  └─ print.css
├─ Master Print Bundle/
│  ├─ course-companion.html        # All weeks concatenated
│  └─ print.css
├─ assembled/                      # Rust output: full assembled pages
│  ├─ week-01/
│  │  ├─ study-guide.html
│  │  ├─ main-summary.html
│  │  └─ id-examples.html
│  ├─ week-02/
│  └─ ...
├─ course-map.json                 # Copy for reference
└─ working-notes.md                # Assumptions & decisions
```

### 4.2 Content Types & Specifications

**Study Guide (`study-guide.html`):**
- Learning scaffold: reading order, concept hierarchy, focus areas
- Connections to prior/next weeks and course progression
- Key questions to guide reading
- Recommended approach for the Main Summary
- Target: ~500-800 words per week
- Week 1 specifically serves as the framework for understanding all subsequent Main Summaries

**Main Summary (`main-summary.html`):**
- Deep structured summary of Subramanyam chapter(s) for the week
- Palepu-Healy-Peek enrichment where topics overlap
- Component types from Ch.1 baseline:
  - `.def` — Definitions with bilingual terms
  - `.cg` — Colgate/textbook examples
  - `.fbox` — Formulas with notation explanation
  - `.ibox` — Analytical insights
  - `.exb` — Exercises and applications
  - `.qb` — Practice questions (graduate-level)
  - `.cc` — Cheat sheet cards (key points)
- Bilingual: English terms/headers, Indonesian explanations
- Target: ~2000-4000 words per week

**Article Analysis (`article-analysis.html`):**
- Research question and motivation
- Methodology and data
- Key findings and contribution
- Connection to the week's chapter topic
- Critical evaluation at graduate level
- Academic significance and limitations
- Target: ~800-1500 words per article

**Indonesian Company Examples (`id-examples.html`):**
- Real financial data from most recent available annual reports (2023/2024)
- Uses `.id-box` component style from Ch.1
- Analysis connects directly to chapter concepts
- Sources documented in accompanying `sources.md`
- Target: ~500-1000 words per week

---

## 5. Indonesian Company Selection

| Week | Topic | Primary Company | Rationale |
|------|-------|-----------------|-----------|
| 1-2 | FSA Overview | UNVR (Unilever Indonesia) | Continues Ch.1 baseline; transparent reporting |
| 3 | Financing | BBCA (Bank Central Asia) | Rich debt/equity structure, bond issuance |
| 4 | Investing | ASII (Astra International) | Major capex, diverse asset acquisition |
| 5 | Intercorporate | ASII + UNTR | Holding/subsidiary consolidation structure |
| 6 | Operations | TLKM (Telkom Indonesia) | Revenue recognition complexity, opex |
| 7 | Review | Various IDX cases | Earnings management in Indonesian context |
| 8 | Cash Flow | ICBP (Indofood CBP) | Complex CFO/CFI patterns |
| 9 | Profitability | BBRI (Bank Rakyat Indonesia) | ROE/ROIC decomposition, DuPont |
| 10 | Prospective | GOTO (GoTo Gojek Tokopedia) | Growth projections, valuation challenges |
| 11 | Credit | BUMI (Bumi Resources) | Distress history, credit risk |
| 12 | Equity Valuation | BBCA | P/E, P/B, residual income application |
| 13 | Comprehensive | HMSP (HM Sampoerna) | Full FSA case study |
| 14 | Review | IDX delisted companies | Bankruptcy prediction in ID context |

**Data Sources:**
- Annual reports from company IR pages
- Audited financial statements (IDX filings)
- IDX Fact Book and statistical data
- OJK regulatory disclosures

---

## 6. Rust Dev Assistant Architecture

### 6.1 Project Structure

```
Dev Assistant/
├─ Cargo.toml
├─ src/
│  ├─ main.rs              # CLI: build | validate | companion | print-bundle
│  ├─ config/
│  │  └─ mod.rs            # Serde structs for course-map.json
│  ├─ assembler/
│  │  └─ mod.rs            # Fragment + template → full HTML
│  ├─ companion/
│  │  └─ mod.rs            # Generate Visual Companion from course map
│  ├─ print_bundle/
│  │  └─ mod.rs            # Generate Master Print Bundle
│  ├─ validator/
│  │  └─ mod.rs            # Validate file references in course-map.json
│  └─ utils/
│     └─ mod.rs            # File I/O, path resolution
├─ configs/
│  └─ course-map.json      # Authoritative mapping
├─ templates/
│  ├─ base.html            # Page wrapper (CSS, nav, metadata)
│  ├─ nav.html             # Sidebar navigation partial
│  ├─ companion.html       # Visual Companion template
│  └─ print.css            # Print stylesheet
└─ README.md
```

### 6.2 Template Interface

**`base.html` template variables (Tera context):**

| Variable | Type | Description |
|---|---|---|
| `page_title` | String | e.g., "Week 03 — Study Guide" |
| `week_number` | u8 | 1-14 |
| `week_topic` | String | Indonesian topic title |
| `week_topic_en` | String | English topic title |
| `content_type` | String | "study-guide" / "main-summary" / "article-analysis" / "id-examples" |
| `subramanyam_chapters` | Vec<u8> | Chapter numbers |
| `palepu_chapters` | Vec<u8> | Chapter numbers |
| `articles` | Vec<{title, author, year}> | Article metadata |
| `id_company` | {ticker, name} | Indonesian company info |
| `learning_focus` | String | One-line learning focus |
| `body_content` | String | The HTML fragment content (injected raw) |
| `nav_items` | Vec<{week, topic, active}> | Sidebar navigation data |
| `prev_week` | Option<u8> | Previous week number (None for week 1) |
| `next_week` | Option<u8> | Next week number (None for week 14) |

**Template block structure in `base.html`:**
```html
<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
  <title>{{ page_title }}</title>
  <style>/* base CSS + print CSS inlined */</style>
</head>
<body>
  <nav>{% include "nav.html" %}</nav>
  <header><!-- week metadata banner from variables --></header>
  <main>{{ body_content | safe }}</main>
  <footer><!-- prev/next navigation --></footer>
</body>
</html>
```

### 6.3 Dependencies

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tera = "1"               # Template engine
clap = { version = "4", features = ["derive"] }
walkdir = "2"
anyhow = "1"
```

### 6.4 CLI Subcommands

- `cargo run -- build` — Reads course-map.json, finds fragment files, assembles into full HTML pages in `assembled/`
- `cargo run -- validate` — Checks that all referenced fragments exist, reports missing files
- `cargo run -- companion` — Generates Visual Companion HTML from course map data
- `cargo run -- print-bundle` — Concatenates all assembled pages into single print-ready HTML

---

## 7. Visual Companion Design

Single-page interactive HTML with vanilla JS (no framework dependencies).

**Views:**
1. **Course Roadmap** — Timeline of 14 weeks + midterm + final, color-coded by topic cluster (financing → blue, investing → green, operations → orange, valuation → purple)
2. **Chapter Map** — Subramanyam chapters mapped to weeks, with Palepu overlay shown as secondary layer
3. **Article Network** — Articles positioned against assigned weeks with connecting lines to relevant chapters
4. **Indonesian Examples** — Company examples per week with expandable data source info

**Interaction:**
- Tab-based navigation between views
- Collapsible sections per week
- Click any week to see its full content map (chapters, articles, companies, learning focus)
- Print-friendly stylesheet included

**Additional data for companion generation (hardcoded in companion module, not in course-map.json):**
- Topic cluster color assignments: financing → `#2196F3`, investing → `#4CAF50`, operations → `#FF9800`, valuation → `#9C27B0`, review → `#607D8B`
- Article-to-chapter connecting lines are derived from the week's `subramanyam_chapters` + `articles` arrays in course-map.json — the companion module draws connections between items sharing the same week

---

## 8. Print Layout Design

Extends the existing Chapter 1 print stylesheet:

```css
@page {
  size: A4;
  margin: 1.8cm 2cm 2cm 2cm;
}

@media print {
  .no-print { display: none; }
  .page-break { page-break-before: always; }
  table, .fbox, .def, .id-box { break-inside: avoid; }
  h1, h2, h3 { break-after: avoid; }
  body { font-size: 10pt; line-height: 1.5; }
}
```

**Master Print Bundle:**
- Single HTML file with all weeks in order
- Table of contents at the start
- Each week starts on a new page
- Within each week: Study Guide → Main Summary → Articles → Indonesian Examples
- Page numbers via CSS counters

---

## 9. Writing & Language Standards

- **Internal/structural content:** Professional English
- **Student-facing outputs:** Bilingual — English terms and section headers, Indonesian explanations and analysis
- **Tone:** Graduate-level — assumes the reader has foundational accounting knowledge
- **Depth:** Analytical, not descriptive; connects concepts, evaluates trade-offs, positions material in academic context
- **Quality benchmark:** The existing Chapter 1 Study Guide HTML (986 lines, with Colgate + UNVR examples, formulas, practice questions, vocabulary grid)

---

## 10. Article Download Status

| # | Article | Status | Week |
|---|---------|--------|------|
| 1 | Altman 1968 — Financial Ratios, Discriminant Analysis, Corporate Bankruptcy | ✅ Downloaded | 3 |
| 2 | Raheman et al. 2010 — Sector-wise Working Capital Management | ❌ Not available online | 5 |
| 3 | Jones 1991 — Earnings Management During Import Relief | ✅ Downloaded | 7 |
| 4 | Acaravci 2007 — Inter-industry Convergence in Financial Ratios | ✅ Downloaded | 7 |
| 5 | Murty & Misra 2004 — Cash Flow Ratios as Indicators of Corporate Failure | ❌ Not available online | 10 |
| 6 | Cheng 2006 — Financial Ratios and Return from IPOs (PCA) | ❌ Not available online | 12 |
| 7 | Deakin 1972 — Discriminant Analysis of Predictors of Business Failure | ✅ Downloaded | 14 |
| 8 | Ohlson 1980 — Financial Ratios and Probabilistic Prediction of Bankruptcy | ✅ Downloaded | 14 |

Articles 2, 5, and 6 will be analyzed using a **degraded-mode format**. For these articles, the analysis will:
- Clearly state at the top: "⚠ Analysis based on published abstract, citation network, and known academic discussion — full text not available."
- Cover: research question (from title/abstract), methodology (from citation context), key findings (from citing papers), contribution to the field, and connection to the week's topic
- Omit: detailed data tables, specific regression coefficients, or direct quotations
- Include a "How to Access" section with JSTOR/EconBiz links and interlibrary loan guidance

If the user obtains full PDFs via university library, the analyses can be upgraded by re-running the content generation for those weeks.

---

## 11. Execution Phases

**Phase 1: Foundation**
1. Set up Dev Assistant Rust project
2. Create complete course-map.json
3. Build Rust assembler + validator
4. Create base HTML template + print CSS

**Phase 2: Content Generation**
5. Generate content in weekly batches (one week = one batch):
   - Each batch produces all fragments for that week (study guide + main summary + article analysis if applicable + Indonesian examples)
   - Run `cargo run -- validate` after each batch to confirm fragments exist
   - Run `cargo run -- build` after completing all 14 weeks (or periodically after every 3-4 weeks for intermediate checks)
6. Content generation order: Weeks 1-7, then 8-14 (matching pre-midterm / post-midterm structure)

**Phase 3: Visual Companion & Print Bundle**
7. Generate Visual Companion
8. Generate Master Print Bundle
9. Final validation and quality review

---

## 12. Working Notes & Assumptions

1. **Palepu-Healy-Peek mapping** is based on topic overlap analysis, not explicit syllabus reference. The syllabus only references "S" (Subramanyam). Palepu integration is supplementary.
2. **Article 2 title ambiguity**: The syllabus lists "Raheman, Abdul; Abdul Qayyum, and Talat Afza (2010) Sector-wise Analysis of Working Capital Management..." — this is a different paper from the more commonly cited "Working Capital Management and Corporate Performance" by the same authors with Bodla.
3. **Week 7 and 14** are "Review bahan" weeks with no new textbook chapters but with article assignments. Study Guides for these weeks focus on synthesis and exam preparation.
4. **Week 13** is "Comprehensive case" — the Study Guide and Summary will focus on applying all prior FSA techniques to a single company case.
5. **Indonesian company data** uses the most recent available annual reports (2023 or 2024 depending on publication date at time of content creation).
