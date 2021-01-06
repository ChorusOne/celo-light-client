use crate::types::header::Header;
use crate::types::istanbul::{IstanbulExtra, IstanbulExtraVanity,  IstanbulAggregatedSeal};
use crate::traits::default::FromBytes;
use crate::errors::Error;

// TODO: This file is temprory holder for those functions, clean this up afterwards
pub const EPOCH_SIZE: u64 = 17280;

pub fn get_epoch_number(number: u64, epoch_size: u64) -> u64 {
    let epoch_number = number / epoch_size;

    if is_last_block_of_epoch(number, epoch_size) {
        epoch_number
    } else {
        epoch_number + 1
    }
}

pub fn is_last_block_of_epoch(number: u64, epoch_size: u64) -> bool {
    get_number_within_epoch(number, epoch_size) == epoch_size
}

// Retrieves the block number within an epoch.  The return value will be 1-based.
// There is a special case if the number == 0.  It is basically the last block of the 0th epoch, and should have a value of epoch_size
pub fn get_number_within_epoch(number: u64, epoch_size: u64) -> u64 {
    if number % epoch_size == 0 {
        epoch_size
    } else {
        number
    }
}

pub fn get_epoch_first_block_number(epoch_number: u64, epoch_size: u64) -> Option<u64> {
    if epoch_number == 0 {
        // no first block for epoch 0
        return None
    }

    Some(((epoch_number - 1) * epoch_size) + 1)
}

pub fn get_epoch_last_block_number(epoch_number: u64, epoch_size: u64) -> u64 {
    if epoch_number == 0 {
        return 0
    }

    let first_block_num = get_epoch_first_block_number(epoch_number, epoch_size).unwrap();
    first_block_num + (epoch_size - 1)
}

pub fn find_epoch_block_number(number: u64, epoch_size: u64) -> u64 {
    let epoch = get_epoch_number(number, epoch_size);
    let epoch_block_number = get_epoch_last_block_number(epoch-1, epoch_size);
    if number == epoch_block_number {
        get_epoch_last_block_number(epoch-2, epoch_size);
    }

    epoch_block_number
}

pub fn istanbul_filtered_header(header: &Header, keep_seal: bool) -> Result<Header, Error> {
    let mut new_header = header.clone();

    let mut extra = IstanbulExtra::from_rlp(&new_header.extra)?;
    if !keep_seal {
        extra.seal = Vec::new();
    }
    extra.aggregated_seal = IstanbulAggregatedSeal::new();

    let payload = extra.to_rlp(IstanbulExtraVanity::from_bytes(&new_header.extra).unwrap()); // todo pass ?
    new_header.extra = payload;

    Ok(new_header)
}
