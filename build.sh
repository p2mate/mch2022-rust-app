#!/bin/sh
cargo espflash --release save-image rust_esp.img 
~/projects/mch2022-tools/webusb_push.py --run rust  ~/projects/mch2022-badge-modplayer/rust_esp.img
