//! Fast WAVE PCM file format encoder and decoder.
//!
//! WAVE PCM is a library for fast encoding and decoding of WAV PCM format files.
//! As the name suggests, the library only supports the PCM version of WAVE format specification.

#![warn(clippy::all, clippy::pedantic, missing_docs)]

use std::convert::TryInto;
use std::fs::File;
use std::io::{prelude::Read, BufReader};

// Read 2 bytes from a reader.
//
// # Arguments
//
// * `reader` - A reader.
fn read2<T>(reader: &mut T) -> [u8; 2]
where
    T: Read,
{
    let mut buf = [0_u8; 2];
    let _nbytes = reader.read(&mut buf);
    buf
}

// Read 4 bytes from a reader.
//
// # Arguments
//
// * `reader` - A reader.
fn read4<T>(reader: &mut T) -> [u8; 4]
where
    T: Read,
{
    let mut buf = [0_u8; 4];
    let _nbytes = reader.read(&mut buf);
    buf
}

// Read arbitrary number of bytes from a reader.
//
// # Arguments
//
// * `reader` - A reader.
//
// # Errors
//
// If the value cannot fit when performing type conversion.
fn readn<T>(reader: T, nbytes: u32) -> Result<Vec<u8>, anyhow::Error>
where
    T: Read,
{
    let mut buf = Vec::with_capacity(nbytes.try_into()?);
    let mut chunk = reader.take(u64::from(nbytes));
    let _val = chunk.read_to_end(&mut buf);
    Ok(buf)
}

/// WAVE PCM file format.
pub struct Format {
    /// RIFF tag ("RIFF").
    pub riff_tag: [u8; 4],
    /// Total size of a file in bytes.
    pub total_size: [u8; 4],
    /// WAVE tag ("WAVE").
    pub wave_tag: [u8; 4],
    /// Format tag ("fmt ").
    pub fmt_chunk_tag: [u8; 4],
    /// Format chunk size (16 for PCM).
    pub fmt_chunk_size: [u8; 4],
    /// Format type (1 for PCM - uncompressed).
    pub fmt_code: [u8; 2],
    /// Number of channels in the audio data.
    pub num_channels: [u8; 2],
    /// Sampling rate in the audio data (blocks per second).
    pub sampling_rate: [u8; 4],
    /// Byte rate (sampling_rate * num_channels * bits_per_sample / 8).
    pub byte_rate: [u8; 4],
    /// Block alignment value (num_channels * bits_per_sample / 8).
    pub block_alignment: [u8; 2],
    /// Bits per sample in the audio data (8 - 8 bits, 16 - 16 bits, etc).
    pub bits_per_sample: [u8; 2],
    /// Data tag ("data").
    pub data_tag: [u8; 4],
    /// Size of the audio data (num_samples * num_channels * bits_per_sample / 8).
    pub data_size: [u8; 4],
    /// Raw audio data.
    pub data: Vec<u8>,
}

impl Format {
    /// `encode` encodes WAVE PCM file.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw audio data.
    /// * `num_channels` - Number of channels in the audio data.
    /// * `sampling_rate` - Sampling rate in the audio data.
    /// * `bits_per_sample` - Bits per sample in the audio data.
    ///
    /// # Errors
    ///
    /// If the value cannot fit when performing type conversion.
    ///
    /// # Example
    ///
    /// ```
    /// use wavepcm::Format;
    ///
    /// let data = vec![1u8; 16];
    /// let num_channels = 1;
    /// let sampling_rate = 16_000;
    /// let bits_per_sample = 16;
    /// let encoding = Format::encode(data, num_channels, sampling_rate, bits_per_sample);
    /// ```
    pub fn encode(
        data: Vec<u8>,
        num_channels: u16,
        sampling_rate: u32,
        bits_per_sample: u16,
    ) -> Result<Self, anyhow::Error> {
        let size: u32 = data.len().try_into()?;

        let riff_tag = "RIFF".as_bytes().try_into()?;
        let total_size = (size + 36).to_le_bytes();
        let wave_tag = "WAVE".as_bytes().try_into()?;
        let fmt_chunk_tag = "fmt ".as_bytes().try_into()?;
        let fmt_chunk_size = 16_u32.to_le_bytes();
        let fmt_code = 1_u16.to_le_bytes();
        let byte_rate = (sampling_rate * u32::from(num_channels) * u32::from(bits_per_sample) / 8)
            .to_le_bytes();
        let block_alignment = (num_channels * bits_per_sample / 8).to_le_bytes();
        let data_tag = "data".as_bytes().try_into()?;
        let data_size = size.to_le_bytes();

        Ok(Format {
            riff_tag,
            total_size,
            wave_tag,
            fmt_chunk_tag,
            fmt_chunk_size,
            fmt_code,
            num_channels: num_channels.to_le_bytes(),
            sampling_rate: sampling_rate.to_le_bytes(),
            byte_rate,
            block_alignment,
            bits_per_sample: bits_per_sample.to_le_bytes(),
            data_tag,
            data_size,
            data,
        })
    }

