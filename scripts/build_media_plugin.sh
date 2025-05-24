#!/bin/bash

cargo build --package rustdeck-media
echo "Built rustdeck-media"

cp -pf ./target/debug/rustdeck_media.dll ./plugins/rustdeck_media.deckplugin
echo "Copied rustdeck-media"
