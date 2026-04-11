# Design Spec: Indonesian Localization — Chapter 1 Study Guide

**Source file:** `Chapter1_StudyGuide_FSA_Subramanyam.html`
**Date:** 2026-04-01
**Objective:** Add Indonesian company comparisons alongside existing Colgate-Palmolive textbook examples

---

## Constraints

- **Do NOT modify** any existing content that covers Subramanyam textbook material points
- **Do NOT replace** Colgate-Palmolive data — it stays as the primary textbook reference
- **ADD** Indonesian company comparison boxes alongside existing content
- Use **real published data** from 2023 annual reports (IDX filings)
- Currency in **Rupiah (Rp)** using Indonesian notation: `Rp16,7 triliun` (period as thousands separator, comma as decimal)

---

## Visual Design

### New CSS Class: `.id-box`

A new styled box type for Indonesian company comparisons, visually distinct from the existing gold Colgate `.cg` boxes:

- **Background:** Light cream `#FFF5F0`
- **Border:** 1px solid `#E8A0A0`
- **Border-left:** 4px solid `#C41E3A` (Indonesian red)
- **Title prefix:** `🇮🇩` flag emoji
- **Title color:** `#8B1A2B` (dark red)
- **Title class:** `.idt` — uppercase, small font, letter-spacing
- **Print behavior:** `.id-box` elements MUST also carry `nb` class (page-break-inside: avoid) just like `.cg` boxes
- **Body text:** Same as `.cg` body (10pt, #444)

This creates a clear visual pattern: gold boxes = Colgate (textbook), red boxes = Indonesian comparison.

---

## Company Mapping

| Section | Indonesian Company | Ticker | Rationale |
|---|---|---|---|
| S2 (Business Analysis) | Unilever Indonesia | UNVR | Direct consumer goods parallel to Colgate |
| S3 (Financial Statements) | Unilever Indonesia | UNVR | Full FS walkthrough, 1:1 comparison |
| S4a-c (Comparative, Common-Size) | Telkom Indonesia | TLKM | Large telecom, interesting trend data |
| S4d (Ratio Reference) | Bank Central Asia | BBCA | Banking ratios show structural contrasts |
| S4e (Valuation) | Bank Central Asia | BBCA | Well-known IDX valuation, high P/E |
| S5 (EMH) | IDX/IHSG market context | — | Indonesian market efficiency, OJK role |
| S6 (Practice) | Mix (UNVR, TLKM) | — | New exercises with IDX company data |
| S7 (Cheat Sheet) | Indonesian regulatory terms | — | PSAK, OJK, BEI, IHSG vocabulary |

---

## Content Additions Per Section

### Section 2 — Business Analysis

**Add after existing Colgate application box (after line ~200, closing `</div>` of `class="cg"` Colgate box):**

1. **UNVR `.id-box nb`:** Unilever Indonesia's business model overview
   - Portfolio: Oral Care (Pepsodent), Personal Care (Lifebuoy, Dove, Sunsilk), Home Care (Rinso, Sunlight), Foods & Refreshment (Bango, Wall's)
   - IDX-listed since 1982 (ticker: UNVR), ~85% owned by Unilever NV
   - Direct parallel to Colgate: consumer goods, global parent with local subsidiary, raw material cost pressures
   - Key insight: UNVR's revenue turun 6.3% (2022→2023), laba bersih turun 10.5% — tekanan serupa dengan Colgate yang mengalami stagnasi 2004–2006 akibat restrukturisasi

2. **Regulatory context note (`.def nb` box, immediately after the `.id-box`):**
   - "Di Indonesia, laporan keuangan disusun berdasarkan PSAK (Pernyataan Standar Akuntansi Keuangan) yang berbasis IFRS, bukan US GAAP. Regulator utama adalah OJK (Otoritas Jasa Keuangan) dan Bursa Efek Indonesia (BEI/IDX)."

### Section 3 — Financial Statements

**Placement:** Each UNVR box goes IMMEDIATELY AFTER the corresponding Colgate box of the same statement type.

1. **UNVR Balance Sheet `.id-box nb`** — place after Colgate Balance Sheet box (after line ~235):
   - Total aset: Rp16,7 triliun | Total liabilitas: Rp13,3 triliun | Ekuitas: Rp3,4 triliun
   - Aset lancar: Rp6,2 triliun (37%) · Aset tetap & lainnya: Rp10,5 triliun (63%)
   - Liabilitas lancar: Rp11,2 triliun → working capital NEGATIF (Rp5,0 triliun)
   - Key contrast: UNVR has negative working capital (like Colgate's thin WC), tetapi di UNVR ini bahkan lebih ekstrem

2. **UNVR Income Statement `.id-box nb`** — place after Colgate Income Statement table (after line ~249):
   - Pendapatan bersih: Rp38,6 triliun | Beban pokok: Rp19,4 triliun
   - Laba bruto: Rp19,2 triliun (gross margin 49.7% — lebih rendah dari Colgate 57.3%)
   - Laba usaha: Rp6,3 triliun (operating margin 16.3%)
   - Laba bersih: Rp4,8 triliun (net margin 12.4%)

3. **UNVR Cash Flow `.id-box nb`** — place after Colgate Cash Flow table (after line ~276):
   - Arus kas operasi: Rp7,1 triliun (positif — bisnis FMCG menghasilkan kas stabil)
   - Arus kas investasi: −Rp0,8 triliun (belanja modal rutin)
   - Arus kas pendanaan: −Rp5,8 triliun (pembayaran dividen sangat besar)
   - Pattern mirip Colgate: generate cash from operations → return to shareholders via dividends
   - UNVR historically has very high dividend payout ratio (~100%), terlihat dari financing outflow yang hampir setara operating inflow

4. **Indonesian reporting context (`.id-box nb`)** — place after the Articulation section (after line ~283):
   - OJK (Otoritas Jasa Keuangan): regulator pasar modal, setara SEC di AS
   - BEI (Bursa Efek Indonesia): bursa efek, setara NYSE
   - Perusahaan publik wajib menyampaikan laporan keuangan tahunan yang diaudit dalam 120 hari setelah akhir tahun buku

### Section 4a-c — Comparative & Common-Size Analysis

**Add after existing Colgate common-size interpretation (after line ~360):**

1. **TLKM Year-to-Year `.id-box nb`** (2022→2023):

   | Item | 2023 (T) | 2022 (T) | Change (T) | % Change |
   |---|---|---|---|---|
   | Pendapatan | 149,2 | 147,3 | +1,9 | +1,3% |
   | Beban usaha | 104,3 | 107,7 | −3,4 | −3,2% |
   | Laba usaha | 44,4 | 39,6 | +4,8 | +12,1% |
   | Laba bersih | 24,6 | 20,8 | +3,8 | +18,3% |

   Interpretasi: Revenue tumbuh moderat (+1,3%), tapi efisiensi biaya menghasilkan pertumbuhan laba bersih +18,3% — mencerminkan keberhasilan program efisiensi, mirip Colgate yang mengkompensasi tekanan gross margin melalui penghematan SG&A.

2. **TLKM Common-Size `.id-box nb`** (2023):

   | Item | 2023 % |
   |---|---|
   | Pendapatan | 100,0% |
   | Beban usaha | 69,9% |
   | Laba usaha | 29,7% |
   | Laba bersih | 16,5% |

   → Operating margin TLKM (29,7%) lebih tinggi dari Colgate (22,95%) karena bisnis telekomunikasi memiliki operating leverage yang tinggi.

### Section 4d — Key Ratio Reference

**Add after existing Colgate ratio summary box (after line ~429):**

1. **BBCA Ratio Comparison `.id-box nb`** (2023):

   Banking-specific ratios — berbeda dari perusahaan manufaktur:

   | Rasio | Formula/Keterangan | BBCA 2023 |
   |---|---|---|
   | NIM (Net Interest Margin) | Net Interest Income / Avg Earning Assets | **5,5%** |
   | LDR (Loan to Deposit Ratio) | Total Kredit / DPK | **70%** |
   | NPL (Non-Performing Loan) | Kredit Bermasalah / Total Kredit | **1,9%** |
   | ROA | Laba Bersih / Rata-rata Total Aset | **3,6%** |
   | ROE | Laba Bersih / Rata-rata Total Ekuitas | **21,5%** |
   | D/E ratio | Total Liabilitas / Total Ekuitas | Rp1.166T / Rp242T = **4,82×** |

   Key insight: D/E ratio BBCA (4,82×) lebih tinggi dari Colgate (4,01×), tetapi ini NORMAL untuk bank. Bisnis perbankan adalah intermediasi: menghimpun dana masyarakat (DPK = Rp1.102 triliun) dan menyalurkannya sebagai kredit (Rp810 triliun). Leverage tinggi bukan tanda risiko jika NPL rendah dan CAR memadai.

### Section 4e — Valuation Models

**Add after existing valuation worked example (after line ~480):**

1. **IDX Valuation Context `.id-box nb`:**
   - BBCA market measures (2023): P/E ~14×, P/B ~3×, EPS Rp467, DPS Rp336, Dividend yield ~4,5%
   - Perbandingan: Colgate P/E 18,55× vs BBCA ~14×; Colgate P/B 17,05× vs BBCA ~3×
   - Perbedaan P/B dijelaskan oleh: (1) book value bank lebih dekat ke economic value karena aset kebanyakan finansial, (2) UNVR/Colgate memiliki intangible value (brand) yang jauh melampaui book value
   - IHSG (Jakarta Composite Index) average P/E: ~12–15×, lebih rendah dari S&P 500 (~20–25×) karena emerging market risk premium

### Section 5 — EMH

**Add after existing EMH implications box (after line ~530):**

1. **Indonesian Market Efficiency `.id-box nb`:**
   - BEI umumnya dianggap semistrong efficient untuk saham LQ45 (likuid), tetapi less efficient untuk saham small-cap
   - OJK berperan setara SEC: menegakkan keterbukaan informasi (full disclosure)
   - Anomali spesifik di IDX:
     - **Ramadan effect**: beberapa studi menunjukkan return positif abnormal selama bulan Ramadan (unik untuk pasar Muslim-majority)
     - **January effect**: ditemukan di IDX meski magnitude-nya lebih kecil dari pasar AS
     - **Post-earnings drift**: juga ditemukan di IDX, menunjukkan pasar tidak langsung fully efficient
   - Regulasi insider trading: UU Pasar Modal No. 8/1995, diperkuat oleh UU OJK No. 21/2011
   - Implikasi: FSA tetap sangat berguna di pasar Indonesia karena banyak perusahaan IDX yang kurang diikuti analis (under-covered) dibandingkan NYSE

### Section 6 — Practice Questions

**Add after the existing Case 1-3 (after line ~653):**

1. **New Exercise `.exb nb`: Analisis Rasio — PT Telkom Indonesia (TLKM) 2023**
   Given data (simplified from actual 2023 annual report):

   | Item | 2023 (Rp miliar) |
   |---|---|
   | Total aset | 287.042 |
   | Total liabilitas | 130.480 |
   | Total ekuitas | 156.562 |
   | Aset lancar | 55.613 |
   | Liabilitas lancar | 71.568 |
   | Pendapatan | 149.216 |
   | Beban usaha | 104.300 |
   | Laba usaha | 44.384 |
   | Laba bersih | 24.560 |

   Required:
   a. Current ratio
   b. Debt-to-equity ratio
   c. Gross profit margin (gunakan laba usaha/pendapatan sebagai proxy)
   d. Net profit margin
   e. ROA dan ROE (gunakan ending balances)
   f. Interpretasi: apakah TLKM lebih sehat dari Colgate (2011)? Bandingkan minimal 3 rasio.

   **Solution (`.sol`):**
   a. Current ratio: 55.613/71.568 = **0,78×** (di bawah 1,0 → working capital negatif)
   b. D/E ratio: 130.480/156.562 = **0,83×** (lebih rendah dari Colgate 4,01× → leverage jauh lebih konservatif)
   c. Operating margin: 44.384/149.216 = **29,7%** (lebih tinggi dari Colgate 22,95%)
   d. Net margin: 24.560/149.216 = **16,5%** (lebih tinggi dari Colgate 14,53%)
   e. ROA: 24.560/287.042 = **8,6%** | ROE: 24.560/156.562 = **15,7%**
   f. Interpretasi: TLKM memiliki operating margin dan net margin lebih tinggi dari Colgate, tetapi ROE jauh lebih rendah (15,7% vs 45,37%) karena leverage Colgate yang sangat tinggi memperbesar return on equity. Current ratio TLKM (0,78×) lebih rendah dari Colgate (1,18×), tetapi arus kas operasi yang kuat memitigasi risiko likuiditas.

2. **New Case Problem `.exb nb`: Perbandingan Dua Emiten IDX — UNVR vs TLKM (Self-Study)**
   Given: simplified 2023 data for both companies (from UNVR and TLKM data above)

   | Item | UNVR (Rp miliar) | TLKM (Rp miliar) |
   |---|---|---|
   | Total aset | 16.664 | 287.042 |
   | Total liabilitas | 13.283 | 130.480 |
   | Ekuitas | 3.381 | 156.562 |
   | Aset lancar | 6.192 | 55.613 |
   | Liabilitas lancar | 11.224 | 71.568 |
   | Pendapatan | 38.611 | 149.216 |
   | Laba bersih | 4.801 | 24.560 |

   Required (guidance only, no full solution):
   a. Hitung current ratio, D/E, ROA, ROE, net margin untuk kedua perusahaan
   b. Dari perspektif credit analysis: perusahaan mana yang lebih layak mendapat pinjaman bank?
   c. Dari perspektif equity analysis: perusahaan mana yang lebih menarik bagi investor?
   d. Jelaskan mengapa jawaban (b) dan (c) mungkin berbeda

### Section 7 — Cheat Sheet

**Add one new `.cc nb` card to the existing grid (after the EMH card):**

1. **"🇮🇩 Indonesian Context / Konteks Indonesia" card:**
   - PSAK = Pernyataan Standar Akuntansi Keuangan (berbasis IFRS)
   - OJK = Otoritas Jasa Keuangan (regulator pasar modal)
   - BEI = Bursa Efek Indonesia (bursa efek)
   - IHSG = Indeks Harga Saham Gabungan (Jakarta Composite Index)
   - NIM = Net Interest Margin (rasio perbankan)
   - LDR = Loan to Deposit Ratio (likuiditas bank)
   - CAR = Capital Adequacy Ratio (solvabilitas bank)
   - KSEI = Kustodian Sentral Efek Indonesia

---

## Data Sources (Verified 2023 Annual Reports)

### PT Unilever Indonesia Tbk (UNVR) — FY2023
| Item | Value (Rp miliar) |
|---|---|
| Total aset | 16.664 |
| Total liabilitas | 13.283 |
| Total ekuitas | 3.381 |
| Aset lancar | 6.192 |
| Liabilitas lancar | 11.224 |
| Pendapatan bersih | 38.611 |
| Beban pokok penjualan | 19.417 |
| Laba bruto | 19.195 |
| Laba usaha | 6.279 |
| Laba bersih | 4.801 |
| Arus kas operasi | 7.118 |
| Arus kas investasi | −829 |
| Arus kas pendanaan | −5.771 |
| Gross margin | 49,7% |
| Operating margin | 16,3% |
| Net margin | 12,4% |

### PT Telkom Indonesia Tbk (TLKM) — FY2023
| Item | Value (Rp miliar) |
|---|---|
| Total aset | 287.042 |
| Total liabilitas | 130.480 |
| Total ekuitas | 156.562 |
| Aset lancar | 55.613 |
| Liabilitas lancar | 71.568 |
| Pendapatan | 149.216 |
| Beban usaha | 104.300 |
| Laba usaha | 44.384 |
| Laba bersih | 24.560 |
| Operating margin | 29,7% |
| Net margin | 16,5% |
| ROA | 8,6% |
| ROE | 15,7% |

### PT Telkom Indonesia Tbk (TLKM) — FY2022 & FY2021 (for trend)
| Item | 2022 (T) | 2021 (T) |
|---|---|---|
| Pendapatan | 147,3 | 143,2 |
| Beban usaha | 107,7 | — |
| Laba usaha | 39,6 | — |
| Laba bersih | 20,8 | 24,8 |
| Total aset | 275,2 | 277,2 |

### PT Bank Central Asia Tbk (BBCA) — FY2023
| Item | Value |
|---|---|
| Total aset | Rp1.408 triliun |
| Total liabilitas | Rp1.166 triliun |
| Total ekuitas | Rp242 triliun |
| Laba bersih | Rp48,6 triliun |
| NIM | 5,5% |
| LDR | 70% |
| NPL | 1,9% |
| ROA | 3,6% |
| ROE | 21,5% |
| DPK (Dana Pihak Ketiga) | Rp1.102 triliun |
| Total kredit | Rp810,4 triliun |
| P/E | ~14× |
| P/B | ~3× |
| EPS | Rp467 |
| DPS | Rp336 |
| Dividend yield | ~4,5% |

Source URLs:
- UNVR: unilever.co.id annual report 2023
- TLKM: telkom.co.id investor relations, FY2023 audited FS
- BBCA: bca.co.id annual report 2023

---

## Cover Page Update

Update the cover info block:
- Change `Format: Bilingual (EN/ID) · Illustrated with Colgate-Palmolive data (2011)` to
  `Format: Bilingual (EN/ID) · Illustrated with Colgate-Palmolive (2011) & Indonesian Companies (2023)`

---

## Out of Scope

- Changing the primary language from bilingual to full Indonesian
- Replacing any Colgate-Palmolive textbook data
- Modifying formula boxes or theoretical content
- Adding content from chapters beyond Chapter 1
- Redesigning the overall page layout or color scheme
