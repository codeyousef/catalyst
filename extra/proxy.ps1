[CmdletBinding()]
param(
    [string]$version,
    [string]$directory
)

$proxy = (Join-Path $directory 'catalyst.exe')

$CatalystProcesses = (Get-Process -Name 'catalyst' -EA SilentlyContinue).Count
if ($CatalystProcesses -ne 0) {
    Write-Host 'Proxy currently in use. Aborting installation'
    exit
}

if (Test-Path $proxy) {
    Write-Host 'Proxy already installed'
    exit
}

switch ($env:PROCESSOR_ARCHITECTURE) {
    # Only x86_64 is supported currently
    'AMD64' {
        $arch = 'x86_64'
    }
}

$url = "https://github.com/catalyst/catalyst/releases/download/${version}/catalyst-proxy-windows-${arch}.gz"
$gzip = Join-Path "${env:TMP}" "catalyst-proxy-windows-${arch}.gz"

$webclient = [System.Net.WebClient]::new()
$webclient.DownloadFile($url, $gzip)
$webclient.Dispose()

[System.IO.Directory]::CreateDirectory($directory)

$archive = [System.IO.File]::Open($gzip, [System.IO.FileMode]::Open)
$proxy_file = [System.IO.File]::Create($proxy)
$compressor = [System.IO.Compression.GZipStream]::new($archive, [System.IO.Compression.CompressionMode]::Decompress)
$compressor.CopyTo($proxy_file)
Start-Sleep -Seconds 3
$compressor.close()
$proxy_file.close()
$archive.close()

[System.IO.File]::Delete($gzip)

Write-Host 'catalyst-proxy installed'