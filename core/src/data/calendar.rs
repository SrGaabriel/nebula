use chrono::{DateTime, Utc, Datelike, Weekday, Duration, NaiveDate};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::data::LOCAL_EPOCH;

const FREQ_BITS: u64 = 2;
const FREQ_SHIFT: u64 = 0;
const INTERVAL_BITS: u64 = 16;
const INTERVAL_SHIFT: u64 = 2;
const END_TYPE_BITS: u64 = 2;
const END_TYPE_SHIFT: u64 = 18;
const END_VALUE_BITS: u64 = 12;
const END_VALUE_SHIFT: u64 = 20;
const WEEKLY_BITS: u64 = 7;
const WEEKLY_SHIFT: u64 = 32;
const MONTHLY_DAY_BITS: u64 = 6;
const MONTHLY_DAY_SHIFT: u64 = 39;
const MONTHLY_WEEKDAY_BITS: u64 = 3;
const MONTHLY_WEEKDAY_SHIFT: u64 = 45;
const MONTHLY_OCCURRENCE_BITS: u64 = 4;
const MONTHLY_OCCURRENCE_SHIFT: u64 = 48;

const MAX_INTERVAL: u32 = 0xFFFF;
const MAX_COUNT: u32 = 0xFFF;
const MAX_MONTHLY_DAY: u32 = 31;
const OCCURRENCE_OFFSET: i8 = 8;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecurrenceEnd {
    Never,
    Count(u32),
    Until(DateTime<Utc>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeeklyPattern {
    pub days: HashSet<Weekday>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MonthlyPattern {
    DayOfMonth(u32),
    WeekdayOccurrence {
        weekday: Weekday,
        occurrence: i8,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecurrenceRule {
    pub frequency: Frequency,
    pub interval: u32,
    pub end: RecurrenceEnd,
    pub weekly_pattern: Option<WeeklyPattern>,
    pub monthly_pattern: Option<MonthlyPattern>,
}

impl RecurrenceRule {
    pub fn daily(interval: u32) -> Self {
        Self {
            frequency: Frequency::Daily,
            interval,
            end: RecurrenceEnd::Never,
            weekly_pattern: None,
            monthly_pattern: None,
        }
    }

    pub fn weekly(interval: u32, days: Vec<Weekday>) -> Self {
        let days_set = days.into_iter().collect();
        Self {
            frequency: Frequency::Weekly,
            interval,
            end: RecurrenceEnd::Never,
            weekly_pattern: Some(WeeklyPattern { days: days_set }),
            monthly_pattern: None,
        }
    }

    pub fn monthly_day(interval: u32, day: u32) -> Self {
        Self {
            frequency: Frequency::Monthly,
            interval,
            end: RecurrenceEnd::Never,
            weekly_pattern: None,
            monthly_pattern: Some(MonthlyPattern::DayOfMonth(day)),
        }
    }

    pub fn monthly_weekday(interval: u32, weekday: Weekday, occurrence: i8) -> Self {
        Self {
            frequency: Frequency::Monthly,
            interval,
            end: RecurrenceEnd::Never,
            weekly_pattern: None,
            monthly_pattern: Some(MonthlyPattern::WeekdayOccurrence { weekday, occurrence }),
        }
    }

    pub fn yearly(interval: u32) -> Self {
        Self {
            frequency: Frequency::Yearly,
            interval,
            end: RecurrenceEnd::Never,
            weekly_pattern: None,
            monthly_pattern: None,
        }
    }

    pub fn with_count(mut self, count: u32) -> Self {
        self.end = RecurrenceEnd::Count(count);
        self
    }

    pub fn with_end_date(mut self, end_date: DateTime<Utc>) -> Self {
        self.end = RecurrenceEnd::Until(end_date);
        self
    }

    pub fn generate_occurrences(
        &self,
        start_time: DateTime<Utc>,
        range_start: DateTime<Utc>,
        range_end: DateTime<Utc>,
        max_occurrences: Option<usize>,
    ) -> Vec<DateTime<Utc>> {
        assert!(self.interval > 0, "Interval must be greater than 0");
        let mut occurrences = Vec::new();
        let mut current = start_time;
        let mut count = 0;
        let max = max_occurrences.unwrap_or(1000);

        if matches!(self.frequency, Frequency::Weekly) && self.weekly_pattern.is_some() {
            let pattern = self.weekly_pattern.as_ref().unwrap();
            let mut week_start = start_time;

            let days_since_monday = start_time.weekday().num_days_from_monday();
            week_start = week_start - Duration::days(days_since_monday as i64);

            let mut week_count = 0;

            while week_start <= range_end && occurrences.len() < max {
                if week_count % self.interval == 0 {
                    for day_offset in 0..7 {
                        let day_date = week_start + Duration::days(day_offset);

                        if day_date < start_time {
                            continue;
                        }

                        match &self.end {
                            RecurrenceEnd::Count(max_count) => {
                                if count >= *max_count {
                                    return occurrences;
                                }
                            }
                            RecurrenceEnd::Until(until_date) => {
                                if day_date > *until_date {
                                    return occurrences;
                                }
                            }
                            RecurrenceEnd::Never => {}
                        }

                        if day_date >= range_start && day_date <= range_end {
                            if pattern.days.contains(&day_date.weekday()) {
                                occurrences.push(day_date);
                                count += 1;

                                if occurrences.len() >= max {
                                    return occurrences;
                                }
                            }
                        }

                        if count > 10000 {
                            return occurrences;
                        }
                    }
                }

                week_start = week_start + Duration::weeks(1);
                week_count += 1;
            }

            return occurrences;
        }

        // Original logic for non-weekly patterns
        while current <= range_end && occurrences.len() < max {
            match &self.end {
                RecurrenceEnd::Count(max_count) => {
                    if count >= *max_count {
                        break;
                    }
                }
                RecurrenceEnd::Until(until_date) => {
                    if current > *until_date {
                        break;
                    }
                }
                RecurrenceEnd::Never => {}
            }

            if current >= range_start && current <= range_end {
                if self.matches_pattern(current) {
                    occurrences.push(current);
                }
            }

            current = self.next_occurrence(current);
            count += 1;

            if count > 10000 {
                break;
            }
        }

        occurrences
    }

    pub fn next_occurrence_after(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>> {
        let far_future = after + Duration::days(365 * 10);
        let occurrences = self.generate_occurrences(after, after, far_future, Some(1));
        occurrences.into_iter().next()
    }

    pub fn is_occurrence_on(&self, date: DateTime<Utc>, start_time: DateTime<Utc>) -> bool {
        if date < start_time {
            return false;
        }

        match &self.end {
            RecurrenceEnd::Until(until_date) => {
                if date > *until_date {
                    return false;
                }
            }
            _ => {}
        }

        let days_diff = (date.date_naive() - start_time.date_naive()).num_days();

        match self.frequency {
            Frequency::Daily => {
                days_diff >= 0 && days_diff as u32 % self.interval == 0
            }
            Frequency::Weekly => {
                let weeks_diff = days_diff / 7;
                weeks_diff >= 0 && weeks_diff as u32 % self.interval == 0 && self.matches_pattern(date)
            }
            _ => {
                let mut current = start_time;
                let mut iterations = 0;

                while current.date_naive() <= date.date_naive() && iterations < 10000 {
                    if current.date_naive() == date.date_naive() && self.matches_pattern(current) {
                        return true;
                    }
                    current = self.next_occurrence(current);
                    iterations += 1;
                }
                false
            }
        }
    }

    fn matches_pattern(&self, date: DateTime<Utc>) -> bool {
        match &self.frequency {
            Frequency::Weekly => {
                if let Some(pattern) = &self.weekly_pattern {
                    pattern.days.contains(&date.weekday())
                } else {
                    true
                }
            }
            Frequency::Monthly => {
                if let Some(pattern) = &self.monthly_pattern {
                    match pattern {
                        MonthlyPattern::DayOfMonth(day) => date.day() == *day,
                        MonthlyPattern::WeekdayOccurrence { weekday, occurrence } => {
                            self.matches_weekday_occurrence(date, *weekday, *occurrence)
                        }
                    }
                } else {
                    true
                }
            }
            _ => true,
        }
    }

    fn matches_weekday_occurrence(&self, date: DateTime<Utc>, weekday: Weekday, occurrence: i8) -> bool {
        if date.weekday() != weekday {
            return false;
        }

        let days_in_month = self.days_in_month(date.year(), date.month());
        let first_day = date.with_day(1).unwrap();
        let first_weekday = first_day.weekday();

        if occurrence > 0 {
            let days_from_first = (weekday.num_days_from_monday() + 7 - first_weekday.num_days_from_monday()) % 7;
            let target_day = days_from_first + 1 + (occurrence as u32 - 1) * 7;
            target_day == date.day() && target_day <= days_in_month
        } else {
            let last_day = NaiveDate::from_ymd_opt(date.year(), date.month(), days_in_month).unwrap();
            let last_weekday = last_day.weekday();
            let days_from_last = (last_weekday.num_days_from_monday() + 7 - weekday.num_days_from_monday()) % 7;
            let target_day = days_in_month - days_from_last - ((-occurrence as u32 - 1) * 7);
            target_day == date.day() && target_day >= 1
        }
    }

    fn next_occurrence(&self, current: DateTime<Utc>) -> DateTime<Utc> {
        match self.frequency {
            Frequency::Daily => current + Duration::days(self.interval as i64),
            Frequency::Weekly => current + Duration::weeks(self.interval as i64),
            Frequency::Monthly => self.add_months(current, self.interval),
            Frequency::Yearly => self.add_years(current, self.interval),
        }
    }

    fn add_months(&self, date: DateTime<Utc>, months: u32) -> DateTime<Utc> {
        let mut year = date.year();
        let mut month = date.month() + months;

        while month > 12 {
            year += 1;
            month -= 12;
        }

        let day = std::cmp::min(date.day(), self.days_in_month(year, month));
        date.with_year(year)
            .and_then(|d| d.with_month(month))
            .and_then(|d| d.with_day(day))
            .unwrap_or(date)
    }

    fn add_years(&self, date: DateTime<Utc>, years: u32) -> DateTime<Utc> {
        let new_year = date.year() + years as i32;
        date.with_year(new_year).unwrap_or(date)
    }

    fn days_in_month(&self, year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if self.is_leap_year(year) { 29 } else { 28 },
            _ => 30,
        }
    }

    fn is_leap_year(&self, year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    pub fn to_u64(&self) -> Result<u64, &'static str> {
        let mut packed = 0u64;

        let freq_bits = match self.frequency {
            Frequency::Daily => 0u64,
            Frequency::Weekly => 1u64,
            Frequency::Monthly => 2u64,
            Frequency::Yearly => 3u64,
        };
        packed |= freq_bits << FREQ_SHIFT;

        if self.interval > MAX_INTERVAL {
            return Err("Interval exceeds maximum allowed value");
        }
        packed |= (self.interval as u64) << INTERVAL_SHIFT;

        match &self.end {
            RecurrenceEnd::Never => {
                packed |= 0u64 << END_TYPE_SHIFT;
            }
            RecurrenceEnd::Count(count) => {
                if *count > MAX_COUNT {
                    return Err("Count exceeds maximum allowed value");
                }
                packed |= 1u64 << END_TYPE_SHIFT;
                packed |= (*count as u64) << END_VALUE_SHIFT;
            }
            RecurrenceEnd::Until(datetime) => {
                let base_date = DateTime::from_timestamp(LOCAL_EPOCH as i64, 0).unwrap();
                let days_diff = (datetime.timestamp() - base_date.timestamp()) / (24 * 60 * 60);

                if days_diff < 0 || days_diff > ((1i64 << END_VALUE_BITS) - 1) {
                    return Err("End date is out of range (must be between 2020 and ~2031)");
                }
                packed |= 2u64 << END_TYPE_SHIFT;
                packed |= (days_diff as u64) << END_VALUE_SHIFT;
            }
        }

        if let Some(pattern) = &self.weekly_pattern {
            for day in &pattern.days {
                let bit_pos = WEEKLY_SHIFT + day.num_days_from_monday() as u64;
                packed |= 1u64 << bit_pos;
            }
        }

        if let Some(pattern) = &self.monthly_pattern {
            match pattern {
                MonthlyPattern::DayOfMonth(day) => {
                    if *day > MAX_MONTHLY_DAY {
                        return Err("Day of month exceeds maximum allowed value");
                    }
                    packed |= (*day as u64) << MONTHLY_DAY_SHIFT;
                }
                MonthlyPattern::WeekdayOccurrence { weekday, occurrence } => {
                    if *occurrence < -7 || *occurrence > 7 || *occurrence == 0 {
                        return Err("Occurrence must be between -7 and 7, excluding 0");
                    }

                    packed |= (weekday.num_days_from_monday() as u64) << MONTHLY_WEEKDAY_SHIFT;
                    let occurrence_bits = (*occurrence + OCCURRENCE_OFFSET) as u64;
                    packed |= occurrence_bits << MONTHLY_OCCURRENCE_SHIFT;
                }
            }
        }

        Ok(packed)
    }

    pub fn from_u64(packed: u64) -> Result<Self, &'static str> {
        let frequency = match (packed >> FREQ_SHIFT) & ((1u64 << FREQ_BITS) - 1) {
            0 => Frequency::Daily,
            1 => Frequency::Weekly,
            2 => Frequency::Monthly,
            3 => Frequency::Yearly,
            _ => unreachable!(),
        };

        let interval = ((packed >> INTERVAL_SHIFT) & ((1u64 << INTERVAL_BITS) - 1)) as u32;
        if interval == 0 {
            return Err("Invalid interval: cannot be zero");
        }

        let end_type = (packed >> END_TYPE_SHIFT) & ((1u64 << END_TYPE_BITS) - 1);
        let end_value = (packed >> END_VALUE_SHIFT) & ((1u64 << END_VALUE_BITS) - 1);
        let end = match end_type {
            0 => RecurrenceEnd::Never,
            1 => RecurrenceEnd::Count(end_value as u32),
            2 => {
                let base_date = DateTime::from_timestamp(1577836800, 0).unwrap();
                let days_diff = end_value as i64;
                let timestamp = base_date.timestamp() + (days_diff * 24 * 60 * 60);
                let datetime = DateTime::from_timestamp(timestamp, 0)
                    .ok_or("Invalid timestamp in packed data")?;
                RecurrenceEnd::Until(datetime)
            }
            _ => return Err("Invalid end type"),
        };

        let weekly_bits = (packed >> WEEKLY_SHIFT) & ((1u64 << WEEKLY_BITS) - 1);
        let weekly_pattern = if weekly_bits != 0 {
            let mut days = HashSet::new();
            for i in 0..7 {
                if (weekly_bits >> i) & 1 == 1 {
                    let weekday = weekday_from_u8(i);
                    days.insert(weekday);
                }
            }
            Some(WeeklyPattern { days })
        } else {
            None
        };

        let monthly_day = (packed >> MONTHLY_DAY_SHIFT) & ((1u64 << MONTHLY_DAY_BITS) - 1);
        let monthly_weekday = (packed >> MONTHLY_WEEKDAY_SHIFT) & ((1u64 << MONTHLY_WEEKDAY_BITS) - 1);
        let monthly_occurrence = (packed >> MONTHLY_OCCURRENCE_SHIFT) & ((1u64 << MONTHLY_OCCURRENCE_BITS) - 1);

        let monthly_pattern = if monthly_day != 0 {
            Some(MonthlyPattern::DayOfMonth(monthly_day as u32))
        } else if monthly_weekday != 7 && monthly_occurrence != 0 {
            let weekday = weekday_from_u8(monthly_weekday as u8);
            let occurrence = (monthly_occurrence as i8) - OCCURRENCE_OFFSET;
            if occurrence == 0 {
                return Err("Invalid occurrence: cannot be zero");
            }
            Some(MonthlyPattern::WeekdayOccurrence { weekday, occurrence })
        } else {
            None
        };

        Ok(RecurrenceRule {
            frequency,
            interval,
            end,
            weekly_pattern,
            monthly_pattern,
        })
    }
}

#[inline]
fn weekday_from_u8(n: u8) -> Weekday {
    match n {
        0 => Weekday::Mon,
        1 => Weekday::Tue,
        2 => Weekday::Wed,
        3 => Weekday::Thu,
        4 => Weekday::Fri,
        5 => Weekday::Sat,
        6 => Weekday::Sun,
        _ => panic!("Invalid weekday number: {}", n),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurrence_rule_serialization() {
        let rule = RecurrenceRule::weekly(2, vec![Weekday::Mon, Weekday::Wed])
            .with_count(10);
        let packed = rule.to_u64().unwrap();
        let unpacked = RecurrenceRule::from_u64(packed).unwrap();
        println!("Original: {:?}", rule);
        println!("Packed: {:064b}", packed);
        println!("Unpacked: {:?}", unpacked);
        assert_eq!(rule, unpacked);
    }

    #[test]
    fn extremely_complex_recurrence_rule() {
        let rule = RecurrenceRule {
            frequency: Frequency::Monthly,
            interval: 3,
            end: RecurrenceEnd::Until(DateTime::parse_from_rfc3339("2030-12-31T00:00:00Z").unwrap().with_timezone(&Utc)),
            weekly_pattern: None,
            monthly_pattern: Some(MonthlyPattern::WeekdayOccurrence {
                weekday: Weekday::Fri,
                occurrence: -1,
            }),
        };
        let packed = rule.to_u64().unwrap();
        let unpacked = RecurrenceRule::from_u64(packed).unwrap();
        println!("Original: {:?}", rule);
        println!("Packed: {:064b}", packed);
        println!("Unpacked: {:?}", unpacked);
        assert_eq!(rule, unpacked);
    }

    #[test]
    fn generate_occurrences() {
        let rule = RecurrenceRule::weekly(1, vec![Weekday::Mon, Weekday::Wed])
            .with_count(5);
        let start_time = DateTime::parse_from_rfc3339("2023-01-02T10:00:00Z").unwrap().with_timezone(&Utc);
        let range_start = DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap().with_timezone(&Utc);
        let range_end = DateTime::parse_from_rfc3339("2023-01-31T23:59:59Z").unwrap().with_timezone(&Utc);
        let occurrences = rule.generate_occurrences(start_time, range_start, range_end, None);
        for occ in &occurrences {
            println!("Occurrence: {}", occ);
        }
        assert_eq!(occurrences.len(), 5);
    }

}