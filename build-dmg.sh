#!/bin/sh

ref_name="$1"
arch="$(uname -m)"

test -f "IceLauncher-${ref_name}-macos-${arch}.dmg" && rm "IceLauncher-${ref_name}-macos-${arch}.dmg"

create-dmg \
    --volname "Ice Launcher Installer" \
    --icon-size 64 \
    --icon "Ice Launcher.app" 0 64 \
    --hide-extension "Ice Launcher.app" \
    --app-drop-link 128 64 \
    --no-internet-enable \
    --format ULMO \
    "IceLauncher-${ref_name}-macos-${arch}.dmg" \
    "target/release/bundle/osx/Ice Launcher.app"
