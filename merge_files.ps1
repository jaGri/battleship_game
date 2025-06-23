<#
.SYNOPSIS
    Recursively combines all .rs and .toml files into a single text file with separators,
    excluding specified directories.
.DESCRIPTION
    This script scans the specified root directory (default: current directory)
    for Rust (.rs) and TOML (.toml) files, excluding any paths containing
    "bkp", ".git", ".vscode", or "target" directories. Each file's content is
    prefixed by a separator line showing its relative path, and all output is
    written to a single output file.
.PARAMETER RootPath
    The directory to scan for files. Defaults to current directory.
.PARAMETER OutputFile
    The file to write combined contents into. Defaults to "combined.txt".
.EXAMPLE
    # Combine files in current folder:
    .\merge_files.ps1
    # Combine files from a project folder:
    .\merge_files.ps1 -RootPath "C:\MyProject" -OutputFile "all_files.txt"
#>
param(
    [string]
    $RootPath = ".",
    [string]
    $OutputFile = "combined.txt"
)

# Ensure output file is reset
if (Test-Path $OutputFile) {
    Remove-Item $OutputFile -Force
}

# Get absolute root path without trailing backslash
$rootFull = (Get-Item $RootPath).FullName.TrimEnd('\')

# Find .rs and .toml files, excluding specified directories
$files = Get-ChildItem -Path $rootFull -Recurse -File | Where-Object {
    ($_.Extension -eq '.rs' -or $_.Extension -eq '.toml') -and
    ($_.FullName -notmatch '\\(?:bkp|\.git|\.vscode|target)\\')
}

# Combine files
foreach ($file in $files) {
    # Compute relative path
    $fullFile = $file.FullName
    $relPath = $fullFile.Substring($rootFull.Length + 1)

    # Separator header
    $header = "===== File: $relPath ====="
    Add-Content -Path $OutputFile -Value $header

    # Append file content
    Get-Content -Path $fullFile | Add-Content -Path $OutputFile

    # Blank line between files
    Add-Content -Path $OutputFile -Value ""
}

Write-Host "Merged $($files.Count) files into '$OutputFile'."
