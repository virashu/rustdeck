cargo build --package rustdeck-media
Write-Output "Built rustdeck-media"

New-Item -ItemType Directory -Path .\plugins -ErrorAction Ignore > $null
Copy-Item .\target\debug\rustdeck_media.dll .\plugins\rustdeck_media.deckplugin
Write-Output "Copied rustdeck-media"
