#!/usr/bin/env pwsh

param(
    [Parameter(Mandatory=$true)]
    [string]$Version
)

Write-Host "[1/7] Bumping version to $Version in all relevant files..."

# Update version in root Cargo.toml, Cargo.llvm.toml, my-app/gigli.toml, gigli.config.json, public/gigli-npm/package.json
(Get-Content "Cargo.toml") -replace 'version = ".*"', 'version = "' + $Version + '"' | Set-Content "Cargo.toml"
(Get-Content "Cargo.llvm.toml") -replace 'version = ".*"', 'version = "' + $Version + '"' | Set-Content "Cargo.llvm.toml"
if (Test-Path "my-app/gigli.toml") {
    (Get-Content "my-app/gigli.toml") -replace 'version = ".*"', 'version = "' + $Version + '"' | Set-Content "my-app/gigli.toml"
}
(Get-Content "gigli.config.json") -replace '"std": "[^"]+"', '"std": "^' + $Version + '"' | Set-Content "gigli.config.json"
(Get-Content "gigli.config.json") -replace '"@gigli/cli": "[^"]+"', '"@gigli/cli": "^' + $Version + '"' | Set-Content "gigli.config.json"
(Get-Content "public/gigli-npm/package.json") -replace '"version": ".*"', '"version": "' + $Version + '"' | Set-Content "public/gigli-npm/package.json"

Write-Host "[2/7] Committing all changes..."
git add .
git commit -m "chore: bump version to v$Version everywhere"

Write-Host "[3/7] Deleting previous git tag if exists..."
git tag -d v$Version 2>$null
Write-Host "[4/7] Creating new git tag v$Version..."
git tag v$Version

Write-Host "[5/7] Pushing code and tags to GitHub..."
git push origin main
git push --delete origin v$Version 2>$null
git push origin main --tags

Write-Host "[6/7] Packing and publishing npm package..."
Push-Location public/gigli-npm
npm pack
npm publish --access public
Pop-Location

Write-Host "[7/7] Release v$Version complete!"
