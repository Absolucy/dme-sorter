# build-and-wrap.ps1

# Save original state
$origPath = $env:PATH
$origLocation = Get-Location

# Temporarily add w2c2 to PATH
$env:PATH = "$($env:W2C2_DIR)\w2c2\build;$env:PATH"

try {
	# 1) cargo build
	Write-Host "🔨 Running cargo build..."
	cargo build --release --target wasm32-wasip1 --features diff
	if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }

	# 2) copy the .wasm
	Write-Host "📦 Copying dme-sorter.wasm into wasi/ folder..."
	Copy-Item -Path "target/wasm32-wasip1/release/dme-sorter.wasm" `
			  -Destination "wasi/dme-sorter.wasm" -Force

	# 3) cd into wasi
	Write-Host "📂 Entering wasi/ directory..."
	Push-Location "wasi"

	try {
		# 4) run w2c2 to produce C
		Write-Host "✍️  Generating C with w2c2..."
		w2c2 dme-sorter.wasm dme-sorter.c
		if ($LASTEXITCODE -ne 0) { throw "w2c2 conversion failed" }

		# 5) clang compile
		Write-Host "🛠️  Compiling with clang..."
		clang `
			-Wno-everything `
			-I "$($env:W2C2_DIR)\w2c2" `
			-I "$($env:W2C2_DIR)" `
			-L "$($env:W2C2_DIR)\wasi\build" `
			main.c dme-sorter.c `
			-lw2c2wasi `
			-ladvapi32 `
			-o dme-sorter.exe

		if ($LASTEXITCODE -ne 0) { throw "clang compile failed" }
		Write-Host "✅ Build complete: wasi\dme-sorter.exe"
	}
	finally {
		# always pop back out of wasi
		Pop-Location
	}
}
finally {
	# restore original PATH and directory
	$env:PATH = $origPath
	Set-Location $origLocation
	Write-Host "🔄 Environment restored."
}
