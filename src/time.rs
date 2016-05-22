#[derive(Eq, PartialEq, Debug)]
pub struct Time {
    pub hour: u32,
    pub minute: u32
}

impl Time {
    pub fn parse(text: String) -> Option<Time> {
        let separator_index = text.find(':');

        if separator_index.is_none() {
            return None;
        }

        let hour_part = &text[..separator_index.unwrap()];
        let minute_part = &text[separator_index.unwrap()+1..];

        let parsed_hour = hour_part.parse::<u32>();
        let parsed_minute = minute_part.parse::<u32>();

        if parsed_hour.is_err() || parsed_minute.is_err() {
            return None;
        }

        let hour = parsed_hour.unwrap();
        let minute = parsed_minute.unwrap();

        if hour >= 24 || minute >= 60 {
            return None;
        }

        return Some(Time { hour: hour, minute: minute });
    }
}

#[cfg(test)]
mod test {
    use super::Time;

    #[test]
    fn parse_00_00() {
        assert_eq!(Some(Time { hour: 0, minute: 0 }), Time::parse("00:00".to_string()));
    }

    #[test]
    fn parse_12_00() {
        assert_eq!(Some(Time { hour: 12, minute: 0 }), Time::parse("12:00".to_string()));
    }

    #[test]
    fn parse_3_45() {
        assert_eq!(Some(Time { hour: 3, minute: 45 }), Time::parse("3:45".to_string()));
    }

    #[test]
    fn parse_03_45() {
        assert_eq!(Some(Time { hour: 3, minute: 45 }), Time::parse("03:45".to_string()));
    }

    #[test]
    fn parse_23_59() {
        assert_eq!(Some(Time { hour: 23, minute: 59 }), Time::parse("23:59".to_string()));
    }

    #[test]
    fn parse_no_hour() {
        assert_eq!(None, Time::parse(":00".to_string()));
    }

    #[test]
    fn parse_no_minute() {
        assert_eq!(None, Time::parse("12:".to_string()));
    }

    #[test]
    fn parse_colon() {
        assert_eq!(None, Time::parse(":".to_string()));
    }

    #[test]
    fn parse_empty_string() {
        assert_eq!(None, Time::parse("".to_string()));
    }

    #[test]
    fn parse_03_60() {
        assert_eq!(None, Time::parse("03:60".to_string()));
    }

    #[test]
    fn parse_24_00() {
        assert_eq!(None, Time::parse("24:00".to_string()));
    }

    #[test]
    fn parse_12_34_56() {
        assert_eq!(None, Time::parse("12:34:56".to_string()));
    }

    #[test]
    fn parse_0a_0a() {
        assert_eq!(None, Time::parse("0a:0a".to_string()));
    }
}
