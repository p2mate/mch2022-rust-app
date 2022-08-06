#!/usr/bin/env bash

set -e

case "$1" in
"" | "release")
    cargo espflash save-image --release rust_esp.img
    ;;
"debug")
    cargo espflash save-image rust_esp.img
    ;;
*)
    echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
    exit 1
    ;;
esac

~/projects/mch2022-tools/webusb_push.py --run rust ~/projects/mch2022-badge-modplayer/rust_esp.img
