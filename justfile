set dotenv-load

collect data="emoji.csv":
    cd data-collection && cargo run -- ../data/{{data}} ../data/proxied_emoji.csv

clean:
    cd data-cleaning && cargo run -- ../data/proxied_emoji.csv ../data/proxied_emoji_clean.csv

build:
    cd discord-unicode-fix-rs && cargo build --release
    cd discord-unicode-fix-rs && wasm-bindgen --target web --omit-default-module-path --out-dir target/wasm-bindgen/release target/wasm32-unknown-unknown/release/discord_unicode_fix.wasm
    cp discord-unicode-fix-rs/target/wasm-bindgen/release/discord_unicode_fix_bg.wasm discord-unicode-fix/resources/
    cp discord-unicode-fix-rs/target/wasm-bindgen/release/discord_unicode_fix.js discord-unicode-fix/src/wasm_bindgen.js

test:
    cd discord-unicode-fix-rs && cargo test