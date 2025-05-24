#!/bin/bash

cargo build --package sample_plugin
echo "Built sample_plugin"

cp -pf ./target/debug/sample_plugin.dll ./plugins/sample_plugin.deckplugin
echo "Copied sample_plugin"
