[CmdletBinding()]
param (
    [Parameter(Mandatory=$false)]
    [Switch]
    $Release
)

if ($Release -eq $true) {
    cargo build --release
} else {
    cargo build
}