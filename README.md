# Temple – Template Manager CLI

> **🚧 Project Status: Pre‑Alpha** – This repository is still empty of code. We’re publishing the vision early so folks can follow along, give feedback, or pitch in. **Nothing here is runnable yet.**

---

## What Is Temple?

**Temple** is a single‑binary command‑line tool that lets developers, data scientists, and analysts:

* **Browse & search** ready‑to‑use project templates (Jupyter notebooks, dbt models, Dash apps, SQL pipelines, etc.) from one or many git‑backed catalogs.
* **Initialize** a fresh project with `temple init …` – variables substituted, startup scripts executed, and best‑practice scaffolding in place within seconds.
* **Contribute** new templates with `temple submit` – including automated validation, metadata linting, and an optional pull‑request flow.

Think of Temple as **`cargo new` / `npm init` / `cookiecutter`** on steroids, but language‑agnostic, multi‑repository, and designed for **teams that mix software engineering with data work**.

---

## Planned Core Features

| Area | Highlights |
|------|------------|
| **CLI UX** | `temple list`, `temple search`, `temple init`, `temple submit`, `temple sync`, `temple doctor` and more – all with rich `--json`, `--yaml`, `--long`, and interactive modes. |
| **Template Spec** | Each template contains a `metadata.yaml`, `files/` skeleton, optional `startup.sh`, and declarative `prompts.yaml` for variables. Templates live in any Git repo (public or private). |
| **Discovery & Caching** | Local FTS catalog powered by SQLite; `temple sync` pulls delta updates via libgit2. Multiple remote indexes merge seamlessly. |
| **Initialization** | Jinja‑style rendering (`tera`), safe overwrite handling, one‑shot or non‑interactive CI flows, optional post‑install shell script. |
| **Validation & Submission** | `temple submit` lints metadata, runs template self‑tests, and opens a pre‑filled PR. Central CI regenerates `index.json`. |
| **Extensibility** | Tag‑based filtering, pluggable render helpers, future signed templates, plugin hooks (`pre_init` / `post_init`). |

---

## Roadmap (High‑Level)

| Milestone | Target | Description |
|-----------|--------|-------------|
| **0.0.1** | ✅ *This doc* | Publish design RFC & README. |
| **0.1.0** | May 2025 | Rust CLI skeleton (`clap`), local template init from bundled examples. |
| **0.2.0** | Jun 2025 | Remote Git sync, search, cache pruning, self‑update. |
| **0.3.0** | Aug 2025 | Template submission workflow, validation CI, first community templates (dbt, Jupyter). |
| **0.4.0** | Q4 2025 | Plugin system, multi‑repo priority rules, signed templates. |
| **1.0.0** | 2026 | Stable CLI, strong backward compatibility guarantees. |

Dates are **aspirational** and will shift as we learn.

---

## Tech Stack (Subject To Change)

* **Rust** 1.85+ for performance & static binaries
* [`clap`](https://github.com/clap-rs/clap) for argument parsing
* [`tera`](https://github.com/Keats/tera) for template rendering
* **libgit2** via [`git2`](https://github.com/rust-lang/git2-rs) for Git operations
* **sled** + **SQLite** for local caching & full‑text search
* GitHub Actions for CI, release, and index generation

An earlier prototype might appear for rapid experimentation.

---

## Contributing Early Feedback

* **Ideas & Discussion:** Open a GitHub Discussion or join the upcoming `#temple-dev` Slack channel.
* **Issues:** File an issue tagged **`design‑review`** for anything spec‑related.
* **Pull Requests:** We’ll accept spec PRs (docs, examples) even before code exists. Code PRs welcome once the initial skeleton lands.

We aim for an open, inclusive contributor experience. The project will be released under the **MIT License**.

---

## Inspiration & Prior Art

* [cookiecutter](https://github.com/cookiecutter/cookiecutter) – Python project scaffolding.
* [cargo‑generate](https://github.com/cargo-generate/cargo-generate) – Rust template manager.
* [Yeoman](https://yeoman.io/) – web‑app scaffolding.
* Internal template tooling we’ve built ad‑hoc across data teams.

Temple combines the strengths of these tools, adds robust Git‑native cataloging, and targets the cross‑disciplinary workflows common in modern data/ML teams.

---

### Call for Early Feedback

If any of the above sparks interest—or you’re tired of copying boilerplate between repos—**star this repo and chime in**. Real code is coming soon. 💫

