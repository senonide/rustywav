# 👾 rustywav 📻
`rustywav` is a sound synthesizer written in Rust, designed to emulate the sound system of the Nintendo Entertainment System (NES). This project focuses on generating waveforms based on the NES sound channels (Pulse, Triangle, and Noise) and allows users to create retro video game-style sound effects and music.

There are 5 sound channels the first 3 are pulse wave type with different duty cycle: 12.5%, 25% and 50% respectively. The fourth is a triangle wave channel and the fifth is the noise channel.

## Features

- **Pulse Wave**: Emulates the NES pulse waveform with configurable duty cycle and ramp-up effect for smoother sound transitions.
- **Triangle Wave**: Simulates the NES triangle wave, with a smooth ramp-up to create a clean, pitched tone.
- **Noise Channel**: Generates noise-based sounds like those found in the NES sound chip, with options for metallic and periodic noise.
- **Ramp-up Effects**: All waveforms support ramp-up to smoothly increase the volume of the sound over time.
- **CSV-based Music Loading**: Easily load music compositions from a CSV file with a cli argument.

> [!WARNING]
> The Noise channel is work in progress actually, so it may not work as expected.

## Installation

To install `rustywav`, you'll need to have Rust installed on your system. If you haven't already installed Rust, you can get it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

### Steps to run it:

1. **Clone the repository:**
```bash
git clone https://github.com/senonide/rustywav.git
cd rustywav
```
2. **Build the project:**
```bash
cargo build --release
```

3. **Run the project with the example song:**
```bash
cargo run --release examples/example_song.csv
```

## CSV File Syntax for Songs

To use a CSV file to load songs, the file must adhere to the following format:

  1. The file should have 10 columns representing up to 5 simultaneous sound channels:
   - Odd-numbered columns (1, 3, 5, 7, 9) are for notes (e.g., A3, C#4, R for rest/silence).
   - Even-numbered columns (2, 4, 6, 8, 10) are for the duration of the notes in milliseconds (e.g., 500, 1000).

  2. Each row represents sequential notes for all channels. The notes in each row are played simultaneously across the 5 channels.
   - If a channel has no note for a specific time slot, you can leave its note and duration columns empty.
