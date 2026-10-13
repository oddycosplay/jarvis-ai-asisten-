#!/usr/bin/env bash
set -euo pipefail

# generate_build_and_release.sh
# Usage:
#   ./script/generate_build_and_release.sh [--publish]
# - without --publish: builds local installer in dist/ (no GitHub publish)
# - with --publish: runs electron-builder --win --publish always (requires GH_TOKEN env var)

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "Working in $ROOT"
command -v node >/dev/null 2>&1 || { echo "Node.js not found. Install Node 20+."; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "npm not found. Install Node 20+."; exit 1; }

# Install dependencies (use ci if lockfile present)
if [ -f package-lock.json ]; then
  npm ci
else
  npm install
fi

# Ensure assets dir
mkdir -p assets

# Try to generate assets/hantuf-logo.ico if not present
if [ ! -f assets/hantuf-logo.ico ]; then
  echo "assets/hantuf-logo.ico not found. Attempting to generate..."

  # Method 1: ImageMagick (convert or magick) + optional png2ico
  if command -v convert >/dev/null 2>&1 || command -v magick >/dev/null 2>&1; then
    IMAGEMAGICK_CMD="$(command -v convert || command -v magick)"
    echo "Using ImageMagick ($IMAGEMAGICK_CMD) to rasterize SVG..."
    "$IMAGEMAGICK_CMD" assets/hantuf-logo.svg -background none -resize 256x256 assets/hantuf-logo.png || true

    if command -v png2ico >/dev/null 2>&1; then
      echo "Using png2ico to create ICO..."
      png2ico assets/hantuf-logo.ico assets/hantuf-logo.png
      rm -f assets/hantuf-logo.png
      echo "Created assets/hantuf-logo.ico via ImageMagick + png2ico"
    else
      # Try npx png-to-ico (node package)
      echo "png2ico not found; attempting npx png-to-ico..."
      npm install --no-save png-to-ico sharp >/dev/null 2>&1 || true
      node -e "const sharp=require('sharp'); const pngToIco=require('png-to-ico'); const fs=require('fs'); (async()=>{ await sharp('assets/hantuf-logo.svg').resize(256,256).png().toFile('assets/hantuf-logo.png'); const buf=await pngToIco(['assets/hantuf-logo.png']); fs.writeFileSync('assets/hantuf-logo.ico',buf); fs.unlinkSync('assets/hantuf-logo.png'); console.log('assets/hantuf-logo.ico created'); })()"
      echo "Created assets/hantuf-logo.ico via sharp + png-to-ico"
    fi

  else
    # Method 2: Node-only fallback (sharp + png-to-ico)
    echo "ImageMagick not found. Attempting Node-based conversion using sharp + png-to-ico..."
    npm install --no-save sharp png-to-ico >/dev/null 2>&1 || true
    node -e "const sharp=require('sharp'); const pngToIco=require('png-to-ico'); const fs=require('fs'); (async()=>{ try{ await sharp('assets/hantuf-logo.svg').resize(256,256).png().toFile('assets/hantuf-logo.png'); const buf=await pngToIco(['assets/hantuf-logo.png']); fs.writeFileSync('assets/hantuf-logo.ico',buf); fs.unlinkSync('assets/hantuf-logo.png'); console.log('assets/hantuf-logo.ico created'); } catch(e){ console.error('Node conversion failed:', e); process.exit(1);} })()"
  fi

  if [ ! -f assets/hantuf-logo.ico ]; then
    echo "Failed to auto-generate assets/hantuf-logo.ico. Please generate manually (see assets/hantuf-logo-ico-guide.md)."
    exit 1
  fi
else
  echo "assets/hantuf-logo.ico already exists — skipping generation."
fi

# Build params
PUBLISH=false
if [ "${1:-}" = "--publish" ] || [ "${2:-}" = "--publish" ]; then
  PUBLISH=true
fi

if [ "$PUBLISH" = true ]; then
  echo "Publishing release to GitHub (requires GH_TOKEN set)..."
  if [ -z "${GH_TOKEN:-}" ]; then
    echo "GH_TOKEN not set. Export GH_TOKEN before running with --publish."
    exit 1
  fi
  # electron-builder publish (always) will attempt to upload to GitHub Release
  npx electron-builder --win --x64 --publish always
else
  echo "Building local installer (no publish)..."
  npx electron-builder --win --x64
fi

echo "Build finished. Artifacts are in the dist/ directory."