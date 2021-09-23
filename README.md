# wavepcm

WAVE PCM file format encoder and decoder in pure Rust.

## API

Below find an example usage of the library:

```rust
use wavepcm::Format;

fn main() -> Result<(), anyhow::Error> {
    let decoding = Format::decode("sample.wav")?;
    decoding.check()?;

    let num_channels = u16::from_le_bytes(decoding.num_channels);
    let sampling_rate = u32::from_le_bytes(decoding.sampling_rate);
    let bits_per_sample = u16::from_be_bytes(decoding.bits_per_sample);

    let encoding = Format::encode(decoding.data, num_channels, sampling_rate, bits_per_sample);
    encoding.check()?;

    Ok(())
}
```

## References

- [WAV](https://en.wikipedia.org/wiki/WAV)

## License

[MIT License](LICENSE)
