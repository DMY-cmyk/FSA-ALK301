# Indonesian Localization Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Indonesian company comparison boxes (UNVR, TLKM, BBCA) alongside existing Colgate-Palmolive textbook examples in the Chapter 1 Study Guide HTML.

**Architecture:** Single-file HTML modification — add new `.id-box` CSS class and insert Indonesian company comparison `<div>` elements after corresponding Colgate boxes throughout the document. No build tools, no dependencies.

**Tech Stack:** Pure HTML/CSS. File: `Chapter1_StudyGuide_FSA_Subramanyam.html`

**Spec:** `docs/superpowers/specs/2026-04-01-indonesian-localization-design.md`

---

## Chunk 1: Foundation (CSS + Cover)

### Task 1: Add `.id-box` and `.idt` CSS Classes

**Files:**
- Modify: `Chapter1_StudyGuide_FSA_Subramanyam.html:94` (after `.term` class)

- [ ] **Step 1: Add CSS classes after line 94 (`.term` rule)**

Insert immediately after `}.term{color:var(--gold);font-weight:bold}` (line 94), before `</style>`:

```css
/* INDONESIAN COMPANY BOX */
.id-box{background:#FFF5F0;border:1px solid #E8A0A0;border-left:4px solid #C41E3A;padding:9px 13px;margin:9px 0;border-radius:0 5px 5px 0}
.idt{font-weight:bold;color:#8B1A2B;font-size:9.5pt;text-transform:uppercase;letter-spacing:.5px;margin-bottom:4px}
.id-box p,.id-box li{font-size:10pt;color:#444}
```

- [ ] **Step 2: Verify CSS was added**

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern "id-box" | Measure-Object`
Expected: Count ≥ 3 (the 3 CSS rules)

- [ ] **Step 3: Commit checkpoint — CSS foundation**

---

### Task 2: Update Cover Page Format Line

**Files:**
- Modify: `Chapter1_StudyGuide_FSA_Subramanyam.html:109`

- [ ] **Step 1: Change the Format line on line 109**

Replace:
```html
<p><strong>Format:</strong> Bilingual (EN/ID) · Illustrated with Colgate-Palmolive data (2011)</p>
```

With:
```html
<p><strong>Format:</strong> Bilingual (EN/ID) · Illustrated with Colgate-Palmolive (2011) & Indonesian Companies (2023)</p>
```

- [ ] **Step 2: Verify cover update**

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern "Indonesian Companies"`
Expected: 1 match on the cover line

---

## Chunk 2: Sections 2–3 (UNVR Business Analysis & Financial Statements)

### Task 3: Section 2 — UNVR Business Analysis + Regulatory Context

**Files:**
- Modify: `Chapter1_StudyGuide_FSA_Subramanyam.html:201` (after closing `</div>` of Colgate `.cg` box, before `</div>` closing `#s2`)

- [ ] **Step 1: Insert UNVR business analysis box and regulatory note after line 200**

Insert after `</ul></div>` (line 200, closing Colgate `.cg` box), before `</div>` (line 201, closing `#s2`):

```html

<div class="id-box nb"><div class="idt">🇮🇩 Unilever Indonesia (UNVR) — Perbandingan Bisnis</div>
<p><strong>Model bisnis:</strong> Portfolio consumer goods: Oral Care (Pepsodent), Personal Care (Lifebuoy, Dove, Sunsilk), Home Care (Rinso, Sunlight), Foods &amp; Refreshment (Bango, Wall's). IDX-listed sejak 1982 (ticker: UNVR), ~85% dimiliki Unilever NV.</p>
<p><strong>Paralel dengan Colgate:</strong> Keduanya adalah consumer goods, anak perusahaan global parent, dan menghadapi tekanan biaya bahan baku.</p>
<p><strong>Key insight:</strong> Revenue UNVR turun 6,3% (2022→2023), laba bersih turun 10,5% — tekanan serupa dengan Colgate yang mengalami stagnasi 2004–2006 akibat program restrukturisasi besar.</p>
</div>

<div class="def nb"><div class="den">Konteks Regulasi Indonesia</div><div class="did">Di Indonesia, laporan keuangan disusun berdasarkan <strong>PSAK</strong> (Pernyataan Standar Akuntansi Keuangan) yang berbasis IFRS, bukan US GAAP. Regulator utama adalah <strong>OJK</strong> (Otoritas Jasa Keuangan) dan <strong>Bursa Efek Indonesia</strong> (BEI/IDX).</div></div>
```

