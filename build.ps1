$MyDir = [System.IO.Path]::GetDirectoryName($myInvocation.MyCommand.Definition)
$env:LIBZMQ_LIB_DIR="$MyDir/crates/my-business/zmqlib/bin"
$env:LIBZMQ_INCLUDE_DIR="$MyDir/crates/my-business/zmqlib/include"
cargo build

cp "$MyDir/crates/my-business/zmqlib/dll/*.dll" "$MyDir/target/debug"