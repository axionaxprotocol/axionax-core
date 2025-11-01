# Axionax Dependency Installer for Windows
# Requires: PowerShell 5.1+ (Run as Administrator)

#Requires -RunAsAdministrator

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Blue
Write-Host "   Axionax Dependency Installer" -ForegroundColor Blue
Write-Host "========================================" -ForegroundColor Blue
Write-Host ""

# Function to check if a command exists
function Test-Command {
    param($Command)
    try {
        if (Get-Command $Command -ErrorAction Stop) {
            return $true
        }
    } catch {
        return $false
    }
}

# Function to download and install
function Install-FromUrl {
    param(
        [string]$Url,
        [string]$Output,
        [string]$Name
    )
    
    Write-Host "Downloading $Name..." -ForegroundColor Cyan
    Invoke-WebRequest -Uri $Url -OutFile $Output
    Write-Host "Installing $Name..." -ForegroundColor Cyan
    Start-Process -FilePath $Output -ArgumentList "/SILENT" -Wait
    Remove-Item $Output
    Write-Host "✓ $Name installed" -ForegroundColor Green
}

# Check Windows version
$winVersion = [System.Environment]::OSVersion.Version
Write-Host "Windows Version: $($winVersion.Major).$($winVersion.Minor)" -ForegroundColor Green
Write-Host ""

if ($winVersion.Major -lt 10) {
    Write-Host "Warning: Windows 10 or later is recommended" -ForegroundColor Yellow
}

# Install Chocolatey (Package Manager for Windows)
if (-not (Test-Command choco)) {
    Write-Host "Installing Chocolatey..." -ForegroundColor Cyan
    Set-ExecutionPolicy Bypass -Scope Process -Force
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
    Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
    Write-Host "✓ Chocolatey installed" -ForegroundColor Green
    
    # Refresh environment
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
} else {
    Write-Host "✓ Chocolatey already installed" -ForegroundColor Green
}

Write-Host ""
Write-Host "Installing dependencies via Chocolatey..." -ForegroundColor Blue
Write-Host ""

# Core development tools
$packages = @(
    "git",
    "vscode",
    "nodejs-lts",
    "python",
    "rustup.install",
    "docker-desktop",
    "postgresql14",
    "nginx"
)

foreach ($package in $packages) {
    Write-Host "Installing $package..." -ForegroundColor Cyan
    choco install $package -y --no-progress
}

Write-Host ""
Write-Host "Installing additional tools..." -ForegroundColor Blue
Write-Host ""

# Visual Studio Build Tools (required for Rust)
if (-not (Test-Path "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools")) {
    Write-Host "Installing Visual Studio Build Tools..." -ForegroundColor Cyan
    choco install visualstudio2022buildtools -y --no-progress
    choco install visualstudio2022-workload-vctools -y --no-progress
} else {
    Write-Host "✓ Visual Studio Build Tools already installed" -ForegroundColor Green
}

# OpenSSL
if (-not (Test-Command openssl)) {
    Write-Host "Installing OpenSSL..." -ForegroundColor Cyan
    choco install openssl -y --no-progress
} else {
    Write-Host "✓ OpenSSL already installed" -ForegroundColor Green
}

# Install Rust components
Write-Host ""
Write-Host "Configuring Rust..." -ForegroundColor Blue

# Refresh PATH
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

if (Test-Command rustc) {
    rustup default stable
    rustup update
    rustup component add clippy rustfmt
    Write-Host "✓ Rust configured" -ForegroundColor Green
} else {
    Write-Host "⚠ Rust not found in PATH. Please restart terminal." -ForegroundColor Yellow
}

# Install Node.js global packages
Write-Host ""
Write-Host "Installing Node.js global packages..." -ForegroundColor Blue

if (Test-Command npm) {
    npm install -g yarn
    npm install -g typescript
    npm install -g ts-node
    npm install -g wscat
    Write-Host "✓ Node.js packages installed" -ForegroundColor Green
} else {
    Write-Host "⚠ npm not found. Please restart terminal." -ForegroundColor Yellow
}

# Install Python packages
Write-Host ""
Write-Host "Installing Python packages..." -ForegroundColor Blue

if (Test-Command python) {
    python -m pip install --upgrade pip
    pip install virtualenv
    pip install pytest
    pip install requests
    Write-Host "✓ Python packages installed" -ForegroundColor Green
} else {
    Write-Host "⚠ Python not found. Please restart terminal." -ForegroundColor Yellow
}

# Configure PostgreSQL
Write-Host ""
Write-Host "Configuring PostgreSQL..." -ForegroundColor Blue

$pgPath = "C:\Program Files\PostgreSQL\14\bin"
if (Test-Path $pgPath) {
    # Add to PATH if not already there
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    if ($currentPath -notlike "*$pgPath*") {
        [Environment]::SetEnvironmentVariable(
            "Path",
            $currentPath + ";$pgPath",
            "Machine"
        )
        Write-Host "✓ PostgreSQL added to PATH" -ForegroundColor Green
    }
} else {
    Write-Host "⚠ PostgreSQL installation path not found" -ForegroundColor Yellow
}

