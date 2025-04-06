# audio-from-video

[![build](https://github.com/crs-org/audio-from-video/actions/workflows/ci.yml/badge.svg)](https://github.com/crs-org/audio-from-video/actions/workflows/ci.yml)
[![release windows x86_64](https://github.com/crs-org/audio-from-video/actions/workflows/release-win_x86_64.yml/badge.svg)](https://github.com/crs-org/audio-from-video/actions/workflows/release-win_x86_64.yml)
[![release windows aarch64](https://github.com/crs-org/audio-from-video/actions/workflows/release-win_aarch64.yml/badge.svg)](https://github.com/crs-org/audio-from-video/actions/workflows/release-win_aarch64.yml)

This program uses `ffprobe` and `ffmpeg` to do this:

- `ffprobe` gets info about audio streams and used for validation;
- `ffmpeg` gets audio stream and saves it in appropriate format.

## Pre-built ffprobe/ffmpeg binaries

- Linux/Windows: https://github.com/BtbN/FFmpeg-Builds/releases
- MacOS:
  - Intel: https://evermeet.cx/ffmpeg
  - Apple Silicon/Intel: https://www.osxexperts.net

## Usage

```shell
audio-from-video --ffprobe-path /opt/homebrew/bin/ffprobe --ffmpeg-path /opt/homebrew/bin/ffmpeg --format wav --input video.mp4 --output files/ --output-sample-rate 16000 --output-channels 1
```

## Build

You need: cargo, rustc, cross, podman, goreleaser.

0. build images and increase resources for podman:

```shell
podman build --platform=linux/amd64 -f dockerfiles/Dockerfile.aarch64-unknown-linux-gnu -t aarch64-unknown-linux-gnu:my-edge .
podman build --platform=linux/amd64 -f dockerfiles/Dockerfile.x86_64-unknown-linux-gnu -t x86_64-unknown-linux-gnu:my-edge .
```

1. make binaries:

```shell
goreleaser build --clean --snapshot --id audio-from-video --timeout 60m
```
