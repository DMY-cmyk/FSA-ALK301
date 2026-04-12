# Working Notes — FSA ALK301 Study System

## Overview

This document records all assumptions, ambiguities, and mapping decisions made during the construction of the study system for "Analisis Laporan Keuangan dan Valuasi Sekuritas" (Financial Statement Analysis and Securities Valuation), STIE YKPN Yogyakarta, Semester 1 / 2025-2026.

---

## Chapter-to-Week Mapping Decisions

### 1. Palepu-Healy-Peek Integration

The syllabus primarily references Subramanyam ("S" notation). Palepu-Healy-Peek chapters were mapped as supplementary material based on topic overlap:

| Palepu Chapter | Mapped to Week | Rationale |
|---|---|---|
| Ch.1 (Framework) | Week 1-2 | Business analysis framework parallels Subramanyam Ch.1 |
| Ch.2 (Strategy) | Week 2 | Strategy analysis complements financial reporting environment |
| Ch.4 (Accounting) | Week 6 | Accounting analysis of operating activities |
| Ch.5-6 (Financial Analysis) | Week 9 | Ratio analysis and profitability framework |
| Ch.7 (Prospective) | Week 10 | Forecasting methodology supplements Subramanyam Ch.9 |
| Ch.8-9 (Valuation) | Week 12 | Equity valuation techniques |
| Ch.10 (Case) | Week 13 | Comprehensive case methodology |

### 2. Article Title and Author Variations

- **Article 2 (Week 5)**: Syllabus lists "Raheman, Abdul; Abdul Qayyum, and Talat Afza (2010) Sector-wise Analysis of Working Capital Management and Firm Performance in Manufacturing Sector of Pakistan." The author order and exact title differ from some commonly cited versions. Treated as given in syllabus.
- **Article 5 (Week 10)**: "Murty, A.V.N and Misra, D.P (2004)" — title matches syllabus exactly.
- **Article 6 (Week 12)**: "Cheng, Min-Tsung (2006)" — matches syllabus listing.

### 3. Review Weeks (7 and 14)

- **Week 7**: "Review bahan" — no new textbook chapters. Dedicated to mid-course synthesis and article discussion (Jones 1991, Acaravci 2007). Positioned before midterm exam.
- **Week 14**: "Review bahan" — final synthesis. Article discussion (Deakin 1972, Ohlson 1980) completes the bankruptcy prediction thread started in Week 3 (Altman 1968).

### 4. Week 13 (Comprehensive Case)

Treated as capstone integration week — no new Subramanyam chapter. Palepu Ch.10 provides case methodology framework. HMSP selected as case company for demonstrating all FSA techniques.

---

## Article Availability

| Article | Week | Status | Notes |
|---|---|---|---|
| Altman (1968) | 3 | ✅ Available | Full PDF downloaded and analyzed |
| Raheman et al. (2010) | 5 | ⚠️ Degraded | Not freely accessible; analyzed from metadata/abstract |
| Jones (1991) | 7 | ✅ Available | Full PDF downloaded and analyzed |
| Acaravci (2007) | 7 | ✅ Available | Full PDF downloaded and analyzed |
| Murty & Misra (2004) | 10 | ⚠️ Degraded | Not freely accessible; analyzed from metadata/abstract |
| Cheng (2006) | 12 | ⚠️ Degraded | Not freely accessible; analyzed from metadata/abstract |
| Deakin (1972) | 14 | ✅ Available | Full PDF downloaded and analyzed |
| Ohlson (1980) | 14 | ✅ Available | Full PDF downloaded and analyzed |

Degraded articles include a visible warning banner (`degraded-warning` CSS class) in the HTML output explaining the limitation.

---

## Indonesian Company Selection Rationale

| Week | Company | Ticker | Selection Rationale |
|---|---|---|---|
| 1-2 | Unilever Indonesia | UNVR | Clean financials, high-quality reporting, continues Ch.1 baseline |
| 3 | Bank Central Asia | BBCA | Strong financing structure, illustrates capital structure analysis |
| 4 | Astra International | ASII | Diversified conglomerate, investing activities analysis |
| 5 | Astra Intl + United Tractors | ASII+UNTR | Parent-subsidiary relationship for intercorporate analysis |
| 6 | Telkom Indonesia | TLKM | SOE with complex operating activities |
| 7 | Various IDX | — | Mid-course review using diverse company examples |
| 8 | Indofood CBP | ICBP | Consumer staples, strong cash flow generation |
| 9 | Bank Rakyat Indonesia | BBRI | Banking sector ROE/ROA analysis, DuPont decomposition |
| 10 | GoTo Gojek Tokopedia | GOTO | Tech startup, prospective analysis challenges |
| 11 | Bumi Resources | BUMI | Financial distress history, credit risk indicators |
| 12 | Bank Central Asia | BBCA | Premium valuation multiples, residual income model |
| 13 | HM Sampoerna | HMSP | Comprehensive case — well-documented, diversified operations |
| 14 | IDX Delisted Companies | Various | Bankruptcy prediction — BTEL, BLTA, DAVO, TRUB |

Selection criteria:
- Topical relevance to chapter concepts
- Data availability (public annual reports, IDX filings)
- Diversity of industries and financial characteristics
- Real, concrete, verifiable data only
- Preference for companies with interesting analytical stories

---

## Data Source Policy

All Indonesian company data is sourced exclusively from:
1. **Audited financial statements** filed with IDX
2. **Annual reports** published by companies
3. **IDX company profiles** and filings
4. **Official investor relations** materials
5. **Independent auditor reports** (Big Four/reputable firms)

Each week's `sources.md` file in `Indonesian Company Examples/Week NN/` documents the specific data points used and their provenance.

---

## Technical Architecture Notes

- **Build system**: Rust CLI (`fsa-dev-assistant`) with 4 subcommands: `build`, `validate`, `companion`, `print-bundle`
- **Content format**: HTML fragments (no `<html>/<head>/<body>` wrappers) assembled via Tera templates
- **CSS design system**: Navy/gold academic theme with Indonesian red accent, A4 print optimization
- **Course map**: `course-map.json` serves as the single source of truth for week-topic-chapter-article mapping
- **Fragment naming convention**: deterministic paths `{content_type_dir}/Week {NN}/{filename}`

---

## Known Limitations

1. Three articles analyzed in degraded mode due to paywall/availability constraints
2. Indonesian company data uses the most recent available fiscal years (typically 2019-2023), not real-time
3. Valuation calculations are illustrative and should not be used for actual investment decisions
4. Some companies' data may have been restated in subsequent filings — we use the originally published figures

---

*Generated as part of the FSA ALK301 Study System. Last updated: July 2025.*