- [ ] **Step 2: Verify insertion**

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern "UNVR.*Perbandingan Bisnis"`
Expected: 1 match

---

### Task 4: Section 3 — UNVR Financial Statements (4 boxes)

**Files:**
- Modify: `Chapter1_StudyGuide_FSA_Subramanyam.html` — 4 insertion points in Section 3

- [ ] **Step 1: Insert UNVR Balance Sheet box after Colgate Balance Sheet (after line 235)**

Insert after `<p>Long-term debt: $4,430 · Treasury stock: ($12,808) — accumulated stock repurchases</p></div>` (line 235), before `</div>` (line 236, closing the `.nb` wrapper):

```html

<div class="id-box nb"><div class="idt">🇮🇩 UNVR — Neraca / Balance Sheet (2023)</div>
<p>Total aset: Rp16,7 triliun | Total liabilitas: Rp13,3 triliun | Ekuitas: Rp3,4 triliun</p>
<p>Aset lancar: Rp6,2 triliun (37%) · Aset tetap &amp; lainnya: Rp10,5 triliun (63%)</p>
<p>Liabilitas lancar: Rp11,2 triliun → <strong>working capital NEGATIF</strong> (Rp5,0 triliun)</p>
<p style="font-size:9.5pt;color:#555"><em>Perbandingan:</em> UNVR memiliki negative working capital (seperti Colgate yang thin WC), tetapi di UNVR ini bahkan lebih ekstrem — liabilitas lancar hampir 2× aset lancar.</p>
</div>
```

- [ ] **Step 2: Insert UNVR Income Statement box after Colgate IS table (after line 249)**

Insert after the closing `</div>` of the Income Statement `.nb` wrapper (line 250), before the Equity Statement section:

```html

<div class="id-box nb"><div class="idt">🇮🇩 UNVR — Laporan Laba Rugi / Income Statement (2023)</div>
<p>Pendapatan bersih: Rp38,6 triliun | Beban pokok penjualan: Rp19,4 triliun</p>
<p>Laba bruto: Rp19,2 triliun (<strong>gross margin 49,7%</strong> — lebih rendah dari Colgate 57,3%)</p>
<p>Laba usaha: Rp6,3 triliun (operating margin 16,3%) | Laba bersih: Rp4,8 triliun (net margin 12,4%)</p>
</div>
```

- [ ] **Step 3: Insert UNVR Cash Flow box after Colgate Cash Flow interpretation (after line 275)**

Insert after the Colgate Cash Flow Interpretation `.cg` box closing `</div>` (line 275), before the closing `</div>` (line 276) of the `.nb` wrapper:

```html

<div class="id-box nb"><div class="idt">🇮🇩 UNVR — Laporan Arus Kas / Cash Flow Statement (2023)</div>
<p><strong>Arus kas operasi:</strong> Rp7,1 triliun (positif — bisnis FMCG menghasilkan kas stabil)</p>
<p><strong>Arus kas investasi:</strong> −Rp0,8 triliun (belanja modal rutin)</p>
<p><strong>Arus kas pendanaan:</strong> −Rp5,8 triliun (pembayaran dividen sangat besar)</p>
<p style="font-size:9.5pt;color:#555">Pattern mirip Colgate: generate cash from operations → return to shareholders via dividends. UNVR memiliki dividend payout ratio mendekati ~100%, terlihat dari financing outflow yang hampir setara operating inflow.</p>
</div>
```

- [ ] **Step 4: Insert Indonesian reporting context after Articulation box (after line 283)**

Insert after the Articulation `.ibox` closing `</div>` (line 283), before `<h3>3.4` (line 285):

```html

<div class="id-box nb"><div class="idt">🇮🇩 Konteks Pelaporan di Indonesia</div>
<p><strong>OJK</strong> (Otoritas Jasa Keuangan): regulator pasar modal, setara SEC di AS.</p>
<p><strong>BEI</strong> (Bursa Efek Indonesia): bursa efek, setara NYSE.</p>
<p>Perusahaan publik wajib menyampaikan laporan keuangan tahunan yang diaudit dalam <strong>120 hari</strong> setelah akhir tahun buku.</p>
</div>
```

- [ ] **Step 5: Verify all 4 Section 3 boxes were inserted**

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern "🇮🇩 UNVR|🇮🇩 Konteks Pelaporan" | Measure-Object`
Expected: Count = 4

