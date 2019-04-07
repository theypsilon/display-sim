#!/usr/bin/env bash

set -euo pipefail

rm -rf opt-frames || true
cp -r frames/ opt-frames/
pushd opt-frames
for i in * ; do
    echo "$i ..."
    if [[ $i =~ \.jpg$ ]] ; then
        convert $i -sampling-factor 4:2:0 -strip -resize 100x100 -quality 85 -interlace JPEG -colorspace sRGB $i
    elif [[ $i =~ \.png$ ]] ; then
        convert $i -strip -resize 100x100 -alpha Remove $i
    elif [[ $i =~ \.gif$ ]] ; then
        convert $i -coalesce -scale 100x100 +dither -remap $i[0] -layers Optimize $i
    fi
done
