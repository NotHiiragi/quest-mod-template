if ($?) {
    Compress-Archive -Path "./build/arm64-v8a/lib#{ID}.so", "./mod.json" -DestinationPath "./#{NAME}_v0.1.0.zip" -Update
}
