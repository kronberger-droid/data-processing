use std::path::PathBuf;

use super::*;

/// A path in the temp folder that no other test uses, removed on drop.
struct TempFile(PathBuf);

impl TempFile {
    fn new(name: &str) -> Self {
        Self(std::env::temp_dir().join(format!("sound-{}-{name}", std::process::id())))
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

/// The largest error of a sample that went through 16 bits and back.
const STEP: f64 = 1.0 / 32768.0;

fn assert_same_signal(got: &[f64], want: &[f64]) {
    assert_eq!(got.len(), want.len(), "length");
    for (i, (g, w)) in got.iter().zip(want).enumerate() {
        assert!((g - w).abs() <= STEP, "sample {i}: {g} is not {w}");
    }
}

#[test]
fn one_second_is_rate_samples() {
    assert_eq!(samples(1.0), RATE as usize);
    assert_eq!(samples(0.5), 11025);
    assert_eq!(samples(0.0), 0);
}

#[test]
fn samples_are_little_endian() {
    // One step up is 0x0001, one step down is 0xffff, low byte first.
    assert_eq!(encode(&[STEP, -STEP]), [0x01, 0x00, 0xff, 0xff]);
}

#[test]
fn full_scale_maps_to_the_ends_of_16_bit() {
    assert_eq!(encode(&[-1.0]), i16::MIN.to_le_bytes());
    // +1 itself is one step past the largest value and clips to it.
    assert_eq!(encode(&[1.0]), i16::MAX.to_le_bytes());
}

#[test]
fn samples_beyond_full_scale_clip_instead_of_wrapping() {
    assert_eq!(encode(&[2.5, -2.5]), encode(&[1.0, -1.0]));
}

#[test]
fn decode_reverses_encode() {
    let signal = [0.0, 0.25, -0.25, 0.999, -1.0, 0.123456];
    assert_same_signal(&decode(&encode(&signal)), &signal);
}

#[test]
fn a_trailing_odd_byte_is_dropped() {
    assert_eq!(decode(&[0x00, 0x40, 0x7f]), [0.5]);
}

#[test]
fn a_wav_file_reads_back_as_written() {
    let file = TempFile::new("roundtrip.wav");
    let signal: Vec<f64> = (0..1000).map(|i| (i as f64 * 0.05).sin() * 0.8).collect();
    write_wav(&file.0, &signal).unwrap();
    assert_same_signal(&read_wav(&file.0).unwrap(), &signal);
}

#[test]
fn the_wav_header_describes_mono_16_bit_at_rate() {
    let file = TempFile::new("header.wav");
    write_wav(&file.0, &[0.0; 100]).unwrap();
    let wav = fs::read(&file.0).unwrap();

    assert_eq!(wav.len(), 44 + 200, "header and two bytes per sample");
    assert_eq!(&wav[..4], b"RIFF");
    assert_eq!(wav[4..8], 236u32.to_le_bytes(), "file size minus 8");
    assert_eq!(&wav[8..16], b"WAVEfmt ");
    assert_eq!(wav[22..24], 1u16.to_le_bytes(), "channels");
    assert_eq!(wav[24..28], RATE.to_le_bytes(), "rate");
    assert_eq!(wav[34..36], 16u16.to_le_bytes(), "bits");
    assert_eq!(&wav[36..40], b"data");
    assert_eq!(wav[40..44], 200u32.to_le_bytes(), "data size");
}

/// A WAV file with the given format fields and `extra` between the format
/// and the samples.
fn wav_with(channels: u16, rate: u32, extra: &[u8], data: &[u8]) -> Vec<u8> {
    let mut wav = b"RIFF\0\0\0\0WAVEfmt ".to_vec();
    wav.extend(16u32.to_le_bytes());
    wav.extend(1u16.to_le_bytes());
    wav.extend(channels.to_le_bytes());
    wav.extend(rate.to_le_bytes());
    wav.extend((rate * 2 * channels as u32).to_le_bytes());
    wav.extend((2 * channels).to_le_bytes());
    wav.extend(16u16.to_le_bytes());
    wav.extend(extra);
    wav.extend(b"data");
    wav.extend((data.len() as u32).to_le_bytes());
    wav.extend(data);
    wav
}

#[test]
fn chunks_other_than_format_and_data_are_skipped() {
    // An odd sized chunk is followed by a padding byte.
    let file = TempFile::new("list.wav");
    let list = [b"LIST".as_slice(), &3u32.to_le_bytes(), b"abc\0"].concat();
    fs::write(&file.0, wav_with(1, RATE, &list, &[0x00, 0x40])).unwrap();
    assert_eq!(read_wav(&file.0).unwrap(), [0.5]);
}

#[test]
fn a_data_size_past_the_end_reads_what_is_there() {
    // What `arecord` leaves behind when it wrote to a pipe.
    let file = TempFile::new("streamed.wav");
    let mut wav = wav_with(1, RATE, &[], &[0x00, 0x40, 0x00, 0xc0]);
    wav[40..44].copy_from_slice(&u32::MAX.to_le_bytes());
    fs::write(&file.0, wav).unwrap();
    assert_eq!(read_wav(&file.0).unwrap(), [0.5, -0.5]);
}

#[test]
fn another_rate_is_rejected() {
    let file = TempFile::new("rate.wav");
    fs::write(&file.0, wav_with(1, 44100, &[], &[0, 0])).unwrap();
    let error = read_wav(&file.0).unwrap_err().to_string();
    assert!(error.contains("44100 Hz"), "{error}");
}

#[test]
fn stereo_is_rejected() {
    let file = TempFile::new("stereo.wav");
    fs::write(&file.0, wav_with(2, RATE, &[], &[0, 0, 0, 0])).unwrap();
    let error = read_wav(&file.0).unwrap_err().to_string();
    assert!(error.contains("2 channels"), "{error}");
}

#[test]
fn a_file_that_is_no_wav_is_rejected() {
    let file = TempFile::new("text.wav");
    fs::write(&file.0, "this is no sound at all").unwrap();
    let error = read_wav(&file.0).unwrap_err().to_string();
    assert!(error.contains("not a WAV file"), "{error}");
}
