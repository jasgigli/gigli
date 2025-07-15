#!/usr/bin/env pwsh

param(
    [Parameter(Mandatory=$true)]
    [string]$Version
)

Write-Host "[1/5] Adding all changes to git..."
git add .

git commit -m "chore: release v$Version"

Write-Host "[2/5] Deleting previous git tag if exists..."
git tag -d v$Version 2>$null
Write-Host "[3/5] Creating new git tag v$Version..."
git tag v$Version

Write-Host "[4/5] Pushing code and tags to GitHub..."
git push origin main
git push --delete origin v$Version 2>$null
git push origin main --tags

Write-Host "[5/5] Packing and publishing npm package..."
Push-Location public/gigli-npm
npm pack
npm publish --access public
Pop-Location

Write-Host "Release v$Version complete!"
