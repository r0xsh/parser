use bitbuffer::{BitRead, BitWrite, BitWriteSized, BitWriteStream, LittleEndian};

use crate::{ReadResult, Stream};

#[derive(Debug, Clone, PartialEq)]
pub struct VoiceInitMessage {
    codec: String,
    quality: u8,
    sampling_rate: u16,
}

impl BitRead<'_, LittleEndian> for VoiceInitMessage {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let codec = stream.read()?;
        let quality = stream.read()?;

        let sampling_rate = if quality == 255 {
            stream.read()?
        } else if codec == "vaudio_celt" {
            11025
        } else {
            0
        };

        Ok(VoiceInitMessage {
            codec,
            quality,
            sampling_rate,
        })
    }
}

impl BitWrite<LittleEndian> for VoiceInitMessage {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        self.codec.write(stream)?;
        self.quality.write(stream)?;

        if self.quality == 255 {
            self.sampling_rate.write(stream)?;
        }

        Ok(())
    }
}

#[test]
fn test_voice_init_roundtrip() {
    crate::test_roundtrip_encode(VoiceInitMessage {
        codec: "foo".into(),
        quality: 0,
        sampling_rate: 0,
    });
    crate::test_roundtrip_encode(VoiceInitMessage {
        codec: "foo".into(),
        quality: 255,
        sampling_rate: 12,
    });
}

#[derive(BitRead, BitWrite, Debug, Clone)]
#[endianness = "LittleEndian"]
pub struct VoiceDataMessage<'a> {
    client: u8,
    proximity: u8,
    length: u16,
    #[size = "length"]
    data: Stream<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseSoundsMessage<'a> {
    pub reliable: bool,
    pub num: u8,
    pub length: u16,
    pub data: Stream<'a>,
}

impl<'a> BitRead<'a, LittleEndian> for ParseSoundsMessage<'a> {
    fn read(stream: &mut Stream<'a>) -> ReadResult<Self> {
        let reliable = stream.read()?;
        let num = if reliable { 1u8 } else { stream.read()? };
        let length = if reliable {
            stream.read_sized::<u16>(8)?
        } else {
            stream.read()?
        };
        let data = stream.read_sized(length as usize)?;

        Ok(ParseSoundsMessage {
            reliable,
            num,
            length,
            data,
        })
    }
}

impl<'a> BitWrite<LittleEndian> for ParseSoundsMessage<'a> {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        self.reliable.write(stream)?;
        if !self.reliable {
            self.num.write(stream)?;
        }

        if self.reliable {
            self.length.write_sized(stream, 8)?;
        } else {
            self.length.write(stream)?;
        }

        self.data.write(stream)?;

        Ok(())
    }
}

#[test]
fn test_parse_sounds_roundtrip() {
    use bitbuffer::BitReadBuffer;
    let inner = BitReadBuffer::new(&[1, 2, 3, 4, 5, 6], LittleEndian);

    crate::test_roundtrip_encode(ParseSoundsMessage {
        reliable: false,
        num: 0,
        length: inner.bit_len() as u16,
        data: inner.clone().into(),
    });
    crate::test_roundtrip_encode(ParseSoundsMessage {
        reliable: true,
        num: 1,
        length: inner.bit_len() as u16,
        data: inner.into(),
    });
}
