#!/bin/sh

DIST_DIR=.
FONT_FAMILIES=(Shippori%20Mincho Roboto)

cd $(dirname $0) \
  && mkdir -p "${DIST_DIR}" \
  && cd "${DIST_DIR}"

for family in "${FONT_FAMILIES[@]}"
do
  curl -o "fonts.zip" --create-dirs "https://fonts.google.com/download?family=${family}" \
    && unzip -o "fonts.zip" \
    && rm "fonts.zip"
done
