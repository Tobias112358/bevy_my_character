#!/bin/sh
cargo build --target x86_64-pc-windows-gnu --release &&
exec ./target/x86_64-pc-windows-gnu/release/bevy_my_character.exe "$@"