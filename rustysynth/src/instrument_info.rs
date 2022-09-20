use std::error;
use std::io;

use crate::binary_reader;

pub(crate) struct InstrumentInfo
{
    pub(crate) name: String,
    pub(crate) zone_start_index: i32,
    pub(crate) zone_end_index: i32,
}

impl InstrumentInfo
{
    fn new<R: io::Read>(reader: &mut R) -> Result<Self, Box<dyn error::Error>>
    {
        let name = match binary_reader::read_fixed_length_string(reader, 20)
        {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        let zone_start_index = match binary_reader::read_u16(reader)
        {
            Ok(value) => value as i32,
            Err(error) => return Err(Box::new(error)),
        };

        Ok(InstrumentInfo
        {
            name: name,
            zone_start_index: zone_start_index,
            zone_end_index: 0,
        })
    }
}

pub(crate) fn read_from_chunk<R: io::Read>(reader: &mut R, size: i32) -> Result<Vec<InstrumentInfo>, Box<dyn error::Error>>
{
    if size % 22 != 0
    {
        return Err(format!("The instrument list is invalid.").into());
    }

    let count = size / 22;

    let mut instruments: Vec<InstrumentInfo> = Vec::new();
    for _i in 0..count
    {
        match InstrumentInfo::new(reader)
        {
            Ok(value) => instruments.push(value),
            Err(error) => return Err(error),
        }
    }

    for i in 0..(count - 1) as usize
    {
        instruments[i].zone_end_index = instruments[i + 1].zone_start_index - 1;
    }

    Ok(instruments)
}