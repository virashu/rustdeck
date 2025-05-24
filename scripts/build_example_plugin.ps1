cargo build --package sample_plugin
Write-Output "Built sample_plugin"

New-Item -ItemType Directory -Path .\plugins -ErrorAction Ignore > $null
Copy-Item .\target\debug\sample_plugin.dll .\plugins\sample_plugin.deckplugin
Write-Output "Copied sample_plugin"
