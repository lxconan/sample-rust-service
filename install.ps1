[CmdletBinding()]
param (
    [Parameter(Mandatory=$false)]
    [Switch]
    $Release
)

./build.ps1 -Release:$Release

if ($Release -eq $true) {
    cargo run --release --bin service-installer -- create --bin ".\target\release\sample-rust-service.exe" --name "sample-rust-service" --disp "Sample Rust Service" --desc "This is a sample service created by RUST programming langauge" --auto
} else {
    cargo run --bin service-installer -- create --bin ".\target\debug\sample-rust-service.exe" --name "sample-rust-service" --disp "Sample Rust Service" --desc "This is a sample service created by RUST programming langauge" --auto
}