---

## Chunk 3: Sections 4–5 (TLKM, BBCA, IDX, EMH)

### Task 5: Section 4a-c — TLKM Comparative & Common-Size Analysis

**Files:**
- Modify: `Chapter1_StudyGuide_FSA_Subramanyam.html:360` (after Colgate common-size interpretation)

- [ ] **Step 1: Insert TLKM Year-to-Year and Common-Size boxes after line 360**

Insert after the Colgate common-size interpretation `<p>` closing `</div>` (line 360, closing `.cg.nb`), before `</div>` (line 361, closing `#s4ac`):

```html

<div class="id-box nb"><div class="idt">🇮🇩 PT Telkom Indonesia (TLKM) — Analisis Komparatif Year-to-Year (2022→2023)</div>
<table style="margin:6px 0"><thead><tr><th>Item</th><th>2023 (T)</th><th>2022 (T)</th><th>Change (T)</th><th>% Change</th></tr></thead>
<tbody>
<tr><td>Pendapatan</td><td>149,2</td><td>147,3</td><td>+1,9</td><td>+1,3%</td></tr>
<tr><td>Beban usaha</td><td>104,3</td><td>107,7</td><td>−3,4</td><td>−3,2%</td></tr>
<tr><td>Laba usaha</td><td>44,4</td><td>39,6</td><td>+4,8</td><td>+12,1%</td></tr>
<tr><td>Laba bersih</td><td>24,6</td><td>20,8</td><td>+3,8</td><td>+18,3%</td></tr>
</tbody></table>
<p style="font-size:9.5pt;color:#555"><em>Interpretasi:</em> Revenue tumbuh moderat (+1,3%), tapi efisiensi biaya menghasilkan pertumbuhan laba bersih +18,3% — mencerminkan keberhasilan program efisiensi, mirip Colgate yang mengkompensasi tekanan gross margin melalui penghematan SG&amp;A.</p>
</div>

<div class="id-box nb"><div class="idt">🇮🇩 PT Telkom Indonesia (TLKM) — Common-Size Income Statement (2023)</div>
<table style="margin:6px 0"><thead><tr><th>Item</th><th>2023 %</th></tr></thead>
<tbody>
<tr><td>Pendapatan</td><td>100,0%</td></tr>
<tr><td>Beban usaha</td><td>69,9%</td></tr>
<tr><td>Laba usaha</td><td>29,7%</td></tr>
<tr><td>Laba bersih</td><td>16,5%</td></tr>
</tbody></table>
<p style="font-size:9.5pt;color:#555">→ Operating margin TLKM (29,7%) lebih tinggi dari Colgate (22,95%) karena bisnis telekomunikasi memiliki operating leverage yang tinggi.</p>
</div>
```

- [ ] **Step 2: Verify TLKM boxes**

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern "TLKM" | Measure-Object`
Expected: Count ≥ 4

---

### Task 6: Section 4d — BBCA Ratio Comparison

**Files:**
- Modify: `Chapter1_StudyGuide_FSA_Subramanyam.html:429` (after Colgate ratio summary `.cg.nb` box)

- [ ] **Step 1: Insert BBCA ratio box after line 429**

Insert after `<p><strong>Market:</strong> P/E 18.55× dan P/B 17.05× mencerminkan premium yang diberikan pasar atas stabilitas, brand strength, dan track record Colgate.</p></div>` (line 429, closing Colgate `.cg.nb`), before `</div>` (line 430, closing `#s4d`):

