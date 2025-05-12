cargo build --package plugin

Copy-Item .\target\debug\plugin.dll .\plugins\plugin.dll
Write-Output "Copied!"

cargo run --package loader
