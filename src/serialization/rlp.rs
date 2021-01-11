use crate::traits::FromBytes;
use rlp::{DecoderError, Rlp};
use rug::{integer::Order, Integer};

pub fn rlp_list_field_from_bytes<T>(rlp: &Rlp, index: usize) -> Result<T, DecoderError> where T: FromBytes + Clone {
    rlp.at(index)?.decoder().decode_value(|data| {
        match T::from_bytes(data) {
            Ok(field) => Ok(field.to_owned()),
            Err(_) => Err(DecoderError::Custom("invalid length data")),
        }
    })
}

pub fn rlp_field_from_bytes<T>(rlp: &Rlp) -> Result<T, DecoderError> where T: FromBytes + Clone {
    rlp.decoder().decode_value(|data| {
        match T::from_bytes(data) {
            Ok(field) => Ok(field.to_owned()),
            Err(_) => Err(DecoderError::Custom("invalid length data")),
        }
    })
}

pub fn rlp_to_big_int(rlp: &Rlp, index: usize) -> Result<Integer, DecoderError> {
    rlp.at(index)?.decoder().decode_value(
        |bytes| Ok(Integer::from_digits(bytes, Order::Msf))
    )
}
