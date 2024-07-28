# decide what audio lib to use per platform

echo "Paste in new text at any time, press ctrl+c to quit"
cargo run 2> /dev/null | paplay --raw --rate=22050 --channels=1 --format=s16le
