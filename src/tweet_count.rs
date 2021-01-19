use std::ops::Bound::Included;
use std::{collections::BTreeMap, os::macos::raw::stat};

enum EventType {
    TWEET,
    RETWEET,
    LIKE,
}

enum Granularity {
    MINUTE,
    HOUR,
    DAY,
}

#[derive(Debug)]
struct Stats {
    minute: [u64; 24 * 60],
}

impl Stats {
    fn new() -> Stats {
        Stats {
            minute: [0; 24 * 60],
        }
    }
}

#[derive(Debug)]
struct Counter {
    btree: BTreeMap<Timestamp, Stats>, // YYYY_MM_DD_00_00 -> Stats
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Copy, Clone)]
struct Timestamp(usize);

impl Timestamp {
    fn pieces(&self) -> (usize, usize, usize) {
        (
            (self.0 / 10000) * 10000,
            (self.0 % 10000) as usize / 100,
            (self.0 % 100) as usize,
        )
    }
}

impl Counter {
    fn new() -> Counter {
        Counter {
            btree: BTreeMap::new(),
        }
    }

    // timestamp format: YYYY_MM_DD_HH_MM
    fn record_tweet(&mut self, event_type: EventType, timestamp: Timestamp) {
        let (day, hour, minute) = timestamp.pieces();
        let key = Timestamp(day);
        let stats = self.btree.entry(key).or_insert(Stats::new());
        stats.minute[hour * 60 + minute] += 1;
    }

    fn getEventCount(
        &self,
        event_type: EventType,
        start: Timestamp,
        end: Timestamp,
        granularity: Granularity,
    ) {
        let (start_day, start_hour, start_minute) = start.pieces();
        let (end_day, end_hour, end_minute) = end.pieces();
        let (start_day, end_day) = (Timestamp(start_day), Timestamp(end_day));

        let batch_size: usize = match granularity {
            Granularity::DAY => 24 * 60,
            Granularity::HOUR => 60,
            Granularity::MINUTE => 1,
        };

        let mut nread = 0usize; // number actually read so far
        let mut index: usize = (start_hour * 60) + start_minute; // for "day" granularity
        let mut event_count = 0usize; // count of events
        dbg!(index);

        for (day, stats) in self.btree.range((Included(&start_day), Included(&end_day))) {
            println!("--- Read stats for {:?}", &day);
            while index < stats.minute.len() {
                if nread == batch_size {
                    println!("--- Emit batch: {}", event_count);
                    nread = 0;
                    event_count = 0;
                }
                let nleft = (batch_size - nread);
                let nreadable = nleft.min(&stats.minute.len() - index);
                let slice = &stats.minute[index..index + nreadable];
                let count: u64 = slice.iter().sum();
                event_count += count as usize;
                index += nreadable;
                nread += nreadable
            }
            index = 0
        }
        if event_count > 0 {
            println!("Last batch incomplete, not reported: {}", event_count)
        }
    }
}

#[test]
fn test() {
    let mut ctr = Counter::new();

    ctr.record_tweet(EventType::TWEET, Timestamp(2021_01_01_08_30));

    ctr.record_tweet(EventType::TWEET, Timestamp(2021_01_02_06_30));
    ctr.record_tweet(EventType::TWEET, Timestamp(2021_01_02_10_30));

    ctr.record_tweet(EventType::TWEET, Timestamp(2021_01_03_08_30));
    ctr.record_tweet(EventType::TWEET, Timestamp(2021_01_03_09_00));
    ctr.record_tweet(EventType::TWEET, Timestamp(2021_01_03_10_00));

    ctr.getEventCount(
        EventType::TWEET,
        Timestamp(2021_01_01_07_30),
        Timestamp(2021_03_01_10_30),
        Granularity::HOUR,
    )
}