    /// `decode` decode WAVE PCM file.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the WAV PCM file.
    ///
    /// # Errors
    ///
    /// This function will return an error if `path` does not already exist.
    /// Other errors may also be returned according to `OpenOptions::open`.
    ///
    /// # Example
    ///
    /// ```
    /// use wavepcm::Format;
    ///
    /// fn main() -> Result<(), anyhow::Error> {
    ///     let decoding = Format::decode("sample.wav")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn decode(path: &str) -> Result<Self, anyhow::Error> {
        let file = File::open(path)?;
        let mut bufr = BufReader::new(file);

        let riff_tag = read4(&mut bufr);
        let total_size = read4(&mut bufr);
        let wave_tag = read4(&mut bufr);
        let fmt_chunk_tag = read4(&mut bufr);
        let fmt_chunk_size = read4(&mut bufr);
        let fmt_code = read2(&mut bufr);
        let num_channels = read2(&mut bufr);
        let sampling_rate = read4(&mut bufr);
        let byte_rate = read4(&mut bufr);
        let block_alignment = read2(&mut bufr);
        let bits_per_sample = read2(&mut bufr);
        let data_tag = read4(&mut bufr);
        let data_size = read4(&mut bufr);
        let data = readn(&mut bufr, u32::from_le_bytes(data_size))?;

