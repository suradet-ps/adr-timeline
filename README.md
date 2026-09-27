# ADR Timeline Generator

[![Deploy](https://github.com/suradet-ps/adr-timeline/actions/workflows/deploy.yml/badge.svg)](https://github.com/suradet-ps/adr-timeline/actions/workflows/deploy.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust: stable](https://img.shields.io/badge/rust-stable-orange.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Leptos v0.8](https://img.shields.io/badge/Leptos-v0.8-blue.svg)](https://leptos.dev)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/suradet-ps/adr-timeline/issues)

---

## ◆ PULSE

Every adverse drug reaction has a moment. This tool makes it visible -
and makes the causality argument legible. A pharmacist logs the drugs, the
reaction, and the lab findings on one A4-shaped timeline, then lets
structured judgment take over: Naranjo for causality, DRESS RegiSCAR for
syndrome severity, and a Lund-Browder BSA estimate when SJS/TEN is in the
room. All in the browser, all in a warm parchment document a clinician
would not mind printing.

| v1.2.0 ▣ | Naranjo ▣ | DRESS ▣ | BSA ▣ | PDF ▣ |
|---|---|---|---|---|

*The core record - drugs, reactions, labs, scores, export - is sealed.
The design language is parchment and ink, not dashboard blue.*

> Built with Rust 2024 + Leptos 0.8, drawn on canvas, printed through
> jsPDF - no server, no backend, no patient data leaving the browser.
>
> **suradet-ps**, artifact keeper

---

## ◆ IGNITION

One target, one tool, one command.

```
⟫ rustup target add wasm32-unknown-unknown
⟫ cargo install --locked trunk
⟫ trunk serve --open
```

The release artifact:

```
⟫ trunk build --release
```

The bundle lands in `dist/` - push to `main` or `master` and the deploy
workflow ships it to GitHub Pages.

<details>
<summary>Prerequisites</summary>

- [Rust](https://rustup.rs/) (stable toolchain, edition 2024)
- [Trunk](https://trunkrs.dev/) - installed above

</details>

---

## ◆ ANATOMY

One screen, several honest functions, zero backend.

- **Draws** - an A4-sized multi-page canvas renders drug bars, reaction
  markers, and lab events on a shared clinical axis; page breaks land
  where paper would.
- **Tracks** - each drug carries dose, start and end dates, ongoing
  status, or an honest "unknown start" - a reaction timeline that hides
  nothing about what it does not know.
- **Judges** - the Naranjo algorithm returns Definite, Probable,
  Possible, or Doubtful; the DRESS RegiSCAR score weighs structured
  criteria; the BSA calculator applies the Lund-Browder method when
  SJS/TEN demands it. All three are deterministic and documented.
- **Saves** - patient cases round-trip as JSON through `localStorage` and
  file import/export - the record belongs to the pharmacist, not to a
  cloud.
- **Prints** - PDF export via jsPDF produces print-ready A4 reports for
  consultation notes and case discussions.
- **Warms** - the editorial design system in `DESIGN.md`: parchment
  surfaces, terracotta accents, serif headings, and ring-shadow buttons -
  clinical clarity with human warmth.

---

## ◆ RITUALS

**The core ceremony** - one patient, one timeline, one export:

1. Enter the drugs: dose, start and end dates, ongoing, or unknown.
2. Log the reaction with its onset date and description.
3. Record the lab investigations where they happened on the axis.
4. Run the scores: Naranjo for causality, RegiSCAR for DRESS, BSA when
   skin is at risk.
5. Export the PDF or save the JSON case - the chart is the testimony.

**The ceremony of honesty** - an unknown start date is marked unknown;
a doubtful Naranjo is reported doubtful. The tool scores, it does not
advocate.

**The ceremony of privacy** - everything stays in the browser: cases in
`localStorage`, exports from a canvas, scores from pure math. No server
sees a single dose.

---

## ◆ ECHOES

**Where this artifact is heading**

```
core record ▸ timeline, drugs, reactions, labs ─────────────────── ▸ sealed
judgment    ▸ Naranjo, DRESS RegiSCAR, BSA (Lund-Browder) ──────── ▸ sealed
delivery    ▸ JSON round-trip, A4 PDF export, GitHub Pages ──────── ▸ sealed
```

**Raising the artifact** - the design language is specified in
`DESIGN.md`; the hard rules live in `AGENTS-RUST.md`. Mandatory gates
before any PR: `cargo fmt --check`, `cargo check`,
`cargo clippy -- -D warnings`, `cargo test`. Keep changes surgical.

**Status** - pushing to `main` or `master` triggers the
[deploy workflow](.github/workflows/deploy.yml) and the artifact goes
live on GitHub Pages.

> Built for clinical pharmacists, by pharmacists.
> Not a substitute for professional medical judgment.

---

```
  ─────────────────────────────────────────
   Every reaction has a moment.
   The timeline makes it visible.
  ─────────────────────────────────────────
```

Licensed under the [MIT License](LICENSE).