#[derive(Debug, Copy, Clone)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16
}

impl Date {
    pub fn total_days(&self) -> usize {
        let mut this = self.clone();

        if this.month <= 2 {
            this.month += 12;
            this.year -= 1;
        }

        let era = (this.year / 400) as usize;
        let year_of_era = (this.year % 400) as usize;

        let day_of_year = (306 * (this.month as usize) + 5) / 10 + (this.day as usize);
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;

        era * 146097 + day_of_era
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Time {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
}

impl Time {
    pub fn total_seconds(&self) -> usize {
        let hs = (self.hour as usize) * 3600;
        let ms = (self.minute as usize) * 60;

        hs + ms + (self.second as usize)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DateTime {
    pub date: Date,
    pub time: Time
}

impl DateTime {
    // XXX(NDRAEY): I may be wrong with that!
    pub fn to_unix(&self) -> u64 {
        fn to_seconds(dt: &DateTime) -> u64 {
            let days = dt.date.total_days() as u64;
            let seconds = dt.time.total_seconds() as u64;

            (days * 24 * 3600) + seconds
        }

        let unix = DateTime {
            date: Date { day: 1, month: 1, year: 1970 },
            time: Time::default()
        };

        to_seconds(self) - to_seconds(&unix)
    }
}