# Create icons for Tauri app
Add-Type -AssemblyName System.Drawing

# Load the PNG
$png = [System.Drawing.Image]::FromFile("$PSScriptRoot\src-tauri\icons\icon.png")

# Create ICO (256x256)
$ico256 = New-Object System.Drawing.Bitmap(256, 256)
$graphics = [System.Drawing.Graphics]::FromImage($ico256)
$graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
$graphics.DrawImage($png, 0, 0, 256, 256)

# Save as PNG sizes for Tauri
$ico256.Save("$PSScriptRoot\src-tauri\icons\icon.ico", [System.Drawing.Imaging.ImageFormat]::Png)
$ico256.Save("$PSScriptRoot\src-tauri\icons\128x128.png", [System.Drawing.Imaging.ImageFormat]::Png)
$ico256.Save("$PSScriptRoot\src-tauri\icons\32x32.png", [System.Drawing.Imaging.ImageFormat]::Png)

# Create smaller sizes
$ico128 = New-Object System.Drawing.Bitmap(128, 128)
$g128 = [System.Drawing.Graphics]::FromImage($ico128)
$g128.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
$g128.DrawImage($png, 0, 0, 128, 128)
$ico128.Save("$PSScriptRoot\src-tauri\icons\128x128@2x.png", [System.Drawing.Imaging.ImageFormat]::Png)

$ico32 = New-Object System.Drawing.Bitmap(32, 32)
$g32 = [System.Drawing.Graphics]::FromImage($ico32)
$g32.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
$g32.DrawImage($png, 0, 0, 32, 32)
$ico32.Save("$PSScriptRoot\src-tauri\icons\32x32@2x.png", [System.Drawing.Imaging.ImageFormat]::Png)

Write-Host "Icons created successfully!"

# Cleanup
$graphics.Dispose()
$ico256.Dispose()
$g128.Dispose()
$ico128.Dispose()
$g32.Dispose()
$ico32.Dispose()
$png.Dispose()
