use byteorder;
use std::io::Cursor;
use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

pub fn from_big_endian(arr: &[u8], size: u16) -> u16 {
    let mut rdr = Cursor::new(arr);
    rdr.read_u16::<BigEndian>().unwrap()
}
pub fn from_little_endian(arr: &[u8], size: u16) -> u16 {
    let mut rdr = Cursor::new(arr);
    rdr.read_u16::<LittleEndian>().unwrap()
}
pub fn to_big_endian<'a>(num: u16) -> Vec<u8> {
    let mut wtr = vec![];
    wtr.write_u16::<BigEndian>(num).unwrap();
    wtr
}
pub fn to_little_endian<'a>(num: u16) -> Vec<u8> {
    let mut wtr = vec![];
    wtr.write_u16::<LittleEndian>(num).unwrap();
    wtr
}

#[test]
fn test_tolittle() {
    let arr = &[1, 0];
    let num = from_big_endian(arr, 2);
    assert_eq!(num, 256);
    let arr_le = to_little_endian(num);
    assert_eq!(arr_le, &[0, 1]);
} 