        Ok(Format {
            riff_tag,
            total_size,
            wave_tag,
            fmt_chunk_tag,
            fmt_chunk_size,
            fmt_code,
            num_channels,
            sampling_rate,
            byte_rate,
            block_alignment,
            bits_per_sample,
            data_tag,
            data_size,
            data,
        })
    }

    /// `check` checks if the read file complies with WAVE PCM format.
    ///
    /// # Errors
    ///
    /// Returns [`Err`](https://docs.rs/core/*/core/result/enum.Result.html) if the slice is not
    /// UTF-8 with a description as to why the provided bytes are not UTF-8. The vector you moved
    /// in is also included.
    ///
    /// # Example
    ///
    /// ```
    /// use wavepcm::Format;
    ///
    /// fn main() -> Result<(), anyhow::Error> {
    ///     let decoding = Format::decode("sample.wav")?;
    ///     decoding.check()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn check(&self) -> Result<(), anyhow::Error> {
        let riff_tag_val = std::string::String::from_utf8(self.riff_tag.to_vec())?;
        if riff_tag_val != "RIFF" {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires string \"RIFF\" as bytes 1 - 4, got {} instead.",
                riff_tag_val
            ));
        }

        let total_size_len = self.total_size.len();
        if total_size_len != 4 {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires 4 bytes as bytes 5 - 8, got {} instead.",
                total_size_len
            ));
        }

        let wave_tag_val = std::string::String::from_utf8(self.wave_tag.to_vec())?;
        if wave_tag_val != "WAVE" {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires string \"WAVE\" as bytes 9 - 12, got {} instead.",
                wave_tag_val
            ));
        }

        let fmt_chunk_tag_val = std::string::String::from_utf8(self.fmt_chunk_tag.to_vec())?;
        if fmt_chunk_tag_val != "fmt " {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires string \"fmt \" as bytes 13 - 16, got {} instead.",
                fmt_chunk_tag_val
            ));
        }

        let fmt_chunk_size = u32::from_le_bytes(self.fmt_chunk_size);
        if fmt_chunk_size != 16 {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires number 16 as bytes 17 - 20, got {} instead.",
                fmt_chunk_size
            ));
        }

        let fmt_code = u16::from_le_bytes(self.fmt_code);
        if fmt_code != 1 {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires number 1 as bytes 21 - 22, got {} instead.",
                fmt_code
            ));
        }

        let num_channels_len = self.num_channels.len();
        if num_channels_len != 2 {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires 2 bytes as bytes 23 - 24, got {} instead.",
                num_channels_len
            ));
        }

        let sampling_rate_len = self.sampling_rate.len();
        if sampling_rate_len != 4 {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires 4 bytes as bytes 25 - 28, got {} instead.",
                sampling_rate_len
            ));
        }

        let byte_rate_len = self.byte_rate.len();
        if byte_rate_len != 4 {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires 4 bytes as bytes 29 - 32, got {} instead.",
                byte_rate_len
            ));
        }

        let block_alignment_len = self.block_alignment.len();
        if block_alignment_len != 2 {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires 2 bytes as bytes 33 - 34, got {} instead.",
                block_alignment_len
            ));
        }

        let bits_per_sample_len = self.bits_per_sample.len();
        if bits_per_sample_len != 2 {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires 2 bytes as bytes 35 - 36, got {} instead.",
                bits_per_sample_len
            ));
        }

        let data_tag_val = std::string::String::from_utf8(self.data_tag.to_vec())?;
        if data_tag_val != "data" {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires string \"data\" as bytes 37 - 40, got {} instead.",
                data_tag_val
            ));
        }

        let data_size_len = self.data_size.len();
        if data_size_len != 4 {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires 4 bytes as bytes 41 - 44, got {} instead.",
                data_size_len
            ));
        }

        if self.data.is_empty() {
            return Err(anyhow::anyhow!(
                "WAVE PCM format requires at least one bytes as bytes 45 - EOF, got 0 instead."
            ));
        }

        Ok(())
    }

    /// `info` prints the information about the properly-encoded WAVE PCM file.
    ///
    /// # Errors
    ///
    /// Returns [`Err`](https://docs.rs/core/*/core/result/enum.Result.html) if the slice is not
    /// UTF-8 with a description as to why the provided bytes are not UTF-8. The vector you moved
    /// in is also included.
    ///
    /// # Errors
    ///
    /// If the value cannot fit when performing type conversion.
    ///
    /// # Example
    ///
    /// ```
    /// use wavepcm::Format;
    ///
    /// fn main() -> Result<(), anyhow::Error> {
    ///     let decoding = Format::decode("sample.wav")?;
    ///     decoding.check()?;
    ///     decoding.info()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn info(&self) -> Result<(), anyhow::Error> {
        let riff_tag = std::string::String::from_utf8(self.riff_tag.to_vec())?;
        let total_size = u32::from_le_bytes(self.total_size);
        let wave_tag = std::string::String::from_utf8(self.wave_tag.to_vec())?;
        let fmt_chunk_tag = std::string::String::from_utf8(self.fmt_chunk_tag.to_vec())?;
        let fmt_chunk_size = u32::from_le_bytes(self.fmt_chunk_size);
        let fmt_code = u16::from_le_bytes(self.fmt_code);
        let num_channels = u16::from_le_bytes(self.num_channels);
        let sampling_rate = u32::from_le_bytes(self.sampling_rate);
        let byte_rate = u32::from_le_bytes(self.byte_rate);
        let block_alignment = u16::from_le_bytes(self.block_alignment);
        let bits_per_sample = u16::from_le_bytes(self.bits_per_sample);
        let data_tag = std::string::String::from_utf8(self.data_tag.to_vec())?;
        let data_size = u32::from_le_bytes(self.data_size);

        println!("THE WAVE PCM FORMAT HAS BEEN VALIDATED!\n");

        println!("RIFF TAG:           {:?}", riff_tag);
        println!("TOTAL SIZE:         {:?}", total_size);
        println!("WAVE TAG:           {:?}", wave_tag);
        println!("FMT CHUNK TAG:      {:?}", fmt_chunk_tag);
        println!("FMT CHUNK SIZE:     {:?}", fmt_chunk_size);
        println!("FMT CODE:           {:?}", fmt_code);
        println!("CHANNELS:           {:?}", num_channels);
        println!("SAMPLING RATE:      {:?}", sampling_rate);
        println!("BYTERATE:           {:?}", byte_rate);
        println!("BLOCK ALIGNMENT:    {:?}", block_alignment);
        println!("BITS PER SAMPLE:    {:?}", bits_per_sample);
        println!("DATA TAG:           {:?}", data_tag);
        println!("DATA SIZE:          {:?}", data_size);

        Ok(())
    }
}
