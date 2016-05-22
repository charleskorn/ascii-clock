use std::char;

pub struct Time {
    hour: u32,
    minute: u32
}

pub fn generate_clock(time: Time) -> String {
    let mut clock = r#"        A
    L       B

 K             C

J               D

 I             E

    H       F
        G"#.to_string();

    let hour_location = time.hour % 12;
    let minute_location = time.minute / 5;

    for location in 0..12 {
        let mut symbol = "o";
        let is_desired_hour_location = location == hour_location;
        let is_desired_minute_location = location == minute_location;

        if is_desired_hour_location && is_desired_minute_location {
            symbol = "x";
        } else if is_desired_hour_location {
            symbol = "h";
        } else if is_desired_minute_location {
            symbol = "m";
        }

        let marker = char::from_u32('A' as u32 + location).unwrap();
        clock = clock.replace(marker, symbol);
    }

    return clock;
}

#[cfg(test)]
mod test {
    use super::generate_clock;
    use super::Time;

    #[test]
    fn generate_clock_00_00() {
        let expected_clock = r#"        x
    o       o

 o             o

o               o

 o             o

    o       o
        o"#;
        let time = Time { hour: 0, minute: 0 };
        assert_eq!(expected_clock, generate_clock(time));
    }

    #[test]
    fn generate_clock_12_00() {
        let expected_clock = r#"        x
    o       o

 o             o

o               o

 o             o

    o       o
        o"#;
        let time = Time { hour: 12, minute: 0 };
        assert_eq!(expected_clock, generate_clock(time));
    }

    #[test]
    fn generate_clock_3_00() {
        let expected_clock = r#"        m
    o       o

 o             o

o               h

 o             o

    o       o
        o"#;
        let time = Time { hour: 3, minute: 0 };
        assert_eq!(expected_clock, generate_clock(time));
    }

    #[test]
    fn generate_clock_0_59() {
        let expected_clock = r#"        h
    m       o

 o             o

o               o

 o             o

    o       o
        o"#;
        let time = Time { hour: 0, minute: 59 };
        assert_eq!(expected_clock, generate_clock(time));
    }

    #[test]
    fn generate_clock_3_30() {
        let expected_clock = r#"        o
    o       o

 o             o

o               h

 o             o

    o       o
        m"#;
        let time = Time { hour: 3, minute: 30 };
        assert_eq!(expected_clock, generate_clock(time));
    }

    #[test]
    fn generate_clock_13_30() {
        let expected_clock = r#"        o
    o       h

 o             o

o               o

 o             o

    o       o
        m"#;
        let time = Time { hour: 13, minute: 30 };
        assert_eq!(expected_clock, generate_clock(time));
    }
}
