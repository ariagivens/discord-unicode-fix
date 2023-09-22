set dotenv-load

collect data="emoji.csv":
    cd data-collection && cargo run -- ../data/{{data}} ../data/proxied_emoji.csv

clean:
    cd data-cleaning && cargo run -- ../data/proxied_emoji.csv ../data/proxied_emoji_clean.csv

test:
    cd discord-unicode-fix-js && npm test