```html

<div class="id-box nb"><div class="idt">🇮🇩 PT Bank Central Asia (BBCA) — Perbandingan Rasio Keuangan (2023)</div>
<p>Banking-specific ratios — berbeda dari perusahaan manufaktur:</p>
<table style="margin:6px 0"><thead><tr><th>Rasio</th><th>Formula / Keterangan</th><th>BBCA 2023</th></tr></thead>
<tbody>
<tr><td><strong>NIM</strong> (Net Interest Margin)</td><td>Net Interest Income / Avg Earning Assets</td><td><strong>5,5%</strong></td></tr>
<tr><td><strong>LDR</strong> (Loan to Deposit Ratio)</td><td>Total Kredit / DPK</td><td><strong>70%</strong></td></tr>
<tr><td><strong>NPL</strong> (Non-Performing Loan)</td><td>Kredit Bermasalah / Total Kredit</td><td><strong>1,9%</strong></td></tr>
<tr><td><strong>ROA</strong></td><td>Laba Bersih / Rata-rata Total Aset</td><td><strong>3,6%</strong></td></tr>
<tr><td><strong>ROE</strong></td><td>Laba Bersih / Rata-rata Total Ekuitas</td><td><strong>21,5%</strong></td></tr>
<tr><td><strong>D/E ratio</strong></td><td>Total Liabilitas / Total Ekuitas</td><td>Rp1.166T / Rp242T = <strong>4,82×</strong></td></tr>
</tbody></table>
<p style="font-size:9.5pt;color:#555"><strong>Key insight:</strong> D/E ratio BBCA (4,82×) lebih tinggi dari Colgate (4,01×), tetapi ini <em>normal</em> untuk bank. Bisnis perbankan adalah intermediasi: menghimpun dana masyarakat (DPK = Rp1.102 triliun) dan menyalurkannya sebagai kredit (Rp810 triliun). Leverage tinggi bukan tanda risiko jika NPL rendah dan CAR memadai.</p>
</div>
```

- [ ] **Step 2: Verify BBCA box**

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern "BBCA" | Measure-Object`
Expected: Count ≥ 2

---

### Task 7: Section 4e — IDX Valuation Context

**Files:**
- Modify: `Chapter1_StudyGuide_FSA_Subramanyam.html:480` (after Labrador/Pitbull worked example)

- [ ] **Step 1: Insert IDX valuation box after line 480**

Insert after `→ Ketiga model menghasilkan nilai yang sama karena secara matematis ekuivalen...</p></div>` (line 480, closing `.cg.nb`), before `</div>` (line 481, closing `#s4e`):

```html

<div class="id-box nb"><div class="idt">🇮🇩 Konteks Valuasi di Bursa Efek Indonesia (IDX)</div>
<p><strong>BBCA market measures (2023):</strong> P/E ~14×, P/B ~3×, EPS Rp467, DPS Rp336, Dividend yield ~4,5%</p>
<p><strong>Perbandingan:</strong> Colgate P/E 18,55× vs BBCA ~14×; Colgate P/B 17,05× vs BBCA ~3×</p>
<p>Perbedaan P/B dijelaskan oleh: (1) book value bank lebih dekat ke economic value karena aset kebanyakan finansial, (2) UNVR/Colgate memiliki intangible value (brand) yang jauh melampaui book value.</p>
<p><strong>IHSG</strong> (Jakarta Composite Index) average P/E: ~12–15×, lebih rendah dari S&amp;P 500 (~20–25×) karena <em>emerging market risk premium</em>.</p>
</div>
```

- [ ] **Step 2: Verify IDX valuation box**

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern "IHSG" | Measure-Object`
Expected: Count ≥ 1

---

### Task 8: Section 5 — Indonesian Market Efficiency

**Files:**
- Modify: `Chapter1_StudyGuide_FSA_Subramanyam.html:530` (after EMH implications `.cg.nb` box)

- [ ] **Step 1: Insert Indonesian market efficiency box after line 530**

Insert after the Warren Buffett/EMH implications `.cg.nb` box closing `</div>` (line 530), before `</div>` (line 531, closing `#s5`):

```html

<div class="id-box nb"><div class="idt">🇮🇩 Efisiensi Pasar Indonesia (IDX/BEI)</div>
<p>BEI umumnya dianggap <strong>semistrong efficient</strong> untuk saham LQ45 (likuid), tetapi <em>less efficient</em> untuk saham small-cap.</p>
<p><strong>OJK</strong> berperan setara SEC: menegakkan keterbukaan informasi (full disclosure).</p>
<p><strong>Anomali spesifik di IDX:</strong></p>
<ul>
<li><strong>Ramadan effect:</strong> beberapa studi menunjukkan return positif abnormal selama bulan Ramadan (unik untuk pasar Muslim-majority)</li>
<li><strong>January effect:</strong> ditemukan di IDX meski magnitude-nya lebih kecil dari pasar AS</li>
<li><strong>Post-earnings drift:</strong> juga ditemukan di IDX, menunjukkan pasar tidak langsung fully efficient</li>
</ul>
<p>Regulasi insider trading: UU Pasar Modal No. 8/1995, diperkuat oleh UU OJK No. 21/2011.</p>
<p style="font-size:9.5pt;color:#555"><em>Implikasi:</em> FSA tetap sangat berguna di pasar Indonesia karena banyak perusahaan IDX yang kurang diikuti analis (under-covered) dibandingkan NYSE.</p>
</div>
```

