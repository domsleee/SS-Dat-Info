$objects = @()

foreach ($file in Get-ChildItem "Raw/**/*.txt") {
    $name = $file.BaseName
    $content = Get-Content $file -Raw
    
    $objects += @{
        name = $name
        raw = $content
    }
}

$json = ConvertTo-Json $objects -Depth 10
"export const LEVEL_DATA = $json" | Out-File -FilePath "levelData.ts"
