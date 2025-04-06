use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, PartialEq, Eq, ValueEnum)]
enum FormatFile {
    WAV,
    MP3,
    OPUS,
}

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    /// The path to the input file
    #[arg(long)]
    input: PathBuf,

    /// The path to the ffprobe executable
    #[arg(long, default_value = "ffprobe")]
    ffprobe_path: PathBuf,

    /// The path to the ffmpeg executable
    #[arg(long, default_value = "ffmpeg")]
    ffmpeg_path: PathBuf,

    /// File format
    #[arg(long)]
    #[clap(value_enum, default_value_t = FormatFile::WAV)]
    format: FormatFile,

    /// The path to the output file
    #[arg(long)]
    output: PathBuf,

    /// Sample rate in Hz
    #[arg(long, default_value_t = 16000)]
    output_sample_rate: i64,

    /// Number of channels
    #[arg(long, default_value_t = 1)]
    output_channels: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command line arguments
    let args = Args::parse();

    // Create the output folder if it doesn't exist
    std::fs::create_dir_all(args.output.clone())?;

    let filename = args.input;

    // Get the audio stream
    let audio_stream = get_audio_streams(&args.ffprobe_path, &filename)?;
    println!("{:?}", audio_stream);

    // Extract the audio stream
    let output_file = args.output.join(format!(
        "{}.{}",
        filename.file_stem().unwrap().to_str().unwrap(),
        match args.format {
            FormatFile::WAV => "wav",
            FormatFile::MP3 => "mp3",
            FormatFile::OPUS => "opus",
        }
    ));

    println!("Extracting audio stream...");

    // Extract the audio stream using ffmpeg
    extract_audio_stream(
        &args.ffmpeg_path,
        &filename,
        &output_file,
        &args.output_sample_rate,
        &args.output_channels,
    )?;

    println!("Done!");

    Ok(())
}

fn extract_audio_stream(
    ffmpeg_path: &PathBuf,
    input: &PathBuf,
    output: &PathBuf,
    output_sample_rate: &i64,
    output_channels: &i64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Use ffmpeg to extract the audio stream
    let output = std::process::Command::new(ffmpeg_path)
        .arg("-i")
        .arg(input)
        .arg("-ac")
        .arg(output_channels.to_string())
        .arg("-ar")
        .arg(output_sample_rate.to_string())
        .arg("-y") // Overwrite output file if it exists
        .arg(output)
        .output()?;

    // Check if ffmpeg was successful
    if !output.status.success() {
        return Err(format!("ffmpeg failed with status: {}", output.status).into());
    }

    Ok(())
}

fn get_audio_streams(
    ffprobe_path: &PathBuf,
    input: &PathBuf,
) -> Result<Stream, Box<dyn std::error::Error>> {
    // Use ffprobe to get the audio streams
    let output = std::process::Command::new(ffprobe_path)
        .arg("-v")
        .arg("quiet")
        .arg("-print_format")
        .arg("json")
        .arg("-show_streams")
        .arg(input)
        .output()?;

    // Check if ffprobe was successful
    if !output.status.success() {
        return Err(format!("ffprobe failed with status: {}", output.status).into());
    }

    // Parse output to Struct
    let output_str = String::from_utf8_lossy(&output.stdout);

    // Convert the JSON string back to a Point.
    let deserialized: FFprobeOutput = serde_json::from_str(&output_str)?;

    // Check the number of audio streams
    let audio_streams: Vec<Stream> = deserialized
        .streams
        .into_iter()
        .filter(|stream| stream.codec_type.as_ref().unwrap() == "audio")
        .collect();

    if audio_streams.is_empty() {
        return Err("No audio streams found".into());
    }

    if audio_streams.len() > 1 {
        return Err("More than one audio stream found".into());
    }

    // Get the first audio stream
    let audio_stream = audio_streams.into_iter().next().unwrap();

    Ok(audio_stream)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FFprobeOutput {
    pub streams: Vec<Stream>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub index: Option<i64>,
    #[serde(rename = "codec_name")]
    pub codec_name: Option<String>,
    #[serde(rename = "codec_long_name")]
    pub codec_long_name: Option<String>,
    #[serde(rename = "codec_type")]
    pub codec_type: Option<String>,
    #[serde(rename = "codec_time_base")]
    pub codec_time_base: Option<String>,
    #[serde(rename = "codec_tag_string")]
    pub codec_tag_string: Option<String>,
    #[serde(rename = "codec_tag")]
    pub codec_tag: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    #[serde(rename = "has_b_frames")]
    pub has_b_frames: Option<i64>,
    #[serde(rename = "pix_fmt")]
    pub pix_fmt: Option<String>,
    pub level: Option<i64>,
    #[serde(rename = "is_avc")]
    pub is_avc: Option<String>,
    #[serde(rename = "nal_length_size")]
    pub nal_length_size: Option<String>,
    #[serde(rename = "r_frame_rate")]
    pub r_frame_rate: Option<String>,
    #[serde(rename = "avg_frame_rate")]
    pub avg_frame_rate: Option<String>,
    #[serde(rename = "time_base")]
    pub time_base: Option<String>,
    #[serde(rename = "start_time")]
    pub start_time: Option<String>,
    pub duration: Option<String>,
    #[serde(rename = "bit_rate")]
    pub bit_rate: Option<String>,
    #[serde(rename = "nb_frames")]
    pub nb_frames: Option<String>,
    #[serde(rename = "sample_fmt")]
    pub sample_fmt: Option<String>,
    #[serde(rename = "sample_rate")]
    pub sample_rate: Option<String>,
    pub channels: Option<i64>,
    #[serde(rename = "bits_per_sample")]
    pub bits_per_sample: Option<i64>,
}