# Enable WSL2 (for Docker Desktop)
Write-Host ""
Write-Host "Checking WSL2..." -ForegroundColor Blue

$wslStatus = wsl --status 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ WSL2 is enabled" -ForegroundColor Green
} else {
    Write-Host "Installing WSL2..." -ForegroundColor Cyan
    wsl --install
    Write-Host "✓ WSL2 installed (restart required)" -ForegroundColor Green
}

# Configure Windows Defender exclusions for better performance
Write-Host ""
Write-Host "Configuring Windows Defender exclusions..." -ForegroundColor Blue

$exclusions = @(
    "$env:USERPROFILE\.cargo",
    "$env:USERPROFILE\.rustup",
    "$env:USERPROFILE\AppData\Local\Temp\cargo",
    "C:\ProgramData\chocolatey"
)

foreach ($exclusion in $exclusions) {
    if (Test-Path $exclusion) {
        try {
            Add-MpPreference -ExclusionPath $exclusion -ErrorAction SilentlyContinue
            Write-Host "✓ Added exclusion: $exclusion" -ForegroundColor Green
        } catch {
            Write-Host "⚠ Could not add exclusion: $exclusion" -ForegroundColor Yellow
        }
    }
}

# Install Windows Terminal (optional but recommended)
if (-not (Get-AppxPackage -Name "Microsoft.WindowsTerminal")) {
    Write-Host ""
    Write-Host "Installing Windows Terminal..." -ForegroundColor Cyan
    choco install microsoft-windows-terminal -y --no-progress
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Blue
Write-Host "   Verifying Installations" -ForegroundColor Blue
Write-Host "========================================" -ForegroundColor Blue
Write-Host ""

# Refresh environment one more time
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

function Check-Installation {
    param(
        [string]$Command,
        [string]$Name
    )
    
    if (Test-Command $Command) {
        try {
            $version = & $Command --version 2>&1 | Select-Object -First 1
            Write-Host "✓ $Name : $version" -ForegroundColor Green
        } catch {
            Write-Host "✓ $Name : Installed" -ForegroundColor Green
        }
    } else {
        Write-Host "✗ $Name : Not found (restart terminal may be required)" -ForegroundColor Red
    }
}

Check-Installation "git" "Git"
Check-Installation "rustc" "Rust"
Check-Installation "cargo" "Cargo"
Check-Installation "node" "Node.js"
Check-Installation "npm" "npm"
Check-Installation "python" "Python"
Check-Installation "pip" "pip"
Check-Installation "docker" "Docker"
Check-Installation "psql" "PostgreSQL"
Check-Installation "nginx" "Nginx"

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "   Installation Complete! 🎉" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Next Steps:" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. Restart your computer to complete installation" -ForegroundColor White
Write-Host "2. Open a new PowerShell/Terminal window" -ForegroundColor White
Write-Host "3. Verify installations:" -ForegroundColor White
Write-Host "   " -NoNewline
Write-Host "rustc --version" -ForegroundColor Cyan
Write-Host "   " -NoNewline
Write-Host "node --version" -ForegroundColor Cyan
Write-Host "   " -NoNewline
Write-Host "python --version" -ForegroundColor Cyan
Write-Host ""
Write-Host "4. Clone repository:" -ForegroundColor White
Write-Host "   " -NoNewline
Write-Host "git clone https://github.com/axionaxprotocol/axionax-core.git" -ForegroundColor Cyan
Write-Host ""
Write-Host "5. Build project:" -ForegroundColor White
Write-Host "   " -NoNewline
Write-Host "cd axionax-core" -ForegroundColor Cyan
Write-Host "   " -NoNewline
Write-Host "cargo build --release" -ForegroundColor Cyan
Write-Host ""
Write-Host "Development Tools Installed:" -ForegroundColor Yellow
Write-Host "  • VS Code: code ." -ForegroundColor Cyan
Write-Host "  • Windows Terminal: wt" -ForegroundColor Cyan
Write-Host "  • Docker Desktop: docker ps" -ForegroundColor Cyan
Write-Host ""
Write-Host "For WSL2/Linux development:" -ForegroundColor Yellow
Write-Host "  • Start WSL: " -NoNewline -ForegroundColor White
Write-Host "wsl" -ForegroundColor Cyan
Write-Host "  • Install Ubuntu: " -NoNewline -ForegroundColor White
Write-Host "wsl --install -d Ubuntu" -ForegroundColor Cyan
Write-Host ""

# Prompt for restart
$restart = Read-Host "Restart computer now? (y/n)"
if ($restart -eq 'y' -or $restart -eq 'Y') {
    Write-Host "Restarting in 10 seconds..." -ForegroundColor Yellow
    Start-Sleep -Seconds 10
    Restart-Computer -Force
}
