# copy-audio-from-video

Copies audio from the video and saves it in a selected format

Algorithm:

- `ffprobe` gets info about audio streams
- `ffmpeg` copies audio
- `symphonia` loads the audio file and `hound` to wav file
