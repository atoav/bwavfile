use std::io::{Write,Seek,SeekFrom};

use super::wavewriter::WaveWriter;
use super::errors::Error;
use super::fmt::WaveFmt;

use byteorder::LittleEndian;
use byteorder::WriteBytesExt;

pub struct AudioFrameWriter<W: Write + Seek> {
    inner : W,
    form_size : u64,
    data_size : u64,
    data_start : u64,
    format: WaveFmt
}

impl<W:Write + Seek> AudioFrameWriter<W>  {

    pub fn new(inner: W, format: WaveFmt) -> Self {
        todo!();
    }

    pub fn write_audio_frame(&mut self, buffer: &[i32]) -> Result<u64,Error> {
        assert!(buffer.len() as u16 == self.format.channel_count, 
        "read_integer_frame was called with a mis-sized buffer, expected {}, was {}", 
        self.format.channel_count, buffer.len());

        let framed_bits_per_sample = self.format.block_alignment * 8 / self.format.channel_count;
        for n in 0..(self.format.channel_count as usize) {
             match (self.format.bits_per_sample, framed_bits_per_sample) {
                (0..=8,8) => self.inner.write_u8(( buffer[n] + 0x80) as u8 )?, // EBU 3285 §A2.2
                (9..=16,16) => self.inner.write_i16::<LittleEndian>(buffer[n] as i16)?,
                (10..=24,24) => self.inner.write_i24::<LittleEndian>(buffer[n])?,
                (25..=32,32) => self.inner.write_i32::<LittleEndian>(buffer[n])?,
                (b,_)=> panic!("Unrecognized integer format, bits per sample {}, channels {}, block_alignment {}", 
                    b, self.format.channel_count, self.format.block_alignment)
            }
        }

        Ok( 1 )
    }
}