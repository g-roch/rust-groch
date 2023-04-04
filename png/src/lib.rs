#![feature(result_option_inspect)]
pub mod tools;

use bytesize::ByteSize;
use crc::Crc;
use crc::CRC_32_ISO_HDLC;
use log::{debug, error, info, trace, warn};
use std::io::Read;
use tools::*;

const HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

pub struct Chunk<'a> {
    name: &'a [u8],
    data: &'a [u8],
}
pub struct Png<'a> {
    chunk_pos: usize,
    data: &'a [u8],
}
impl<'a> Png<'a> {
    pub fn next_chunk(&mut self) -> Option<Chunk<'a>> {
        if self.data.is_empty() {
            trace!("Dernier chunk atteint");
            return None;
        }
        trace!(
            "Lectue d'un chunk (Donnée encore à lire = {})",
            ByteSize(self.data.len() as u64),
        );
        if self.data.len() < 8 {
            error!("Pas assez de donnée pour contenir une en-tête de chunk");
        }

        let length;
        (length, self.data) = self.data.split_at(4);
        let length = u32::from_be_bytes(length.try_into().unwrap());
        trace!("Taille du chunk : {}", ByteSize(length as u64));

        let name_bytes;
        (name_bytes, self.data) = self.data.split_at(4);
        let name = String::from_utf8(name_bytes.into())
            .log_error("Chunk's name isn't valid")
            .map_err(|_| name_bytes)
            .inspect(|s| {
                if s.find(|c: char| !(c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z'))
                    .is_some()
                {
                    error!("Chunk's name isn't valid {s:?}")
                }
            });
        trace!("chunk name : {name:?}");

        let data;
        (data, self.data) = if self.data.len() < length as usize {
            error!("Pas assez de donnée pour contenir le chunk");
            self.data.split_at(self.data.len())
        } else {
            self.data.split_at(length as usize)
        };
        if self.data.len() < 4 {
            error!("Pas assez de donnée pour contenir le CRC");
        } else {
            let crc;
            (crc, self.data) = self.data.split_at(4);
            let crc = u32::from_be_bytes(crc.try_into().unwrap());
            let crc_building = Crc::<u32>::new(&CRC_32_ISO_HDLC);
            let mut digest = crc_building.digest();
            digest.update(&name_bytes);
            digest.update(&data);
            let crc_calc = digest.finalize();
            if crc_calc == crc {
                trace!("CRC valide");
            } else {
                warn!("CRC invalide obtain {crc_calc:?} but in file is {crc:?} ");
            }
        }

        if self.chunk_pos == 0 && name_bytes != b"IHDR" {
            error!("First chunk isn't IHDR");
        }
        if !self.data.is_empty() && name_bytes == b"IEND" {
            error!("IEND isn't last chunk");
        }
        if self.data.is_empty() && name_bytes != b"IEND" {
            error!("Last chunk isn't IEND");
        }

        self.chunk_pos += 1;
        Some(Chunk {
            name: name_bytes,
            data,
        })
    }
}
impl<'a> From<&'a [u8]> for Png<'a> {
    fn from(mut data: &'a [u8]) -> Self {
        let mut header = [0u8; HEADER.len()];
        data.read_exact(&mut header)
            .log_expect("Can't read headers bytes");
        if header == HEADER {
            trace!("Header is ok");
        } else {
            warn!("Header is NOK Expected = {HEADER:?}; Optain = {header:?}");
        }
        Self { chunk_pos: 0, data }
    }
}
impl<'a> Iterator for Png<'a> {
    type Item = Chunk<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_chunk()
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
