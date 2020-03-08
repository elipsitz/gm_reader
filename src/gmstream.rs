extern crate byteorder;
extern crate encoding_rs;

use std::io;
use std::io::{Read, Take};
use byteorder::{ReadBytesExt, LittleEndian};
use flate2::read::ZlibDecoder;

pub trait GmStream: Sized {
    fn read_u32(&mut self) -> io::Result<u32>;

    fn read_u16(&mut self) -> io::Result<u16>;

    fn read_u8(&mut self) -> io::Result<u8>;

    fn read_bool(&mut self) -> io::Result<bool>;

    fn read_string(&mut self) -> io::Result<String>;

    fn read_f64(&mut self) -> io::Result<f64>;

    fn skip(&mut self, bytes: u32) -> io::Result<()>;

    fn read_compressed(&mut self) -> io::Result<ZlibDecoder<Take<&mut Self>>>;

    fn skip_section(&mut self) -> io::Result<()>;
}

impl<T: Read> GmStream for T {
    fn read_u32(&mut self) -> io::Result<u32> {
        ReadBytesExt::read_u32::<LittleEndian>(self)
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        ReadBytesExt::read_u16::<LittleEndian>(self)
    }

    fn read_u8(&mut self) -> io::Result<u8> {
        ReadBytesExt::read_u8(self)
    }

    fn read_bool(&mut self) -> io::Result<bool> {
        Ok(GmStream::read_u32(self)? > 0)
    }

    fn read_string(&mut self) -> io::Result<String> {
        let length = GmStream::read_u32(self)?;
        let mut data = Vec::with_capacity(length as usize);
        self.take(length as u64).read_to_end(&mut data)?;
        let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(&data);
        let string = decoded.to_string();
        Ok(string)
    }

    fn read_f64(&mut self) -> io::Result<f64> {
        ReadBytesExt::read_f64::<LittleEndian>(self)
    }

    fn skip(&mut self, bytes: u32) -> io::Result<()> {
        // println!("Skipping {} bytes", bytes);
        let mut sub = self.take(bytes as u64);
        match io::copy(&mut sub, &mut io::sink()) {
            Err(e) => Err(e),
            Ok(_) => Ok(())
        }
    }

    fn skip_section(&mut self) -> io::Result<()> {
        let length = GmStream::read_u32(self)?;
        self.skip(length)
    }

    fn read_compressed(&mut self) -> io::Result<ZlibDecoder<Take<&mut T>>> {
        let length = GmStream::read_u32(self)?;
        let substream = self.take(length as u64);
        let decoder = ZlibDecoder::new(substream);

        Ok(decoder)
    }
}