use integer_encoding::VarInt;
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time};

use super::Discombobulate;

pub fn u8_to_month(value: u8) -> Option<Month> {
    match value {
        1 => Some(Month::January),
        2 => Some(Month::February),
        3 => Some(Month::March),
        4 => Some(Month::April),
        5 => Some(Month::May),
        6 => Some(Month::June),
        7 => Some(Month::July),
        8 => Some(Month::August),
        9 => Some(Month::September),
        10 => Some(Month::October),
        11 => Some(Month::November),
        12 => Some(Month::December),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    pub value: PrimitiveDateTime,
    pub tolerance: Option<Duration>,
}

impl Timestamp {
    pub fn new(value: PrimitiveDateTime) -> Self {
        Self {
            value,
            tolerance: None,
        }
    }

    pub fn with_tolerance(value: PrimitiveDateTime, tolerance: Duration) -> Self {
        Self {
            value,
            tolerance: Some(tolerance),
        }
    }

    /*

        The DOS date/time format is a bitmask:

                        24                16                 8                 0
        +-+-+-+-+-+-+-+-+ +-+-+-+-+-+-+-+-+ +-+-+-+-+-+-+-+-+ +-+-+-+-+-+-+-+-+
        |Y|Y|Y|Y|Y|Y|Y|M| |M|M|M|D|D|D|D|D| |h|h|h|h|h|m|m|m| |m|m|m|s|s|s|s|s|
        +-+-+-+-+-+-+-+-+ +-+-+-+-+-+-+-+-+ +-+-+-+-+-+-+-+-+ +-+-+-+-+-+-+-+-+
        \___________/\________/\_________/ \________/\____________/\_________/
            year        month       day      hour       minute        second

        The year is stored as an offset from 1980.
        Seconds are stored in two-second increments.
        (So if the "second" value is 15, it actually represents 30 seconds.)
    */
    pub fn to_dos_time(&self) -> u32 {
        ((self.value.year() - 1980) as u32) << 25
            | (self.value.month() as u32) << 21
            | (self.value.day() as u32) << 16
            | (self.value.hour() as u32) << 11
            | (self.value.minute() as u32) << 5
            | (self.value.second() as u32) >> 1
    }

    pub fn to_epoch_secs(&self) -> i32 {
        self.value.assume_utc().unix_timestamp() as i32
    }

    pub fn from_dos_time(value: u32) -> Option<Self> {
        let year = ((value >> 25) & 0x7F) + 1980;
        let month = (value >> 21) & 0x0F;
        let day = (value >> 16) & 0x1F;
        let hour = (value >> 11) & 0x1F;
        let minute = (value >> 5) & 0x3F;
        let second = (value & 0x1F) << 1;

        if let Some(month) = u8_to_month(month as u8) {
            if let Ok(date) = Date::from_calendar_date(year as i32, month, day as u8) {
                if let Ok(time) = Time::from_hms(hour as u8, minute as u8, second as u8) {
                    Some(Timestamp::new(PrimitiveDateTime::new(date, time)))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn from_epoch_secs(value: i32) -> Option<Self> {
        if let Ok(dtg) = OffsetDateTime::from_unix_timestamp(value as i64) {
            Some(Timestamp::new(PrimitiveDateTime::new(
                dtg.date(),
                dtg.time(),
            )))
        } else {
            None
        }
    }

    pub fn from_epoch_nanos(value: i64) -> Option<Self> {
        if let Ok(dtg) = OffsetDateTime::from_unix_timestamp_nanos(value as i128) {
            Some(Timestamp::new(PrimitiveDateTime::new(
                dtg.date(),
                dtg.time(),
            )))
        } else {
            None
        }
    }
}

impl Discombobulate for Timestamp {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        let mut variants = Vec::<(Vec<u8>, String)>::new();

        // Epoch seconds
        let epoch_secs = self.value.assume_utc().unix_timestamp() as i32;
        let epoch_secs_le_bytes = epoch_secs.to_le_bytes().as_slice().to_owned();
        variants.push((
            epoch_secs_le_bytes,
            String::from("Epoch Seconds | i32 little endian"),
        ));
        let epoch_secs_be_bytes = epoch_secs.to_be_bytes().as_slice().to_owned();
        variants.push((
            epoch_secs_be_bytes,
            String::from("Epoch Seconds | i32 big endian"),
        ));
        let epoch_secs_varint = epoch_secs.encode_var_vec();
        variants.push((
            epoch_secs_varint,
            String::from("Epoch Seconds | i32 varint"),
        ));

        // Epoch millis
        let epoch_millis = (epoch_secs as i64) * 1000;
        let epoch_millis_le_bytes = epoch_millis.to_le_bytes().as_slice().to_owned();
        variants.push((
            epoch_millis_le_bytes,
            String::from("Epoch Millis | i64 little endian"),
        ));
        let epoch_millis_be_bytes = epoch_millis.to_be_bytes().as_slice().to_owned();
        variants.push((
            epoch_millis_be_bytes,
            String::from("Epoch Millis | i64 big endian"),
        ));
        let epoch_millis_varint = epoch_millis.encode_var_vec();
        variants.push((
            epoch_millis_varint,
            String::from("Epoch Millis | i64 varint"),
        ));

        // Epoch micros
        let epoch_micros = epoch_millis * 1000;
        let epoch_micros_le_bytes = epoch_micros.to_le_bytes().as_slice().to_owned();
        variants.push((
            epoch_micros_le_bytes,
            String::from("Epoch Micros | i64 little endian"),
        ));
        let epoch_micros_be_bytes = epoch_micros.to_be_bytes().as_slice().to_owned();
        variants.push((
            epoch_micros_be_bytes,
            String::from("Epoch Micros | i64 big endian"),
        ));
        let epoch_micros_varint = epoch_micros.encode_var_vec();
        variants.push((
            epoch_micros_varint,
            String::from("Epoch Micros | i64 varint"),
        ));

        // Epoch nanos
        let epoch_nanos = epoch_micros * 1000;
        let epoch_nanos_le_bytes = epoch_nanos.to_le_bytes().as_slice().to_owned();
        variants.push((
            epoch_nanos_le_bytes,
            String::from("Epoch Nanos | i64 little endian"),
        ));
        let epoch_nanos_be_bytes = epoch_nanos.to_be_bytes().as_slice().to_owned();
        variants.push((
            epoch_nanos_be_bytes,
            String::from("Epoch Nanos | i64 big endian"),
        ));
        let epoch_nanos_varint = epoch_nanos.encode_var_vec();
        variants.push((epoch_nanos_varint, String::from("Epoch Nanos | i64 varint")));

        // 18-digit 'Windows NT time format', 'Win32 FILETIME or SYSTEMTIME' or NTFS file time
        // The timestamp is the number of 100-nanosecond intervals (1 nanosecond = one billionth of a second) since Jan 1, 1601 UTC

        // WebKit/Chrome timestamps
        // A 64-bit value for microseconds since Jan 1, 1601 00:00 UTC. One microsecond is one-millionth of a second

        // Apple Cocoa Core Data timestamp
        // The number of seconds (or nanoseconds) since midnight, January 1, 2001
        // Difference between this and epoch is 978307200 seconds

        // Mac HFS+ timestamp
        // Seconds since January 1, 1904 (add 2082844800 to epoch)

        // SAS 4GL datetime
        // Seconds since January 1, 1960 (add 315619200 to epoch)

        // DOS/FAT timestamp
        let dos_time = self.to_dos_time();
        let dos_time_le_bytes = dos_time.to_le_bytes().as_slice().to_owned();
        variants.push((
            dos_time_le_bytes,
            String::from("DOS Time | u32 little endian"),
        ));
        let dos_time_be_bytes = dos_time.to_be_bytes().as_slice().to_owned();
        variants.push((dos_time_be_bytes, String::from("DOS Time | u32 big endian")));

        // NTP timestamp

        variants
    }
}

#[cfg(test)]
mod tests {
    use time::{format_description, macros::datetime};

    use super::*;

    #[test]
    fn dos_time_test() {
        let format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();

        let dtg = Timestamp::new(datetime!(2023-12-31 23:59:58));
        println!("DTG: {}", dtg.value.format(&format).unwrap());

        let dos_time = dtg.to_dos_time();
        println!("DOS: {}", dos_time);

        let dtg2 = Timestamp::from_dos_time(dos_time).unwrap();
        println!("DTG: {}", dtg2.value.format(&format).unwrap());

        assert_eq!(dtg, dtg2);

        let dos2 = Timestamp::from_dos_time(0x5822728e).unwrap();
        println!("dos2: {}", dos2.value.format(&format).unwrap());
    }

    #[test]
    fn timestamp_test() {
        let format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();

        let dtg = Timestamp::new(datetime!(2023-12-31 23:59:58));
        println!("DTG: {}", dtg.value.format(&format).unwrap());

        let variants = dtg.discombobulate();
        for variant in &variants {
            println!("{:02x?}", variant);
        }

        let dos_time = dtg.to_dos_time();
        println!("DOS: {}", dos_time);

        let dtg2 = Timestamp::from_dos_time(dos_time).unwrap();
        println!("DTG: {}", dtg2.value.format(&format).unwrap());

        assert_eq!(dtg, dtg2);

        let dtg = Timestamp::new(datetime!(1969-12-31 23:59:59));
        println!("{:?}", &dtg);
        println!("{:?}", &dtg.to_epoch_secs());
        for variant in dtg.discombobulate() {
            println!("{:02x?}", variant);
        }

        let nye23 = Timestamp::new(datetime!(2023-12-31 23:59:59)).to_dos_time();

        let i1: i32 = -10;
        let v = i1.encode_var_vec();

        let i2 = i32::decode_var(&v).unwrap().0;
        println!("{} -> {:02x?} -> {}", i1, v, i2);

        assert_eq!(1, 1);
    }
}
