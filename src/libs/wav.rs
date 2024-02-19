use byteorder::{LittleEndian, ReadBytesExt};
use std::str;
use std::{
    fs::{read, File},
    io::{Error, Read, Seek, Write},
    ops::Add,
    path::Path,
};

type Fourcc = [u8; 4];

pub struct RiffHdr {
    id: Fourcc,
    pub size: u32,
    block_type: Fourcc,
}

pub struct FmtHdr {
    id: Fourcc,
    pub size: u32,
    pub fmt_tag: u16,
    pub channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

pub struct DataHdr {
    id: Fourcc,
    pub size: u32,
}

pub struct WavHdr {
    pub riff_hdr: RiffHdr,
    pub fmt_ck: FmtHdr,
    pub data_hdr: DataHdr,
}

pub struct WavFile {
    pub hdr: WavHdr,
    pub data: Vec<BitDepth>,
}

pub struct WavParams {
    pub sample_rate: u32,
    pub channels: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum BitDepth {
    U8(i8),
    U16(i16),
    U32(i32),
}

impl Add for BitDepth {
    type Output = BitDepth;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            BitDepth::U8(l) => match rhs {
                BitDepth::U8(r) => BitDepth::U8(l + r),
                BitDepth::U16(r) => BitDepth::U16(l as i16 + r),
                BitDepth::U32(r) => BitDepth::U32(l as i32 + r),
            },
            BitDepth::U16(l) => match rhs {
                BitDepth::U8(r) => BitDepth::U16(l + r as i16),
                BitDepth::U16(r) => BitDepth::U16(l + r),
                BitDepth::U32(r) => BitDepth::U32(l as i32 + r),
            },
            BitDepth::U32(l) => match rhs {
                BitDepth::U8(r) => BitDepth::U32(l + r as i32),
                BitDepth::U16(r) => BitDepth::U32(l + r as i32),
                BitDepth::U32(r) => BitDepth::U32(l + r),
            },
        }
    }
}

impl WavFile {
    pub fn new(params: WavParams, data: Vec<BitDepth>) -> WavFile {
        let bit_depth = match data[0] {
            BitDepth::U8(_) => 8,
            BitDepth::U16(_) => 16,
            BitDepth::U32(_) => 32,
        };
        println!("bit_depth: {}", bit_depth);
        let mut size = bit_depth as u32 * data.len() as u32;
        if size % 2 != 0 {
            size += 1;
        }
        WavFile {
            hdr: WavHdr {
                riff_hdr: RiffHdr {
                    id: *b"RIFF",
                    size: 36 + size,
                    block_type: *b"WAVE",
                },
                fmt_ck: FmtHdr {
                    id: *b"fmt ",
                    size: 16,
                    fmt_tag: 1,
                    channels: params.channels,
                    sample_rate: params.sample_rate,
                    byte_rate: params.sample_rate * params.channels as u32 * bit_depth as u32 / 8,
                    block_align: params.channels * (bit_depth as u16),
                    bits_per_sample: bit_depth as u16,
                },
                data_hdr: DataHdr { id: *b"data", size },
            },
            data: data,
        }
    }
    pub fn write(self, path: &Path) -> std::io::Result<()> {
        let mut f = File::create(path).expect("Unable to create file");
        // RIFF header
        f.write_all(&self.hdr.riff_hdr.id).unwrap();
        f.write_all(&self.hdr.riff_hdr.size.to_le_bytes()).unwrap();
        f.write_all(&self.hdr.riff_hdr.block_type).unwrap();
        // fmt chunk
        f.write_all(&self.hdr.fmt_ck.id).unwrap();
        f.write_all(&self.hdr.fmt_ck.size.to_le_bytes()).unwrap();
        f.write_all(&self.hdr.fmt_ck.fmt_tag.to_le_bytes()).unwrap();
        f.write_all(&self.hdr.fmt_ck.channels.to_le_bytes())
            .unwrap();
        f.write_all(&self.hdr.fmt_ck.sample_rate.to_le_bytes())
            .unwrap();
        f.write_all(&self.hdr.fmt_ck.byte_rate.to_le_bytes())
            .unwrap();
        f.write_all(&self.hdr.fmt_ck.block_align.to_le_bytes())
            .unwrap();
        f.write_all(&self.hdr.fmt_ck.bits_per_sample.to_le_bytes())
            .unwrap();
        // data chunk
        f.write_all(&self.hdr.data_hdr.id).unwrap();
        f.write_all(&self.hdr.data_hdr.size.to_le_bytes()).unwrap();
        // data
        let pos_data_start = f.stream_position()?;
        for d in self.data {
            match d {
                BitDepth::U8(d) => {
                    f.write_all(&d.to_le_bytes()).unwrap();
                }
                BitDepth::U16(d) => {
                    f.write_all(&d.to_le_bytes()).unwrap();
                }
                BitDepth::U32(d) => {
                    f.write_all(&d.to_le_bytes()).unwrap();
                }
            }
        }
        let pos_end = f.stream_position()?;
        // Pad with zeroes to make the file size a multiple of 2
        let chunk_size_data: u32 = (pos_end - pos_data_start) as u32;
        if chunk_size_data % 2 != 0 {
            f.write_all(&[0x00])?;
        }

        f.sync_all().unwrap();
        Ok(())
    }
    pub fn read(path: &Path) -> Result<WavFile, Error> {
        // Reads the whole file into an in memory vector
        let file = match read(path) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let invalid_file_error =
            Error::new(std::io::ErrorKind::InvalidInput, "Not a valid WAV file");
        let header = match str::from_utf8(&file[0..4]) {
            Ok(v) => v,
            Err(e) => return Err(invalid_file_error),
        };

        match header {
            "RIFF" => (),
            _ => return Err(invalid_file_error),
        };

        let channels = (&file[22..24]).read_u16::<LittleEndian>().unwrap();
        let sample_size = (&file[32..36]).read_u16::<LittleEndian>().unwrap();
        let sample_rate = (&file[24..28]).read_u32::<LittleEndian>().unwrap();
        let size = (&file[40..44]).read_u32::<LittleEndian>().unwrap();
        println!("File size: {size}, sample size: {sample_size}, sample size: {sample_rate}, {channels} channels");
        // let samples = &file[44..];
        let mut data = Vec::new();

        match sample_size {
            8 => {
                for s in 0..(size / sample_size as u32) {
                    let start = (44 + s * 1) as usize;
                    let end = start + 1;
                    data.push(BitDepth::U8((&file[start..end]).read_i8().unwrap()));
                }
            }
            16 => {
                for s in 0..(size / sample_size as u32) {
                    let start = (44 + s * 2) as usize;
                    let end = start + 2;
                    data.push(BitDepth::U16(
                        (&file[start..end]).read_i16::<LittleEndian>().unwrap(),
                    ))
                }
            }
            32 => {
                for s in 0..(size / sample_size as u32) {
                    let start = (44 + s * 4) as usize;
                    let end = start + 4;
                    data.push(BitDepth::U32(
                        (&file[start..end]).read_i32::<LittleEndian>().unwrap(),
                    ))
                }
            }
            _ => return Err(invalid_file_error),
        };

        return Ok(WavFile::new(
            WavParams {
                channels,
                sample_rate,
            },
            data,
        ));
    }
}
