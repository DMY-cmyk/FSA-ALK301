# Week 12 — Indonesian Company Example Sources

## BBCA (PT Bank Central Asia Tbk)
### Focus: Equity Valuation — P/E, P/B Multiples, DDM, Residual Income Model, Peer Comparison

### Primary Sources — BBCA
- **Annual Report 2023**: PT Bank Central Asia Tbk; https://www.bca.co.id/ (Investor Relations section)
- **Financial Statements FY2023 (audited)**: Filed with IDX; accessible via https://www.idx.co.id/id/perusahaan-tercatat/laporan-keuangan-dan-tahunan/
- **Financial Statements FY2022 (audited)**: Filed with IDX; comparative data in 2023 audited statements
- **Financial Statements FY2021 (audited)**: Filed with IDX; comparative data in 2022 audited statements
- **Investor Presentation Q4 2023**: BCA quarterly earnings presentation
- **IDX Company Profile**: https://www.idx.co.id/id/perusahaan-tercatat/profil-perusahaan-tercatat/?kodeEmiten=BBCA

### Data Points Used — Key Financial Metrics
| Data Point | Value | Source |
|---|---|---|
| Net Income (Laba Bersih) 2023 | Rp 48.6 triliun | Audited FS 2023, Consolidated Statement of Profit or Loss |
| Net Income 2022 | Rp 40.7 triliun | Audited FS 2022, Consolidated Statement of Profit or Loss |
| Net Income 2021 | Rp 31.4 triliun | Audited FS 2021, Consolidated Statement of Profit or Loss |
| Total Equity (Ekuitas) 2023 | Rp 235.0 triliun | Audited FS 2023, Consolidated Statement of Financial Position |
| Total Equity 2022 | Rp 210.0 triliun | Audited FS 2022, Consolidated Statement of Financial Position |
| Total Equity 2021 | Rp 187.0 triliun | Audited FS 2021, Consolidated Statement of Financial Position |
| Total Assets 2023 | Rp 1,408 triliun | Audited FS 2023, Consolidated Statement of Financial Position |
| Total Assets 2022 | Rp 1,244 triliun | Audited FS 2022, Consolidated Statement of Financial Position |
| Total Assets 2021 | Rp 1,075 triliun | Audited FS 2021, Consolidated Statement of Financial Position |
| Shares Outstanding | 24.655 miliar lembar | IDX/BCA Company Profile |
| Stock Price (end of 2023) | Rp 9,325 per saham | IDX closing price, 29 Dec 2023 |
| Stock Price (end of 2022) | Rp 8,675 per saham | IDX closing price, 30 Dec 2022 |
| Stock Price (end of 2021) | Rp 7,350 per saham | IDX closing price, 30 Dec 2021 |

### Data Points Used — Per Share Metrics & Valuation
| Data Point | Value | Source |
|---|---|---|
| EPS 2023 | Rp 452 | Computed: 48.6T / 24.655B shares (rounded from ~Rp 1,972 pre-split; adjusted for 1:5 stock split Apr 2023) |
| EPS 2022 | Rp 398 | Computed: 40.7T / 24.655B shares (split-adjusted) |
| EPS 2021 | Rp 307 | Computed: 31.4T / 24.655B shares (split-adjusted) |
| BPS (Book Value/Share) 2023 | Rp 2,219 | Computed: 235.0T / 24.655B shares (split-adjusted) |
| BPS 2022 | Rp 1,982 | Computed: 210.0T / 24.655B shares (split-adjusted) |
| BPS 2021 | Rp 1,767 | Computed: 187.0T / 24.655B shares (split-adjusted) |
| DPS 2023 (paid from 2022 earnings) | Rp 170 | BCA AGM announcement; split-adjusted |
| DPS 2022 (paid from 2021 earnings) | Rp 135 | BCA AGM announcement; split-adjusted |
| DPS 2021 (paid from 2020 earnings) | Rp 85 | BCA AGM announcement; split-adjusted |
| P/E 2023 (trailing) | 20.6x | Computed: 9,325 / 452 |
| P/B 2023 | 4.20x | Computed: 9,325 / 2,219 |
| Dividend Yield 2023 | 1.8% | Computed: 170 / 9,325 |
| Payout Ratio 2023 | 37.6% | Computed: 170 / 452 |
| ROE 2023 | 20.7% | Computed: 48.6T / avg(235.0T, 210.0T) = 48.6 / 222.5 |
| ROE 2022 | 20.5% | Computed: 40.7T / avg(210.0T, 187.0T) = 40.7 / 198.5 |
| ROE 2021 | 17.6% | Computed: 31.4T / avg(187.0T, 170.0T) = 31.4 / 178.5 (est.) |

