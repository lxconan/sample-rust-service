$MyDir = [System.IO.Path]::GetDirectoryName($myInvocation.MyCommand.Definition)
$env:LIBZMQ_LIB_DIR="$MyDir/zmqlib/bin"
$env:LIBZMQ_INCLUDE_DIR="$MyDir/zmqlib/include"
$env:PATH=$env:PATH + ";$MyDir/zmqlib/dll"

cargo build --example debug
cargo run --example debug