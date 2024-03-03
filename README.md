# CLI Video Player

> [!WARNING]
> It's can run a video but currently it's locked at 30.5fps good enough to play Bad Apple
> But it will be changed to dynamically use from ffmpeg metadata, and the sound will be out of sync, since I'm trying to figure it out

This is very simple and basic CLI video player written in Rust just for fun. it's currently support grayscale color, if colored video is passed it will be converted into grayscale.

## 🚀 Installation & Running

> [!WARNING]
> Make sure you have ffmpeg on your path, since I'm being too lazy to interface with FFI

```sh
# Clone repository and enter to cloned directory
git clone https://github.com/UnknownRori/cli-video-player
cd cli-video-player

# Run the bad apple video
cargo run --release -- -i ./bad-apple.mp4
```

## License

This project is licensed under the [GLWTSPL](/LICENSE).

![Good Luck](https://github.com/me-shaon/GLWTPL/raw/master/good-luck.gif)

...and godspeed.
