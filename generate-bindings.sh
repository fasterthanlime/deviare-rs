#!/bin/sh
 ~/Dev/rust-bindgen/target/debug/bindgen.exe --verbose -o src/bindings.rs --whitelist-type CNktHookLib ~/Dev/capsule/build32/deps/include/NktHookLib.h -- -I /c/Program\ Files\ \(x86\)/Microsoft\ Visual\ Studio\ 14.0/VC/include -x c++
