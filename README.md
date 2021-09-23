# wavepcm

Fast WAVE PCM file format encoder and decoder in pure Rust.

## API

Below find an example usage of the library:

```rust
use wavepcm::Format;

fn main() -> Result<(), anyhow::Error> {
    let decoding = Format::decode("sample.wav")?;
    decoding.check()?;
    decoding.info()?;

    let encoding = Format::encode(decoding.data, 2, 44_100, 32)?;
    encoding.check()?;
    encoding.info()?;
    encoding.write("sample_new.wav")?;

    Ok(())
}
```

## References

- [WAV](https://en.wikipedia.org/wiki/WAV)

## License

[MIT License](LICENSE)
