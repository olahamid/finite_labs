# Finite Labs — Product Requirements Document (MVP)

**Version:** 0.4 (draft)
**Owner:** Finite Labs founder
**Status:** In progress
**Last updated:** 11 November 2026

### Changes in 0.4
- Products and education carry equal weight (Section 1). Products lead slightly in order; education is never secondary.
- The Playground sits under Learn (navigation, homepage, `/learn/playground` URL).
- Homepage restructured (Section 5.1): new hero subline, "Get early access" and "Start learning" as equal-size buttons, a Products section, and five audience cards.
- New hero "Live proof" widget: real SHA-256 and Ed25519 in the browser via Rust/WebAssembly, with a Tamper toggle (Sections 5.1, 5.4). This brings part of M3 forward.
- Visual details (Section 7): JetBrains Mono for technical text, real-hash hex texture on cards, a dot-lattice hero background.
- Audience list updated to five groups (Section 4).

### Changes in 0.3
- New mission statement (Section 1).
- Landing page, navigation and footer structure now take inspiration from pqshield.com (Section 5.1), adapted to an education-first MVP.
- Copy tone guidance updated to match (Section 7).

### Changes in 0.2
- **Rust instead of JavaScript.** The site is a server-rendered Rust app (Axum + Askama), the same pattern as the founder's portfolio. The interactive demos are Rust compiled to WebAssembly. Next.js, Tailwind, MDX, Supabase client, KaTeX, Mermaid and Playwright are removed.
- Content uses Markdown with a `+++` TOML frontmatter block instead of MDX.
- The open questions in Section 11 are updated with the decisions made so far.

---

## 1. Vision and mission

**Mission:** Finite Labs builds real-world cryptographic solutions, through software, hardware, education, and research, to put ownership and control back in your hands.

**Long-term vision:** Finite Labs becomes Africa's leading cryptography company, offering real-world cryptographic software, hardware, education, and independent research, built in Nigeria for the world.

**Company pillars (long term):**

1. **Education:** courses, articles, and learning paths on cryptography.
2. **Cryptographic tools and products:** including security for AI systems (signing and verifying model outputs, content provenance, protecting model weights and data, privacy-preserving ML, secrets management for AI agents).
3. **Independent research:** public papers, analyses, and open-source work.
4. **Hardware:** later stage, likely through design licensing or partnerships.

**Products and education carry equal weight.** Products lead slightly in ordering (navigation, homepage), but education is a core offering in its own right, never support material for products. Future paid courses and corporate training are education products.

**This PRD covers the MVP only: the education platform plus a research publication section.** The site presents products as "in development" with an early-access call to action. Shipping products and hardware come in later phases (see Section 10).

---

## 2. Problem

- Cryptography expertise in Nigeria and across Africa is scarce. Most local companies use cryptography without understanding it, and most learning material assumes a Western context and a strong math background.
- Banks, fintechs, and companies covered by the Nigeria Data Protection Act (NDPA) 2023 need security-literate staff, but there are few local, practical training options.
- There is no trusted African voice publishing independent cryptography research.

## 3. Goals (MVP)

1. Launch a public website that establishes Finite Labs as a credible cryptography brand.
2. Publish educational articles and structured learning paths.
3. Offer interactive, in-browser cryptography demos that make concepts tangible.
4. Publish research notes and papers.
5. Capture an audience (newsletter sign-ups, accounts) to support future paid courses and products.

### Non-goals (MVP)

- No paid courses or payments yet (planned for Phase 2).
- No production cryptographic products for customers.
- No hardware work.
- No custom ("homemade") cryptographic algorithms anywhere in the codebase.
- No JavaScript framework or JS build pipeline (see Section 6.0).

---

## 4. Target users

