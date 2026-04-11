# Design Spec: Print Layout Polish — Chapter 1 Study Guide

**Source file:** `Chapter1_StudyGuide_FSA_Subramanyam.html`
**Date:** 2026-04-01
**Objective:** Tighten spacing, prevent content splitting across pages, minimize total page count when printing/saving as PDF.

---

## Constraints

- Do NOT change any text content
- Do NOT change colors, fonts, or visual identity
- Do NOT remove any elements
- Changes are CSS spacing/print rules + minimal HTML structure wrapping

---

## CSS Changes

### 1. Body & Global Spacing (tighten)

| Property | Before | After |
|----------|--------|-------|
| `body` line-height | 1.6 | 1.45 |
| `p` margin | 5px 0 | 3px 0 |
| `ul,ol` margin | 5px 0 | 3px 0 |
| `li` margin | 3px 0 | 2px 0 |
| `.section` padding | 16px 0 | 8px 0 |
| `.phdr` margin-bottom | 14px | 10px |

### 2. Headings (reduce top/bottom margins)

| Selector | Before | After |
|----------|--------|-------|
| `h2` margin | 18px 0 10px | 10px 0 6px |
| `h3` margin | 14px 0 6px | 8px 0 4px |
| `h4` margin | 10px 0 4px | 6px 0 3px |

### 3. Content Boxes (reduce padding & margin)

| Selector | Padding Before | Padding After | Margin Before | Margin After |
|----------|---------------|---------------|---------------|--------------|
| `.def` | 9px 13px | 7px 11px | 7px 0 | 5px 0 |
| `.cg` | 9px 13px | 7px 11px | 9px 0 | 5px 0 |
| `.id-box` | 9px 13px | 7px 11px | 9px 0 | 5px 0 |
| `.fbox` | 9px 13px | 7px 11px | 7px 0 | 5px 0 |
| `.ibox` | 9px 13px | 7px 11px | 9px 0 | 5px 0 |
| `.exb` | 11px 13px | 8px 11px | 11px 0 | 6px 0 |
| `.qb` | 9px 13px | 7px 11px | 5px 0 | 4px 0 |
| `.sol` | 7px 11px | 6px 9px | 7px 0 (top) | 5px 0 (top) |

### 4. Tables (tighter cells)

| Property | Before | After |
|----------|--------|-------|
| `table` margin | 7px 0 | 5px 0 |
| `th` padding | 6px 8px | 4px 6px |
| `td` padding | 5px 8px | 3px 6px |

### 5. Cover Page (compact)

| Property | Before | After |
|----------|--------|-------|
| `.cover` padding | 70px 40px 50px | 40px 30px 30px |
| `.cauth` margin-bottom | 22px | 12px |
| `.badge` margin-bottom | 18px | 10px |
| `.cdiv` margin | 16px auto | 8px auto |
| `.cinfo` padding | 14px 22px | 10px 16px |
| `.cmap` margin | 18px auto | 10px auto |
| `.olist li` padding | 4px 0 4px 24px | 3px 0 3px 24px |

### 6. Cheat Sheet Grid (tighter)

| Property | Before | After |
|----------|--------|-------|
| `.cgrid` gap | 9px | 6px |
| `.cc` padding | 8px 9px | 6px 7px |

### 7. Print-Specific Rules (add to `@media print`)

```css
h2,h3,h4{page-break-after:avoid}
.def,.cg,.id-box,.fbox,.ibox,.exb,.qb,.sol,.cc{page-break-inside:avoid}
table{page-break-inside:avoid}
.phdr{margin-bottom:8px}
```

---

## HTML Structure Changes

Wrap these heading+content pairs in `<div class="nb">` where not already wrapped, to prevent headings from orphaning at page bottoms:

1. `<h3>3.3 Links Between Financial Statements` + the `.ibox` articulation box + the `.id-box` Indonesian reporting context (already exists in file)
2. `<h3>3.4 Additional Information Sources` + the table after it
3. `<h3>4.2 Comparative` + the paragraph after it
4. `<h3>4.3 Common-Size` + the paragraph + `.ibox` after it
5. `<h3>5.2 The EMH Paradox` + the `.ibox` after it
6. `<h3>5.3 Market Anomalies` + the table after it
7. `<h3>6.1 Discussion Questions` — wrap only the heading + Q1-1 together (NOT all 3 questions, to avoid oversized no-break block)

Only wrap where the heading is NOT already inside an `nb` div.

---

## Out of Scope

- Changing content text, financial data, or translations
- Changing the color scheme or visual identity
- Rearranging section order
- Adding/removing sections
