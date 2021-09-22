# wavepcm

WAVE PCM file format encoder and decoder in pure Rust.

## API

Below find an example usage of the library:

```rust
use std::convert::TryInto;
use wavepcm::Format;

fn main() -> Result<(), anyhow::Error> {
    let num_u16 = |x: Vec<u8>| u16::from_le_bytes(x.try_into().expect("Value cannot fit."));
    let num_u32 = |x: Vec<u8>| u32::from_le_bytes(x.try_into().expect("Value cannot fit."));

    let d = Format::decode("sample.wav")?;
    d.check()?;

    let num_channels = num_u16(d.num_channels.to_vec());
    let sampling_rate = num_u32(d.sampling_rate.to_vec());
    let bits_per_sample = num_u16(d.bits_per_sample.to_vec());

    let e = Format::encode(d.data, num_channels, sampling_rate, bits_per_sample);
    e.check()?;

    Ok(())
}
```

## References

- [WAV](https://en.wikipedia.org/wiki/WAV)

## License

[MIT License](LICENSE)
