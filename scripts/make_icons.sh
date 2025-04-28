#!/usr/bin/env bash
set -e

INPUT="../assets/icon.svg"
OUTPUT_DIR="../assets"
BASENAME="icon"
TMP_DIR=$(mktemp -d)
ICONSET_DIR="$TMP_DIR/iconset"

# Check required tools
command -v rsvg-convert >/dev/null || { echo "Missing: rsvg-convert"; exit 1; }
command -v convert >/dev/null || { echo "Missing: ImageMagick convert"; exit 1; }
command -v png2icns >/dev/null || { echo "Missing: png2icns (install icnsutils)"; exit 1; }

# Only valid ICNS sizes
SIZES=(16 32 48 128 256 512 1024)

mkdir -p "$ICONSET_DIR"

# Generate PNGs
for SIZE in "${SIZES[@]}"; do
  rsvg-convert -w "$SIZE" -h "$SIZE" "$INPUT" -o "$ICONSET_DIR/icon_${SIZE}x${SIZE}.png"
done

# Create .ico (supports more sizes including 64x64)
convert \
  "$ICONSET_DIR/icon_16x16.png" \
  "$ICONSET_DIR/icon_32x32.png" \
  "$ICONSET_DIR/icon_48x48.png" \
  "$ICONSET_DIR/icon_128x128.png" \
  "$ICONSET_DIR/icon_256x256.png" \
  "$OUTPUT_DIR/${BASENAME}.ico"

# Create .icns (macOS icons)
png2icns "$OUTPUT_DIR/${BASENAME}.icns" "$ICONSET_DIR"/icon_*.png

# Create standalone 1024x1024 PNG
cp "$ICONSET_DIR/icon_1024x1024.png" "$OUTPUT_DIR/${BASENAME}.png"

echo "✅ Done: ${OUTPUT_DIR}/${BASENAME}.{ico,icns,png}"

# Clean up

rm -rf "$TMP_DIR"
echo "🧹 Cleaned up temporary files."