| Persona | Description | What they need |
|---|---|---|
| **Students & learners** | University students, self-learners, early-career people | Beginner-friendly lessons from zero, no heavy maths up front |
| **Everyday users & creators** | People and creators online | Protecting their identity; verifying digital content |
| **Builders & AI makers** | Software engineers and AI developers at startups and fintechs | Shipping secure apps and AI systems without a maths degree: which algorithms to use, how to use libraries correctly, common mistakes |
| **Businesses & compliance teams** | Banks, fintechs, NDPA-regulated companies | Meeting NDPA 2023 and real-world risk standards; training for staff |
| **Researchers** | Academics and advanced practitioners | Rigorous notes with open citations |

No persona is prioritised at launch (see Section 11). Difficulty levels and learning paths let each group find its own entry point.

---

## 5. Core features (MVP)

### 5.1 Landing page, navigation and footer

**Inspiration:** [pqshield.com](https://pqshield.com/). We borrow its structure, not its content or visual identity:
- a short, confident hero;
- a stack of sections, each under a plain "We …" heading with a short paragraph and one link or button;
- card grids for products and articles;
- publications given their own prominent section;
- a newsletter block near the bottom;
- a footer with link columns.

Every block's text lives in `data/site.toml` (like `profile.toml` in the portfolio), so copy can change without touching templates.

**Top navigation:** wordmark (left), then Products, Learn, Research, About. **The Playground is part of Learn**, not a separate top-level item: on the homepage it is a subsection of the Learn section, and from M2 the `/learn` page links to paths, articles and the playground (at `/learn/playground`). On the right: a theme toggle and an "Early access" button styled as the primary call to action (a search link joins in M2). On mobile, the menu collapses into a `<details>` element, so it needs no JavaScript; on very narrow phones the call to action moves into that menu.

**Homepage sections, in order:**

| # | Section | Contents |
|---|---|---|
| 1 | **Hero** | Headline (draft: "Cryptography you can own.", final wording TBD). Subline: "Finite Labs builds real-world cryptographic software and hardware, teaches the knowledge behind it, and publishes open research, to put digital ownership and control back in your hands." Two **equal-size** buttons: "Get early access" and "Start learning". Right column: the Live Proof widget (below). Background: a subtle dot lattice (Section 7). |
| 2 | **We know the problem** | 2–3 sentences from Section 2: little cryptography expertise in Africa, NDPA pressure on companies, no independent African research voice. |
| 3 | **We build** (Products) | Product areas in development (content provenance, secrets for AI agents, crypto-misuse scanner), one "in development" label for the whole section instead of a badge per card, and a "Get early access" button. |
| 4 | **We teach** (Learn) | **Directly after Products, full size.** Cards for the Foundations path, articles, and courses & training, followed by the **Playground subsection** ("Playground: try it yourself") with cards linking to the demos (hashing, AES-GCM, signatures). Never shrunk or removed when cutting "coming soon" cards elsewhere. In M2 these become the featured path(s) and 3 latest articles with difficulty and reading time. |
| 5 | **We publish our work** | Latest 2–3 research notes, plus an arrow link to `/research`. |
| 6 | **Four pillars** | Subtext: "We build practical security tools and hardware, teach cryptography clearly, and publish our research in the open." One card each: Software & tools (in development), Education (live), Research (live), Hardware (later). |
| 7 | **Who it's for** | Five cards, one per persona in Section 4. Layout: 3 + 2 on desktop, 2 + 2 + 1 on tablets, a swipeable row on phones. |
| 8 | **Early access** | Early-access and newsletter block. Until M5 it links to an email address; then it becomes a plain HTML sign-up form that works without JavaScript (Section 5.7). |

**Live Proof widget (hero, right column).** Real cryptography, nothing simulated, built with the same Rust/WebAssembly stack as the playground (Section 5.4):
- A text input (default: "Send ₦5,000 to Ada"). As the visitor types, the SHA-256 of the text updates live in monospace.
- On page load, an Ed25519 key pair is generated in the browser from `crypto.getRandomValues`. The text is signed on every keystroke, and a status badge shows "Signature verified".
- A **Tamper** switch changes exactly one character of the message the verifier receives (the first digit if there is one, so ₦5,000 becomes ₦9,000), highlights it, and the badge flips to "Verification failed". This is the clearest moment of the demo.
- Note: "Runs entirely in your browser. Nothing is sent to our servers. For learning only."
- Without JavaScript or WebAssembly, the widget still shows the real SHA-256 of the default text (computed by the server) and a short explanation.

**Left out of the MVP:**
- **Partner logo wall:** add it only once real partners or recognitions exist; never use placeholders.
- **"How we work with you" process steps** (e.g. Discuss → Evaluate → Design → Implement): add with Phase 2 corporate training.
- **Events listing:** add once Finite Labs runs or speaks at events.

**Footer:**
- a one-line company description and the mission;
- link columns: Build & learn (Products, Learn, Playground), Research, Company (About, Contact, Privacy policy);
- social links (GitHub, X, LinkedIn);
- "© Finite Labs <year>".

**Contact:** an `/about` page with a short company story and a contact email. A contact form can come later.

### 5.2 Articles (blog / knowledge base)
- Articles are Markdown files in `content/articles/`, each starting with a `+++` TOML frontmatter block, parsed with `pulldown-cmark` and loaded into memory at startup (same approach as the portfolio's `contents::load_dir`).
- Frontmatter fields: `title`, `summary`, `author`, `date`, `difficulty` (`beginner` / `intermediate` / `advanced`), `tags`, `prerequisites` (optional list of article slugs). Reading time is calculated from the word count when the file loads.

  ```toml
  +++
  title = "Hash functions"
  summary = "What a hash function does, and why SHA-256 is everywhere."
  author = "Finite Labs"
  date = "2026-12-01"
  difficulty = "beginner"
  tags = ["hashing", "foundations"]
  prerequisites = ["what-is-cryptography"]
  +++
  ```

- Filtering by tag and difficulty uses query parameters handled on the server (`/articles?tag=hashing&level=beginner`). No JavaScript is needed.
- **Search:** `/search?q=` runs on the server against an in-memory index built at startup (lowercased title, summary, tags and body text). This is enough for the MVP's content size. Move to `tantivy` if content grows past a few hundred documents.
- **Rendering:**
  - Code blocks: syntax highlighting with `syntect` at load time (output is static HTML with no client JS).
  - Math: `pulldown-cmark`'s math extension, converted to MathML (e.g. `pulldown-latex` or `latex2mathml`). Browsers render MathML natively, so KaTeX is not needed.
  - Diagrams: SVG files committed under `static/diagrams/` and referenced from Markdown. They can be drawn with any tool. Nothing renders diagrams in the browser.
  - Embedded demos: a placeholder element in the Markdown, e.g. `<div data-demo="hashing"></div>`. When the loader sees `data-demo`, it sets `has_demo = true` on the item. The template then adds the "For learning only" notice and loads the WebAssembly bundle **only on that page**.

### 5.3 Learning paths
- A learning path is an ordered list of articles and demos, defined in a TOML file in `content/paths/` (e.g. `foundations.toml`):

  ```toml
  slug = "foundations"
  title = "Cryptography Foundations"
  summary = "Start here."
  lessons = ["what-is-cryptography", "classical-ciphers", "hash-functions"]
  ```

- On startup the server checks that every lesson slug exists and **fails to start** if one is missing.
- The path page lists lessons in order with progress indicators.
- Logged-in users mark a lesson complete with a POST form. Progress is saved to their account.

**Initial learning path: Cryptography Foundations**
1. What cryptography is and why it matters
2. Classical ciphers (Caesar, Vigenère) and why they fail
3. Hash functions
4. Symmetric encryption (AES)
5. Public key cryptography and key exchange
6. Digital signatures
7. Randomness and why it matters
8. Common mistakes developers make
9. Intro to post-quantum cryptography

### 5.4 Interactive crypto playground
Demos run entirely in the browser as **Rust compiled to WebAssembly**. No user input is sent to the server.

The Rust code is split into two crates:
- `crates/crypto` holds thin wrappers around audited crates.
- `crates/playground` is the `wasm-bindgen` layer. It reads inputs from the page, calls `crates/crypto`, and writes outputs using `web-sys`.

| Demo | Description | Implementation |
|---|---|---|
| Caesar & Vigenère | Encrypt/decrypt, plus a frequency-analysis attack demo | Plain Rust in `crates/crypto/src/classical.rs` (educational only, clearly labelled) |
| Hashing | Type text, see SHA-256 / SHA3-256 output change (avalanche effect) | `sha2`, `sha3` (RustCrypto) |
| Symmetric encryption | AES-GCM encrypt/decrypt, show nonce and auth tag | `aes-gcm` (RustCrypto) |
| Key exchange | Visual ECDH (X25519) between "Alice" and "Bob" | `x25519-dalek` |
| Digital signatures | Ed25519 sign and verify; tamper with a message and watch verification fail | `ed25519-dalek` |
| Post-quantum KEM | ML-KEM key encapsulation demo | `ml-kem` (RustCrypto). **Check its audit status before relying on it.** |

- Browser randomness comes from `getrandom` with its WebAssembly/browser backend enabled, which calls the browser's `crypto.getRandomValues`.
- Every demo must display a clear notice: **"For learning only. Do not use this page to protect real secrets."**
- **Size budget:** the gzipped WebAssembly bundle stays under 300 KB. Build with `opt-level = "z"`, LTO, and `wasm-opt`. If ML-KEM pushes the bundle over the budget, split it into its own bundle.
- Every demo page shows a short "This demo needs WebAssembly" message for browsers without it.
- One bundle serves every demo: `start()` in `crates/playground` looks for demo containers on the page (e.g. `#live-proof`) and wires up the ones it finds. Pages without a demo don't load it.
- Demo output is written with DOM text nodes, never `innerHTML`, so typed input can't inject HTML.
- Build: `scripts/build-wasm.sh` locally (output in `static/pkg/`, git-ignored); the Dockerfile runs the same steps.

### 5.5 Research section
- Markdown files in `content/research/` with frontmatter fields `title`, `authors`, `date`, `abstract`, `tags`, and optional `pdf` (path under `static/papers/`).
- Each entry has a detail page and an optional downloadable PDF.
- Each paper has a BibTeX citation block, generated by the server from the frontmatter.

### 5.6 Accounts (lightweight)
- Sign up / log in with an emailed magic link or GitHub OAuth.
  - **Magic link:** the server creates 32 random bytes from the OS random number generator, emails them as a link, and stores **only a SHA-256 hash** of the token. The token is single-use and expires after 15 minutes.
  - **GitHub OAuth:** the `oauth2` crate, with PKCE and a `state` parameter.
- Sessions: `tower-sessions` with a Postgres store. The cookie is `Secure`, `HttpOnly`, and `SameSite=Lax`. Every POST form carries a CSRF token.
- The profile stores name, learning progress, and newsletter preference.
- Finite Labs stores no passwords.

### 5.7 Newsletter
- Email capture on the landing page and in article footers.
- Double opt-in: the confirmation link uses the same hashed, single-use token scheme as the magic link.
- Consent text and timestamp are stored to comply with the NDPA.
- Emails go out through Resend's HTTP API (`reqwest`, or the `resend-rs` crate).

---

## 6. Technical requirements

### 6.0 Principle: Rust everywhere
- All application code is Rust: the server, templates (Askama), content loading, and the browser demos (WebAssembly).
- The only JavaScript allowed in the shipped site is:
  1. the glue file that `wasm-bindgen` generates (we never edit it),
  2. `static/playground.js`, a three-line loader that imports and starts that glue, and
  3. a small `static/theme.js` file for the light/dark toggle.
- Every page except the playground must work fully with JavaScript turned off.
- JS-based tools (e.g. Lighthouse, axe) are allowed in CI for auditing only. They never ship to users.

### 6.1 Stack
| Concern | Choice |
|---|---|
| Language | Rust stable (`rust-toolchain.toml`), minimum version set by `rust-version` in `Cargo.toml`, edition 2024 |
| Web server | `axum` + `tokio` |
| Templates | `askama` (type-checked, auto-escaping) |
| Static files, headers, tracing | `tower-http` (`fs`, `trace`, `set-header`, `compression`) |
| Content | Markdown + TOML frontmatter (`pulldown-cmark`, `toml`, `serde`, `chrono`) |
| Syntax highlighting | `syntect` |
| Math | `pulldown-cmark` math events → MathML |
| Search | in-memory index at startup; `tantivy` later if needed |
| Database | PostgreSQL via `sqlx` (compile-time checked queries, migrations in `migrations/`) |
| Hosting for Postgres | any managed Postgres (e.g. Neon, or Supabase used only as hosted Postgres) |
| Auth | `oauth2` (GitHub), magic links (Section 5.6), `tower-sessions` |
| Rate limiting | `tower_governor` |
| Email | Resend HTTP API via `reqwest` |
| Crypto | RustCrypto crates (`sha2`, `sha3`, `aes-gcm`, `ml-kem`), `x25519-dalek`, `ed25519-dalek`, `getrandom` / `rand_core` |
| Browser demos | `wasm-bindgen` + `web-sys`, built with `cargo build --target wasm32-unknown-unknown` and the `wasm-bindgen` CLI, then `wasm-opt` |
| Logging | `tracing`, `tracing-subscriber` |
| Testing | `cargo test`, `#[sqlx::test]`, `wasm-bindgen-test`, `thirtyfour` (WebDriver) for end-to-end |
| Deployment | Docker multi-stage build (`rust:slim-trixie` builder → `debian:trixie-slim` runtime, non-root user) on Render, same as the portfolio |

### 6.2 Folder structure (Cargo workspace)
```
finite_labs/
├── Cargo.toml                # [workspace] members = ["crates/*"]
├── rust-toolchain.toml
├── crates/
│   ├── web/                  # the Axum server (binary)
│   │   └── src/
│   │       ├── main.rs       # startup: load content, connect DB, build Router
│   │       ├── models.rs     # ContentItem, FrontMatter, Path, Research, Difficulty
│   │       ├── content.rs    # load_dir, frontmatter parsing, markdown → HTML, search index
│   │       ├── routes/       # home.rs, about.rs, articles.rs, paths.rs, playground.rs, research.rs, account.rs, newsletter.rs
│   │       ├── auth/         # magic_link.rs, github.rs, session.rs, csrf.rs
│   │       ├── db.rs         # sqlx pool and queries
│   │       ├── email.rs      # Resend client
│   │       └── security.rs   # CSP/HSTS headers, rate-limit layers
│   ├── crypto/               # thin wrappers around audited crates; #![forbid(unsafe_code)]
│   │   └── src/{lib.rs, hash.rs, aead.rs, kx.rs, sign.rs, kem.rs, classical.rs}
│   └── playground/           # wasm-bindgen crate (cdylib), depends on crates/crypto
├── templates/                # base.html, index.html, list.html, article.html, path.html,
│                             # research.html, playground.html, account.html, 404.html
├── static/                   # style.css, theme.js, fonts/, diagrams/, papers/, pkg/ (generated wasm, git-ignored)
├── content/
│   ├── articles/*.md
│   ├── paths/*.toml
│   └── research/*.md
├── data/site.toml            # site name, domain, nav, homepage section copy, footer links
├── migrations/               # sqlx migrations
├── tests/                    # end-to-end tests
├── Dockerfile
├── .github/workflows/ci.yml
├── PRD.md
└── README.md
```

### 6.3 Data model (initial)
- **users:** id, email, name, created_at
- **oauth_identities:** user_id, provider, provider_user_id
- **login_tokens:** token_hash, email, expires_at, used_at
- **sessions:** managed by the `tower-sessions` Postgres store
- **progress:** user_id, lesson_slug, path_slug, completed_at
- **newsletter_subscribers:** email, confirmed, confirm_token_hash, consent_text, consented_at, unsubscribed_at

### 6.4 Security requirements
These are non-negotiable for a cryptography company. A security flaw on our own site would damage credibility.

1. **Never implement custom cryptographic primitives.** Use only the crates listed in Section 6.1, called through `crates/crypto`. Classical ciphers are the only exception, and only as clearly labelled teaching demos. Before launch, record each crypto crate's audit status in `docs/crypto-deps.md`.
2. All demo cryptography runs client-side in WebAssembly. Demo inputs are never logged or sent to the server.
3. Security headers on every response:
   - `Content-Security-Policy: default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; object-src 'none'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'`
   - Inline scripts are not allowed, so the theme script lives in its own file and fonts are self-hosted.
   - Also send HSTS, `X-Content-Type-Options: nosniff`, and `Referrer-Policy: strict-origin-when-cross-origin`. Serve HTTPS only and use secure cookies.
4. Askama auto-escaping stays on. `|safe` is used only for HTML rendered from content in this repository, never for user input.
5. Secrets live only in environment variables and are never committed. CI runs secret scanning (`gitleaks`).
6. `Cargo.lock` is committed. CI runs `cargo deny` (advisories, licences, duplicate crates) and `cargo audit`, and Dependabot is enabled for Cargo.
7. `#![forbid(unsafe_code)]` in `crates/web` and `crates/crypto`.
8. Users can only access their own data:
   - every query that touches user data is scoped by the `user_id` from the session;
   - as a second layer, Postgres row-level security policies use `current_setting('app.user_id')`, set with `SET LOCAL` in each transaction;
   - the app connects as a role that does not own the tables.
9. Rate limiting (`tower_governor`) on login, OAuth callback, and newsletter endpoints.
10. Minimal data collection, and a clear privacy policy aligned with NDPA 2023.

### 6.5 Non-functional requirements
- **Performance:** Lighthouse score 90+ on mobile; pages usable on slow 3G (important for Nigerian mobile users). Server-rendered HTML with no JS on non-demo pages, gzip/brotli compression through `tower-http`, and WebAssembly loaded only on demo pages.
- **Mobile-first:** most users will be on phones.
- **Accessibility:** WCAG 2.1 AA; demos usable by keyboard, with labelled inputs and outputs announced through `aria-live`.
- **SEO:** title and Open Graph meta in `base.html`, a `/sitemap.xml` route generated from loaded content, JSON-LD `Article` structured data, and an Atom feed at `/feed.xml`.
- **Light and dark mode:** follows `prefers-color-scheme` by default, with a toggle in `theme.js`.

---

## 7. Content and brand guidelines

- Tone: clear, confident, friendly. Explain jargon the first time it appears.
- Site copy follows the pqshield.com pattern: short declarative headings ("We teach", "We publish our work") and a plain voice that cuts through hype. Make no claims we can't back up: no "world-class" or "first" without evidence.
- Every article states its difficulty level and prerequisites (enforced by the frontmatter).
- Use African examples and context where natural (e.g. mobile money, fintech, NIN/BVN identity systems).
- Cite sources for all technical claims in research and advanced articles.
- **Brand (for now):** keep it simple:
  - a text wordmark ("Finite Labs") in the site font, no logo file yet;
  - a neutral palette with one accent colour (green), defined as CSS variables in `style.css` so it is easy to change later.
- **Typography:** the system sans-serif for body text; **JetBrains Mono** (self-hosted, OFL licence) for eyebrow tags, status badges, hashes and decorative hex.
- **Hex texture:** cards carry a small, low-contrast hex string that is real: the first 8 characters of the SHA-256 of the card's title. Plain hex, **never a `0x` prefix** (to avoid looking like a cryptocurrency site). One per card, so it reads as texture, not noise. Hidden from screen readers.
- **Hero background:** a very subtle dot-lattice grid (a nod to finite fields and lattice cryptography), fading toward the edges, with opacity tuned separately for light and dark mode. Static: no animation.

---

## 8. Success metrics (first 6 months after launch)

- 25+ published articles; 1 complete learning path.
- 5+ interactive demos live.
- 2+ research notes published.
- 1,000+ newsletter subscribers.
- 300+ registered learners; 20% complete at least one learning path.
- At least 1 inquiry from a company about training (signal for Phase 2).

---

## 9. MVP milestones

| Milestone | Scope |
|---|---|
| **M1: Foundation** | Cargo workspace; Axum server with `base.html` layout (nav + footer), landing page sections from `data/site.toml` (Section 5.1), About page; light/dark mode; security headers; 404 page; Dockerfile and first deploy to Render; CI with `cargo fmt --check`, `clippy -D warnings`, `cargo test`, `cargo deny`, and `gitleaks` |
| **M2: Content engine** | Markdown + frontmatter loader for articles; list and detail pages; tag and difficulty filters; server-side search; syntax highlighting and MathML; sitemap and feed; 5 launch articles |
| **M3: Playground** | *Started early (v0.4): `crates/crypto` (SHA-256, Ed25519), `crates/playground`, and the homepage Live Proof widget exist.* Remaining: `crates/crypto` with test vectors for the other primitives; `crates/playground` WebAssembly build; hashing, AES-GCM, and Ed25519 demos with learning-only notices; WebAssembly bundle within the size budget |
| **M4: Accounts & progress** | Postgres + `sqlx` migrations; magic link and GitHub OAuth; sessions and CSRF; learning paths from TOML; progress tracking; row-level security; rate limiting |
| **M5: Research & newsletter** | Research section, BibTeX citations, PDFs; newsletter with double opt-in through Resend |
| **M6: Launch** | Security review, performance and accessibility audit, privacy policy, `docs/crypto-deps.md`, domain set up, public launch |

---

## 10. Future phases (out of scope for MVP)

- **Phase 2 — Paid education:** paid courses, certificates, corporate training packages for banks and fintechs, payments via a local provider (e.g. Paystack or Flutterwave, called from Rust over their HTTP APIs).
- **Phase 3 — Products:** cryptographic developer tools and AI security products (e.g. model output signing and provenance, secrets management for AI agents, a crypto-misuse scanner for codebases). `crates/crypto` can grow into a publishable crate here.
- **Phase 4 — Research expansion:** open-source libraries, collaborations with universities, conference talks.
- **Phase 5 — Hardware:** secure hardware design or IP licensing, likely through partnerships.

---

## 11. Open questions and decisions

| Question | Status |
|---|---|
| Which audience do we prioritise at launch? | **Decided:** none for now. Serve all four personas through difficulty levels and paths. |
| Personal byline, team byline, or both? | **Open.** The `author` frontmatter field supports either. Default to "Finite Labs" until decided. |
| English only at launch, or other languages later? | **Decided:** English only at launch; no other languages planned for now. |
| Domain name and brand assets? | **Decided for now:** keep it simple (see Section 7). The domain is a placeholder in `data/site.toml`; choose the real one before M6. |

---

## 12. Instructions for AI coding assistants

When working in this repository:

1. Read this PRD before starting any task, and stay within the current milestone unless asked otherwise.
2. The founder is building this partly to learn Rust. Assistants may write code and run `cargo`/`docker` commands, but always explain what was built and why, including any Rust concepts that are new.
3. Rust only (Section 6.0). Don't add JavaScript or npm dependencies beyond the generated `wasm-bindgen` glue and `theme.js`.
4. No `unwrap()` or `expect()` in request handlers. Return proper errors there. They are fine at startup (failing fast on bad content or config) and in tests. Any other use needs a comment explaining why.
5. Never write custom cryptographic algorithms. Use the crates in Section 6.1, wrapped in `crates/crypto`.
6. Every interactive demo must include the "For learning only" notice.
7. Write tests for all library code. Test `crates/crypto` against official test vectors (NIST CAVP for SHA-2/SHA-3/AES-GCM, RFC 7748 for X25519, RFC 8032 for Ed25519, FIPS 203 for ML-KEM). Write a route test for each new page, and end-to-end tests for user flows.
8. Keep modules and templates small. Shared layout goes in `templates/base.html`. Shared logic goes in its own module, not copied between handlers.
9. Never commit secrets or `.env` files.
10. When a requirement is unclear, ask rather than guess, and note assumptions in the pull request description.
