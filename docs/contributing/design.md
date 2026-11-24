# fiskus design tokens
 
Reference for the fiskus wordmark and its core visual tokens. Source of truth for README headers, docs sites (mdBook / GitHub Pages), social cards and badges.
 
> fiskus is an independent open-source reference engine for German tax calculation. It is not affiliated with the German government, the Federal Ministry of Finance (Bundesministerium der Finanzen, BMF) or ELSTER.
 
## 1. Identity
 
| Token | Value |
|---|---|
| Name | `fiskus` (always lowercase) |
| Logo type | Wordmark only, no symbol |
| Direction | Vercel-style restraint (tight, monochrome, no ornament) with about 20% German roots (fir green, paper white, Fira Sans) |
| Files | `fiskus-light-mode.svg`, `fiskus-dark-mode.svg` (transparent); `-on-white` and `-on-green` variants include a background |
 
## 2. Color
 
### Core
 
| Token | Hex | Role |
|---|---|---|
| `color.fir` | `#0f3d2e` | Wordmark on light surfaces; background on dark surfaces |
| `color.paper` | `#f3f0e8` | Wordmark on dark surfaces; optional warm light surface |
| `color.white` | `#ffffff` | Default light surface |
 
### Semantic
 
| Token | Light mode | Dark mode |
|---|---|---|
| `color.logo` | `color.fir` `#0f3d2e` | `color.paper` `#f3f0e8` |
| `color.surface` | `color.white` `#ffffff` | `color.fir` `#0f3d2e` |
 
Contrast (WCAG): fir on white **12.2:1**, paper on fir **10.7:1**, fir on paper **10.7:1**. All pass AA for normal text.
 
### Reserved secondary (explored, not part of the logo)
 
Use sparingly in docs and diagrams, one at a time, never together as a tricolor and never with black-red-gold.
 
| Token | Hex | Note |
|---|---|---|
| `color.anthrazit` | `#2b2f33` | Body text on paper surfaces |
| `color.prussian` | `#0b2545` | Alternate dark surface |
| `color.signal` | `#e0391f` | Single-use highlight (e.g. a warning or an i-dot experiment) |
 
## 3. Typography
 
| Token | Value |
|---|---|
| `font.logo.family` | Fira Sans (SIL OFL) |
| `font.logo.weight` | Medium (500) |
| `font.logo.tracking` | `-0.03em` |
| `font.logo.min-ink-gap` | `0.04em` between adjacent letters (applied optically; prevents f and i collisions) |
| `font.logo.case` | lowercase |
 
The shipped SVGs are outlined paths, so no font install is needed. For docs body text, pair with Fira Sans Regular or a system sans stack.
 
## 4. Geometry and spacing
 
Unit `u` = the logo font size (the wordmark was drawn at 96 units).
 
| Token | Value |
|---|---|
| `logo.ascender` | about `0.74u` |
| `logo.clear-space` | `0.6u` on all sides (equals the padding in the shipped files) |
| `logo.min-size` | ascender height of 12px recommended; verified legible at about 10px |
| `logo.aspect` | about 2.1:1 width to height of the text, excluding padding |
 
## 5. Logo usage rules

These rules are about the logo only. Icons and charts follow the [guide](guide.md) rules.
 
- Use `fiskus-light-mode.svg` on light surfaces and `fiskus-dark-mode.svg` on dark surfaces. Never recolor outside the core pairs.
- No gradients, shadows, outlines, effects, rotation or stretching.
- Keep lowercase. Do not add a symbol or tagline into the lockup.
- Do not use the federal eagle (Bundesadler), official seals, flag stripes, rubber stamps, piggy banks or coins anywhere alongside the logo.

## 6. CSS tokens
 
```css
:root {
  --fiskus-fir: #0f3d2e;
  --fiskus-paper: #f3f0e8;
  --fiskus-white: #ffffff;
 
  --fiskus-logo: var(--fiskus-fir);
  --fiskus-surface: var(--fiskus-white);
}
 
@media (prefers-color-scheme: dark) {
  :root {
    --fiskus-logo: var(--fiskus-paper);
    --fiskus-surface: var(--fiskus-fir);
  }
}
```
 
```html
<picture>
  <source srcset="fiskus-dark-mode.svg" media="(prefers-color-scheme: dark)">
  <img src="fiskus-light-mode.svg" alt="fiskus" height="32">
</picture>
```
 
## 7. Open items
 
- Hand-kern letter pairs and ship a one-color badge version.
- Favicon (16 and 32 px), README header and social card (1200x630) built from these tokens.
- Confirm whether dark mode should stay fir green or move to a neutral near-black.
