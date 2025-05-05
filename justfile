fmt:
    cargo fmt

release: fmt
    cargo build --release

clippy:
    cargo clippy --all-targets

archive:
    ouch compress dist/audio-from-video_aarch64-apple-darwin dist/audio-from-video_aarch64-apple-darwin.zip
    ouch compress dist/audio-from-video_aarch64-unknown-linux-gnu dist/audio-from-video_aarch64-unknown-linux-gnu.zip
    ouch compress dist/audio-from-video_x86_64-apple-darwin dist/audio-from-video_x86_64-apple-darwin.zip
    ouch compress dist/audio-from-video_x86_64-unknown-linux-gnu dist/audio-from-video_x86_64-unknown-linux-gnu.zip
