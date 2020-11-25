use log::*;

use crate::wire::*;


/// A **TXT** record, which holds arbitrary descriptive text.
///
/// # Encoding
///
/// The text encoding is not specified, but this crate treats it as UTF-8.
/// Invalid bytes are turned into the replacement character.
///
/// # References
///
/// - [RFC 1035 §3.3.14](https://tools.ietf.org/html/rfc1035) — Domain Names,
///   Implementation and Specification (November 1987)
#[derive(PartialEq, Debug)]
pub struct TXT {

    /// The messages contained in the record.
    pub messages: Vec<String>,
}

impl Wire for TXT {
    const NAME: &'static str = "TXT";
    const RR_TYPE: u16 = 16;

    #[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
    fn read(stated_length: u16, c: &mut Cursor<&[u8]>) -> Result<Self, WireError> {
        let mut messages = Vec::new();
        let mut total_length = 0_u16;

        loop {
            let mut buf = Vec::new();

            loop {
                let next_length = c.read_u8()?;
                total_length += u16::from(next_length) + 1;
                trace!("Parsed slice length -> {:?} (total so far {:?})", next_length, total_length);

                for _ in 0 .. next_length {
                    buf.push(c.read_u8()?);
                }

                if next_length < 255 {
                    break;
                }
                else {
                    trace!("Got length 255, so looping");
                }
            }

            let message = String::from_utf8_lossy(&buf).to_string();
            trace!("Parsed message -> {:?}", message);
            messages.push(message);

            if total_length >= stated_length {
                break;
            }
        }

        if stated_length == total_length {
            trace!("Length is correct");
            Ok(Self { messages })
        }
        else {
            warn!("Length is incorrect (stated length {:?}, messages length {:?})", stated_length, total_length);
            Err(WireError::WrongLabelLength { stated_length, length_after_labels: total_length })
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_one_iteration() {
        let buf = &[
            0x06,  // message chunk length
            0x74, 0x78, 0x74, 0x20, 0x6d, 0x65,  // message chunk
        ];

        assert_eq!(TXT::read(buf.len() as _, &mut Cursor::new(buf)).unwrap(),
                   TXT {
                       messages: vec![ String::from("txt me") ],
                   });
    }

    #[test]
    fn parses_two_iterations() {
        let buf = &[
            0xFF,  // message chunk length
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
            0x41, 0x41,  // exactly two hundred and fifty five ‘A’s (screaming)
            0x04,  // message chunk length
            0x41, 0x41, 0x41, 0x41,  // four more ‘A’s (the scream abruptly stops)
        ];

        assert_eq!(TXT::read(buf.len() as _, &mut Cursor::new(buf)).unwrap(),
                   TXT {
                       messages: vec![
                           String::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                         AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                         AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                         AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                         AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                         AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                         AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                         AAAAAAAAAAAAAAAAAAAAAAAAAAAAA\
                                         AAAAAAAAAAAAAAAAAAAAAAAAAAA"),
                       ],
                   });
        // did you know you can just _write_ code like this, and nobody will stop you?
    }

    #[test]
    fn right_at_the_limit() {
        let buf = &[
            0xFE,  // message chunk length
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42,  // exactly two hundred and fifty four ‘B’s (a hive)
        ];

        assert_eq!(TXT::read(buf.len() as _, &mut Cursor::new(buf)).unwrap(),
                   TXT {
                       messages: vec![
                           String::from("BBBBBBBBBBBBBBBBBBBBBBBBBBBBB\
                                         BBBBBBBBBBBBBBBBBBBBBBBBBBBBB\
                                         BBBBBBBBBBBBBBBBBBBBBBBBBBBBB\
                                         BBBBBBBBBBBBBBBBBBBBBBBBBBBBB\
                                         BBBBBBBBBBBBBBBBBBBBBBBBBBBBB\
                                         BBBBBBBBBBBBBBBBBBBBBBBBBBBBB\
                                         BBBBBBBBBBBBBBBBBBBBBBBBBBBBB\
                                         BBBBBBBBBBBBBBBBBBBBBBBBBBBBB\
                                         BBBBBBBBBBBBBBBBBBBBBB"),
                       ],
                   });
    }

    #[test]
    fn another_message() {
        let buf = &[
            0x06,  // message chunk length
            0x74, 0x78, 0x74, 0x20, 0x6d, 0x65,  // message chunk
            0x06,  // message chunk length
            0x79, 0x61, 0x20, 0x62, 0x65, 0x62,  // message chunk
        ];

        assert_eq!(TXT::read(buf.len() as _, &mut Cursor::new(buf)).unwrap(),
                   TXT {
                       messages: vec![
                           String::from("txt me"),
                           String::from("ya beb"),
                       ],
                   });
    }

    #[test]
    fn length_too_short() {
        let buf = &[
            0x06,  // message chunk length
            0x74, 0x78, 0x74, 0x20, 0x6d, 0x65,  // message chunk
        ];

        assert_eq!(TXT::read(2, &mut Cursor::new(buf)),
                   Err(WireError::WrongLabelLength { stated_length: 2, length_after_labels: 7 }));
    }

    #[test]
    fn record_empty() {
        assert_eq!(TXT::read(0, &mut Cursor::new(&[])),
                   Err(WireError::IO));
    }

    #[test]
    fn buffer_ends_abruptly() {
        let buf = &[
            0x06, 0x74,  // the start of a message
        ];

        assert_eq!(TXT::read(23, &mut Cursor::new(buf)),
                   Err(WireError::IO));
    }
}
