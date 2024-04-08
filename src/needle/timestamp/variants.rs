use anyhow::{anyhow, Result};

use crate::needle::{number::variants::IntegerVariant, Interpret, Needle, Recombobulate};

#[derive(Clone, Debug, PartialEq)]
pub enum TimestampVariant {
    // Epoch seconds
    EpochSecsLE(Vec<u8>),
    EpochSecsBE(Vec<u8>),
    EpochSecsVarint(Vec<u8>),

    // Epoch millis
    EpochMillisLE(Vec<u8>),
    EpochMillisBE(Vec<u8>),
    EpochMillisVarint(Vec<u8>),

    // Epoch micros
    EpochMicrosLE(Vec<u8>),
    EpochMicrosBE(Vec<u8>),
    EpochMicrosVarint(Vec<u8>),

    // Epoch nanos
    EpochNanosLE(Vec<u8>),
    EpochNanosBE(Vec<u8>),
    EpochNanosVarint(Vec<u8>),

    // DOS time
    DOSTimeLE(Vec<u8>),
    DOSTimeBE(Vec<u8>),
}

impl TimestampVariant {
    pub fn as_epoch_secs_le(data: &[u8]) -> Result<(TimestampVariant, u8)> {
        todo!()
    }
}

impl Recombobulate for TimestampVariant {
    fn recombobulate(&self) -> Result<Needle> {
        match self {
            TimestampVariant::EpochSecsLE(_) => todo!(),
            TimestampVariant::EpochSecsBE(_) => todo!(),
            TimestampVariant::EpochSecsVarint(_) => todo!(),
            TimestampVariant::EpochMillisLE(_) => todo!(),
            TimestampVariant::EpochMillisBE(_) => todo!(),
            TimestampVariant::EpochMillisVarint(_) => todo!(),
            TimestampVariant::EpochMicrosLE(_) => todo!(),
            TimestampVariant::EpochMicrosBE(_) => todo!(),
            TimestampVariant::EpochMicrosVarint(_) => todo!(),
            TimestampVariant::EpochNanosLE(_) => todo!(),
            TimestampVariant::EpochNanosBE(_) => todo!(),
            TimestampVariant::EpochNanosVarint(_) => todo!(),
            TimestampVariant::DOSTimeLE(_) => todo!(),
            TimestampVariant::DOSTimeBE(_) => todo!(),
        }
    }
}

impl Interpret for TimestampVariant {
    fn interpret(data: &[u8]) -> Result<Vec<Self>>
    where
        Self: std::marker::Sized,
    {
        let mut intepretations = Vec::<Self>::new();

        if intepretations.is_empty() {
            Err(anyhow!(
                "Failed to interpret bytes as any valid TimestampVariant!"
            ))
        } else {
            Ok(intepretations)
        }
    }
}