- [ ] **Step 2: Verify EMH Indonesian box**

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern "Ramadan effect" | Measure-Object`
Expected: Count = 1

---

## Chunk 4: Sections 6–7 + Final Validation

### Task 9: Section 6 — New Indonesian Practice Exercises

**Files:**
- Modify: `Chapter1_StudyGuide_FSA_Subramanyam.html:654-655` (after Case 1-3 closing, before Section 7)

- [ ] **Step 1: Insert TLKM ratio exercise after line 654**

Insert after `</div>` (line 654, closing the Case 1-3 `.exb`), before `</div>` (line 655, closing `#s6`):

```html

<h3>6.4 Indonesian Company Exercises / Latihan Perusahaan Indonesia</h3>

<div class="exb nb">
<div class="ext">🇮🇩 Exercise: Analisis Rasio — PT Telkom Indonesia (TLKM) 2023</div>
<p>Given data (simplified from actual 2023 annual report):</p>
<table style="margin:6px 0"><thead><tr><th>Item</th><th>2023 (Rp miliar)</th></tr></thead>
<tbody>
<tr><td>Total aset</td><td>287.042</td></tr>
<tr><td>Total liabilitas</td><td>130.480</td></tr>
<tr><td>Total ekuitas</td><td>156.562</td></tr>
<tr><td>Aset lancar</td><td>55.613</td></tr>
<tr><td>Liabilitas lancar</td><td>71.568</td></tr>
<tr><td>Pendapatan</td><td>149.216</td></tr>
<tr><td>Beban usaha</td><td>104.300</td></tr>
<tr><td>Laba usaha</td><td>44.384</td></tr>
<tr><td>Laba bersih</td><td>24.560</td></tr>
</tbody></table>
<p><strong>Required:</strong></p>
<ol type="a">
<li>Current ratio</li>
<li>Debt-to-equity ratio</li>
<li>Operating profit margin (gunakan laba usaha / pendapatan)</li>
<li>Net profit margin</li>
<li>ROA dan ROE (gunakan ending balances)</li>
<li>Interpretasi: apakah TLKM lebih sehat dari Colgate (2011)? Bandingkan minimal 3 rasio.</li>
</ol>
<div class="sol"><div class="solt">Solution / Solusi</div>
<p><strong>a. Current ratio:</strong> 55.613 / 71.568 = <strong>0,78×</strong> (di bawah 1,0 → working capital negatif)</p>
<p><strong>b. D/E ratio:</strong> 130.480 / 156.562 = <strong>0,83×</strong> (lebih rendah dari Colgate 4,01× → leverage jauh lebih konservatif)</p>
<p><strong>c. Operating margin:</strong> 44.384 / 149.216 = <strong>29,7%</strong> (lebih tinggi dari Colgate 22,95%)</p>
<p><strong>d. Net margin:</strong> 24.560 / 149.216 = <strong>16,5%</strong> (lebih tinggi dari Colgate 14,53%)</p>
<p><strong>e. ROA:</strong> 24.560 / 287.042 = <strong>8,6%</strong> | <strong>ROE:</strong> 24.560 / 156.562 = <strong>15,7%</strong></p>
<p><strong>f. Interpretasi:</strong> TLKM memiliki operating margin dan net margin lebih tinggi dari Colgate, tetapi ROE jauh lebih rendah (15,7% vs 45,37%) karena leverage Colgate yang sangat tinggi memperbesar return on equity. Current ratio TLKM (0,78×) lebih rendah dari Colgate (1,18×), tetapi arus kas operasi yang kuat memitigasi risiko likuiditas.</p>
</div>
</div>

<div class="exb nb">
<div class="ext">🇮🇩 Case Problem: Perbandingan Dua Emiten IDX — UNVR vs TLKM (Self-Study)</div>
<p>Data berikut disederhanakan dari laporan keuangan tahunan 2023:</p>
<table style="margin:6px 0"><thead><tr><th>Item</th><th>UNVR (Rp miliar)</th><th>TLKM (Rp miliar)</th></tr></thead>
<tbody>
<tr><td>Total aset</td><td>16.664</td><td>287.042</td></tr>
<tr><td>Total liabilitas</td><td>13.283</td><td>130.480</td></tr>
<tr><td>Ekuitas</td><td>3.381</td><td>156.562</td></tr>
<tr><td>Aset lancar</td><td>6.192</td><td>55.613</td></tr>
<tr><td>Liabilitas lancar</td><td>11.224</td><td>71.568</td></tr>
<tr><td>Pendapatan</td><td>38.611</td><td>149.216</td></tr>
<tr><td>Laba bersih</td><td>4.801</td><td>24.560</td></tr>
</tbody></table>
<div class="ibox" style="margin-top:8px"><div class="ibt">Panduan Analisis / Student Guidance</div>
<p><strong>a.</strong> Hitung current ratio, D/E, ROA, ROE, net margin untuk kedua perusahaan.</p>
<p><strong>b.</strong> Dari perspektif credit analysis: perusahaan mana yang lebih layak mendapat pinjaman bank?</p>
<p><strong>c.</strong> Dari perspektif equity analysis: perusahaan mana yang lebih menarik bagi investor?</p>
<p><strong>d.</strong> Jelaskan mengapa jawaban (b) dan (c) mungkin berbeda.</p>
<p style="margin-top:6px;font-style:italic;color:#555">Petunjuk: UNVR memiliki laba bersih lebih kecil, tetapi periksa ROA dan ROE-nya — apakah ukuran perusahaan selalu menentukan kualitas investasi?</p></div>
</div>
```

