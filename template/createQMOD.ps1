# Builds a .qmod file
cargo ndk build --release

if ($?) {
    Compress-Archive -Path "./mod.json", "./target/aarch64-linux-android/release/lib#{ID}.so" -DestinationPath "./#{ID}_${version}.zip" -Update
    Remove-Item "./#{ID}_${version}.qmod"
    Rename-Item "./#{ID}_${version}.zip" "./#{ID}_${version}.qmod"
}