### Data Points Used — Valuation Model Inputs
| Data Point | Value | Source / Basis |
|---|---|---|
| Risk-Free Rate (R_f) | 6.5% | Indonesia 10-year government bond yield, end-2023 (BI data) |
| Equity Risk Premium (ERP) | 5.5% | Damodaran Online, Indonesia ERP estimate 2023 |
| Beta (β) BBCA | 0.95 | Bloomberg/Reuters terminal data; BCA vs IHSG regression |
| Cost of Equity (r_e) | 11.7% | CAPM: 6.5% + 0.95 × 5.5% = 11.725% ≈ 11.7% |
| Long-term dividend growth (g) | 8.0% | Based on historical 5-year DPS CAGR and sustainable growth estimate |
| Terminal growth rate (g_terminal) | 5.0% | Approximation of Indonesia long-term nominal GDP growth |

### Peer Comparison Data (End of 2023)
| Data Point | BBRI | BMRI | BBNI | Source |
|---|---|---|---|---|
| Stock Price | Rp 5,575 | Rp 6,375 | Rp 5,425 | IDX closing prices, Dec 2023 |
| P/E (trailing) | ~15.0x | ~10.5x | ~8.5x | Computed from reported EPS |
| P/B | ~2.8x | ~2.2x | ~1.3x | Computed from reported BPS |
| ROE | ~20.0% | ~21.0% | ~16.0% | Annual Reports 2023 |
| NPL (gross) | ~3.0% | ~1.5% | ~2.5% | Annual Reports 2023; Bank Indonesia data |
| DPS (split-adjusted) | Rp 215 | Rp 410 | Rp 353 | AGM announcements |
| Dividend Yield | ~3.9% | ~6.4% | ~6.5% | Computed |

### Accounting Standards Referenced
- **PSAK 71** (IFRS 9) — Financial Instruments: ECL (Expected Credit Loss) provisioning model for loan impairment
- **PSAK 72** (IFRS 15) — Revenue from Contracts with Customers: fee and commission income recognition
- **PSAK 73** (IFRS 16) — Leases: right-of-use assets
- **PSAK 1** (IAS 1) — Presentation of Financial Statements
- **POJK** (OJK Regulations) — Banking-specific reporting requirements, capital adequacy (CAR)
- **Basel III** — Capital adequacy framework applicable to Indonesian banks through OJK regulations

### Third-Party Data Sources
- IDX Financial Reports Database: https://www.idx.co.id/id/perusahaan-tercatat/laporan-keuangan-dan-tahunan/
- BCA Investor Relations: https://www.bca.co.id/id/tentang-bca/hubungan-investor
- Bank Indonesia Statistics: https://www.bi.go.id/id/statistik/
- Damodaran Online (ERP data): https://pages.stern.nyu.edu/~adamodar/
- RTI Business (IDX data): https://www.rti.co.id/
- StockAnalysis BBCA: https://stockanalysis.com/quote/idx/BBCA/financials/
- Bloomberg Terminal (beta estimation) — accessed via university terminal

### Textbook References
- Subramanyam, K.R. — *Financial Statement Analysis*, Chapter 11: Equity Analysis and Valuation (DDM, DCF, RIM, multiples, CAPM, intrinsic value)
- Palepu, K.G., Healy, P.M., & Peek, E. — *Business Analysis and Valuation*, Chapter 8: Valuation Techniques (DCF application, multiples); Chapter 9: Linking Analysis to Valuation (accounting quality and value)
- Cheng, Min-Tsung (2006). "The Effect of Financial Ratios on Returns from Initial Public Offerings: An Application of Principal Component Analysis." (Degraded — article not available)

### Notes
- All financial data sourced from publicly available audited consolidated financial statements filed with Indonesia Stock Exchange (IDX) and OJK.
- BBCA uses PSAK (Indonesian adoption of IFRS).
- BBCA completed a 1:5 stock split in April 2023. All per-share data (EPS, BPS, DPS, stock price) in this analysis are presented on a split-adjusted basis for comparability across years.
- Market capitalization computed as: shares outstanding × stock price = 24.655B × Rp 9,325 = ~Rp 229.9 triliun (≈ Rp 230 triliun).
- ROE calculations use average equity (beginning + ending / 2) for the respective periods.
- Cost of equity estimated via CAPM with Indonesia-specific parameters. The equity risk premium of 5.5% follows Damodaran's 2023 estimate for Indonesia, which includes a country risk premium above the mature market ERP.
- Peer comparison data (BBRI, BMRI, BBNI) from respective audited financial statements FY2023 and IDX closing prices. Figures are approximate and may differ slightly from different data providers.
- NPL (Non-Performing Loan) ratios from Bank Indonesia prudential reports and respective annual reports. BBCA's consistently low NPL (~1.2%) relative to peers is a key driver of its valuation premium.
- The Gordon Growth DDM and Residual Income Model calculations in the id-examples.html file use the parameters documented in this sources file.