- [ ] **Step 2: Verify exercises were inserted**

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern "6\.4 Indonesian" | Measure-Object`
Expected: Count = 1

---

### Task 10: Section 7 — Indonesian Context Cheat Sheet Card

**Files:**
- Modify: `Chapter1_StudyGuide_FSA_Subramanyam.html:756` (after EMH card, inside `.cgrid`)

- [ ] **Step 1: Insert Indonesian context card after EMH card (after line 756)**

Insert after `</div>` (line 756, closing the EMH `.cc.nb` card), before `</div>` (line 758, closing `.cgrid`):

```html

<div class="cc nb">
<h4>🇮🇩 Indonesian Context</h4>
<p><strong>PSAK</strong> = Standar Akuntansi (IFRS-based)<br>
<strong>OJK</strong> = Otoritas Jasa Keuangan (≈ SEC)<br>
<strong>BEI</strong> = Bursa Efek Indonesia (≈ NYSE)<br>
<strong>IHSG</strong> = Jakarta Composite Index<br>
<strong>NIM</strong> = Net Interest Margin (bank)<br>
<strong>LDR</strong> = Loan to Deposit Ratio (bank)<br>
<strong>CAR</strong> = Capital Adequacy Ratio (bank)<br>
<strong>KSEI</strong> = Kustodian Sentral Efek Indonesia</p>
</div>
```

- [ ] **Step 2: Verify cheat sheet card**

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern "Indonesian Context" | Measure-Object`
Expected: Count = 1

---

### Task 11: Final Validation

- [ ] **Step 1: Count all `.id-box` instances in the file**

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern 'class="id-box' | Measure-Object`
Expected: Count = 10 (S2: 1 UNVR, S3: 4 boxes, S4a-c: 2 TLKM, S4d: 1 BBCA, S4e: 1 IDX, S5: 1 EMH)

- [ ] **Step 2: Verify HTML structure — single body/html close**

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern "</body>" | Measure-Object`
Expected: Count = 1

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern "</html>" | Measure-Object`
Expected: Count = 1

- [ ] **Step 3: Verify no existing content was modified**

Run: `Select-String -Path "Chapter1_StudyGuide_FSA_Subramanyam.html" -Pattern "Colgate-Palmolive|Colgate Balance Sheet|Colgate Ratio Analysis" | Measure-Object`
Expected: Count unchanged (all original Colgate references still present)

- [ ] **Step 4: Open in browser for visual verification**

Run: `Start-Process "Chapter1_StudyGuide_FSA_Subramanyam.html"`
Verify: Red-bordered Indonesian boxes appear after gold Colgate boxes in each section.

---

## Notes

- All insertions are ADDITIVE — no existing lines are deleted or modified (except the cover Format line)
- The `.id-box` visual pattern (red left border on cream background) contrasts with `.cg` (gold left border on warm yellow background)
- All `.id-box` elements carry `nb` class for print page-break-inside avoidance
- Financial data sourced from verified 2023 annual reports (UNVR, TLKM, BBCA)
- Rupiah formatting uses Indonesian notation: period for thousands, comma for decimal (e.g., Rp16,7 triliun)
