use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

use super::data_packet::DataPacket;

pub fn get_data_end(rdr: &Cursor<&std::vec::Vec<u8>>, use_crc: bool) -> u64 {
    if use_crc {
        return (rdr.get_ref().len() as u64) - 2_u64;
    } else {
        return rdr.get_ref().len() as u64;
    };
}

pub fn write_data_length(wtr: &mut Vec<u8>, data_length: usize) {
    if data_length <= 0xFF {
        wtr.write_u8(data_length as u8).unwrap_or_default();
    } else if data_length <= 0xFFFF {
        wtr.write_u16::<BigEndian>(data_length as u16)
            .unwrap_or_default();
    } else {
        wtr.write_u32::<BigEndian>(data_length as u32)
            .unwrap_or_default();
    }
}

pub fn read_data_length(rdr: &mut Cursor<&std::vec::Vec<u8>>) -> u32 {
    let initial_rdr_position = rdr.position();
    let mut data_length: u32 = rdr.read_u8().unwrap_or_default() as u32;
    if data_length > 0xFF {
        rdr.set_position(initial_rdr_position);
        data_length = rdr.read_u16::<BigEndian>().unwrap_or_default() as u32;
        if data_length > 0xFFFF {
            rdr.set_position(initial_rdr_position);
            data_length = rdr.read_u32::<BigEndian>().unwrap_or_default();
        }
    }
    data_length
}

pub fn extract_subpacket_data(
    rdr: &Cursor<&std::vec::Vec<u8>>,
    data_start_position: u64,
    sub_packet_data_length: u32,
) -> Vec<u8> {
    let full_data_vec = rdr.get_ref().to_vec();
    full_data_vec[data_start_position as usize
        ..(data_start_position + sub_packet_data_length as u64) as usize]
        .to_vec()
}

pub fn write_packet_data(wtr: &mut Vec<u8>, data_packet: &mut DataPacket) {
    wtr.write_u16::<BigEndian>(data_packet.sequence)
        .unwrap_or_default();
    wtr.append(data_packet.get_data_mut());
}

#[cfg(test)]
mod tests {
    use crate::soeprotocol::{data_packet::DataPacket, protocol::SoeOpcode};

    #[test]
    fn write_packet_data_test() {
        let data_to_pack: Vec<u8> = [
            2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0,
        ]
        .to_vec();
        let mut wtr = vec![];
        let mut data_packet = DataPacket::new(data_to_pack, 0, SoeOpcode::Data as u16);
        super::write_packet_data(&mut wtr, &mut data_packet);
        assert_eq!(
            wtr,
            [0, 0, 2, 1, 1, 0, 0, 0, 1, 1, 3, 0, 0, 0, 115, 111, 101, 0, 0, 0, 0].to_vec()
        )
    }
}
