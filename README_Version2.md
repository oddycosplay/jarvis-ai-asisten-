# JARVIS OS Desktop (Windows 10 / 11)

JARVIS OS Desktop adalah wrapper desktop berbasis Electron yang membungkus mockup `jarvis_ui_mockup.html` menjadi aplikasi Windows (.exe / portable). Proyek ini ditujukan untuk demo dan presentasi produk.

Highlights
- Desktop wrapper untuk mockup JARVIS OS (glassmorphism UI)
- Siap dikemas menjadi installer NSIS (.exe) dan artefak portable (.zip)
- Workflow GitHub Actions otomatis build saat Anda mendorong tag rilis `v*`
- Target: Windows 10 & Windows 11 (x64)

Persyaratan
- Node.js 20+
- npm (atau yarn)
- Jika ingin publish otomatis: GitHub repo secret `GH_TOKEN` (PAT dengan scope repo) atau gunakan default `GITHUB_TOKEN` (terbatas).
- Untuk pembuatan ikon Windows: convert SVG → PNG → ICO (instruksi di assets/hantuf-logo-ico-guide.md)

Cara menjalankan lokal (development)
1. Clone repo dan pindah ke branch:
   ```bash
   git checkout -b release/v1.0.1