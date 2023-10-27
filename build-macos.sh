#|/bin/zsh
set -e

cargo build --release --bin crab-launcher --target x86_64-apple-darwin
cargo build --release --bin crab-launcher --target aarch64-apple-darwin

mkdir -p CrabLauncher/CrabLauncher.app/Contents/MacOS
lipo target/x86_64-apple-darwin/release/crab-launcher target/aarch64-apple-darwin/release/crab-launcher -create -output CrabLauncher/CrabLauncher.app/Contents/MacOS/crab-launcher
mkdir -p CrabLauncher/CrabLauncher.app/Contents/Resources
cp assets/CrabLauncher.icns CrabLauncher/CrabLauncher.app/Contents/Resources/
tee -a CrabLauncher/CrabLauncher.app/Contents/Info.plist << END
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>English</string>
  <key>CFBundleDisplayName</key>
  <string>CrabLauncher</string>
  <key>CFBundleExecutable</key>
  <string>crab-launcher</string>
  <key>CFBundleIconFile</key>
  <string>CrabLauncher.icns</string>
  <key>CFBundleIdentifier</key>
  <string>eu.mq1.CrabLauncher</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>CrabLauncher</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>$1</string>
  <key>CFBundleVersion</key>
  <string>20231026.095229</string>
  <key>CSResourcesFileMapped</key>
  <true/>
  <key>LSApplicationCategoryType</key>
  <string>public.app-category.games</string>
  <key>LSRequiresCarbon</key>
  <true/>
  <key>NSHighResolutionCapable</key>
  <true/>
  <key>NSHumanReadableCopyright</key>
  <string>Copyright (c) 2023 Manuel Quarneti</string>
</dict>
</plist>
END
ln -sf /Applications CrabLauncher/Applications

rm -f "CrabLauncher-$1-MacOS-Universal2.dmg"
hdiutil create "CrabLauncher-$1-MacOS-Universal2.dmg" -volname CrabLauncher -fs HFS+ -srcfolder CrabLauncher -format UDZO
rm -rf CrabLauncher
