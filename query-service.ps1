[CmdletBinding()]
param (
    [Parameter(Mandatory=$false)]
    [Switch]
    $Release
)

./build.ps1 -Release:$Release

if ($Release -eq $true) {
    cargo run --release --bin service-installer -- query --name "sample-rust-service"
} else {
    cargo run --bin service-installer -- query --name "sample-rust-service"
}

