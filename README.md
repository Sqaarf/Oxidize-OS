# Oxidize-OS

A crash test OS developed in Rust.

## How to run
<code>cargo build</code><br>
<code>cargo bootimage</code><br>
<code>qemu-system-x86_64 -drive format=raw,file=target/x86_64-oxidize_os/debug/bootimage-oxidize_os.bin</code><br>
or<br>
<code>cargo run</code>
