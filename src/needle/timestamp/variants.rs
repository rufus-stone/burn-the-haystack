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
