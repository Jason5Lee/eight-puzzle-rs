cp -R mac_os_app_template "Eight Puzzle.app"
cp Resources/* "Eight Puzzle.app/Contents/Resources"
# Now built by Azure pipeline
# cargo build --release
cp target/release/eight-puzzle "Eight Puzzle.app/Contents/MacOS"