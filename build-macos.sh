#|/bin/zsh
set -e

cargo build --release --bin crab-launcher --target x86_64-apple-darwin
cargo build --release --bin crab-launcher --target aarch64-apple-darwin

cd gui
cargo install cargo-bundle
cargo bundle --release

cd ../target
lipo x86_64-apple-darwin/release/crab-launcher aarch64-apple-darwin/release/crab-launcher -create -output release/bundle/osx/CrabLauncher.app/Contents/MacOS/crab-launcher
cd release/bundle/osx
rm -rf CrabLauncher
mkdir CrabLauncher
cd CrabLauncher
ln -sf /Applications ./Applications
cd ..
mv CrabLauncher.app CrabLauncher/

rm -f "CrabLauncher-$1-MacOS-Universal2.dmg"
hdiutil create "CrabLauncher-$1-MacOS-Universal2.dmg" -volname CrabLauncher -fs HFS+ -srcfolder CrabLauncher -format UDZO
mv "CrabLauncher-$1-MacOS-Universal2.dmg" ../../../../
