# ADR Timeline Generator

[![Version](https://img.shields.io/badge/version-1.2.0-blue)](https://github.com/suradet-ps/adr-timeline/releases)
[![Rust](https://img.shields.io/badge/rust-edition_2024-orange)](https://www.rust-lang.org)
[![WASM](https://img.shields.io/badge/target-WASM-red)](https://webassembly.org)
[![Leptos](https://img.shields.io/badge/leptos-0.8-purple)](https://leptos.dev)
[![Build](https://img.shields.io/badge/build-trunk-brightgreen)](https://github.com/trunk-rs/trunk)

> **Drug Exposure & Adverse Drug Reaction Timeline** — A professional clinical tool for pharmacists to visualize medication timelines, document adverse reactions, and perform causality assessments. Built with Rust + WebAssembly.

---

##  Features

-  **Interactive Timeline Canvas** — A4-sized, multi-page canvas rendering with drug bars, reaction markers, and lab events
-  **Drug Entry** — Track medications with dose, start/end dates, ongoing status, and unknown start dates
-  **ADR Documentation** — Log adverse reactions with onset dates and descriptions
-  **Lab Results** — Record laboratory investigations on the timeline
-  **Naranjo Algorithm** — Built-in causality assessment for ADRs (Definite/Probable/Possible/Doubtful)
-  **DRESS RegiSCAR Score** — Evaluate DRESS syndrome likelihood with structured criteria
-  **BSA Calculator** — Lund-Browder method for body surface area estimation (SJS/TEN assessment)
-  **JSON Import/Export** — Save and load patient cases locally
-  **PDF Export** — Generate print-ready A4 reports via jsPDF
-  **Editorial Design System** — Warm parchment aesthetic inspired by Claude/Anthropic, optimized for clinical documentation

---

##  Tech Stack

| Component | Technology |
|-----------|-----------|
| **Language** | Rust 2024 Edition |
| **Frontend** | Leptos 0.8 (CSR) |
| **WASM** | `wasm-bindgen`, `wasm32-unknown-unknown` |
| **Build Tool** | Trunk |
| **Storage** | `gloo-storage` (LocalStorage) |
| **File I/O** | `gloo-file`, `web-sys` |
| **Serialization** | `serde`, `serde_json` |
| **PDF Export** | jsPDF (via CDN) |
| **Styling** | CSS Variables + Design Tokens |

---

##  Quick Start

### Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# WASM target
rustup target add wasm32-unknown-unknown

# Trunk (build tool)
cargo install --locked trunk
```

### Development

```bash
# Clone and enter project
git clone https://github.com/suradet-ps/adr-timeline.git
cd adr-timeline

# Start dev server (hot-reload enabled)
trunk serve --open
```

### Production Build

```bash
# Build optimized WASM bundle
trunk build --release

# Output: ./dist/ (ready for static hosting)
```

### Deploy to GitHub Pages

```bash
# Enable GitHub Pages in repo settings
# Push to main/master branch — CI/CD via .github/workflows/deploy.yml handles the rest
```

---

##  Design System

This project implements a warm, editorial-inspired UI based on the [Claude/Anthropic design language](./DESIGN.md):

- **Palette**: Parchment (`#f5f4ed`), Terracotta (`#c96442`), Warm Neutrals
- **Typography**: Serif headings (Georgia), Sans UI (Sarabun), Mono for code
- **Components**: Ring-shadow buttons, ivory cards, organic illustrations
- **Philosophy**: Literary pacing, human warmth, clinical clarity

> All CSS variables are defined in `:root` for easy theming. See `style.css` for tokens.

---

##  Contributing

1. Follow guidelines in [`AGENTS-RUST.md`](./AGENTS-RUST.md)
2. Run mandatory checks before PR:
   ```bash
   cargo fmt --check && cargo check && cargo clippy -- -D warnings && cargo test
   ```
3. Keep changes surgical — touch only what the issue requires
4. Document public APIs with `///` and include examples

---

##  License

MIT License — see `LICENSE` for details.

> Built for clinical pharmacists, by pharmacists.  
> Not a substitute for professional medical judgment.
