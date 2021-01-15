use std::collections::BTreeMap;
use std::ops::Bound::Included;

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
    day: u64,
    hour: [u64; 24],
    minute: [[u64; 60]; 24],
}

impl Stats {
    fn new() -> Stats {
        Stats {
            day: 0,
            hour: [0; 24],
            minute: [[0; 60]; 24],
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
    fn get_yyyyddmm(&self) -> Timestamp {
        Timestamp((self.0 / 10000) * 10000)
    }

    fn get_hh(&self) -> usize {
        (self.0 % 10000) as usize / 100
    }

    fn get_mm(&self) -> usize {
        (self.0 % 100) as usize
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
        let key = timestamp.get_yyyyddmm();
        let stats = self.btree.entry(key).or_insert(Stats::new());
        let hour = timestamp.get_hh();
        let minute = timestamp.get_mm();
        stats.day += 1;
        stats.hour[hour] += 1;
        stats.minute[hour][minute] += 1;
    }

    fn getEventCount(
        &self,
        event_type: EventType,
        start: Timestamp,
        end: Timestamp,
        granularity: Granularity,
    ) {
        let (start_day, end_day) = (start.get_yyyyddmm(), end.get_yyyyddmm());
        for (key, value) in self.btree.range((Included(&start), Included(&end))) {
            println!("{:?}: {:?}", &key, &value.day);
        }
    }
}

#[test]
fn test() {
    let mut ctr = Counter::new();

    ctr.record_tweet(EventType::TWEET, Timestamp(2021_01_01_06_01));
    ctr.record_tweet(EventType::TWEET, Timestamp(2021_01_02_06_02));
    ctr.record_tweet(EventType::TWEET, Timestamp(2021_01_02_06_01));

    ctr.record_tweet(EventType::TWEET, Timestamp(2021_01_03_06_01));
    ctr.record_tweet(EventType::TWEET, Timestamp(2021_01_03_09_01));
    ctr.record_tweet(EventType::TWEET, Timestamp(2021_01_03_09_21));


    ctr.getEventCount(
        EventType::TWEET,
        Timestamp(2021_01_01_00_00),
        Timestamp(2021_03_01_00_00),
        Granularity::MINUTE,
    )
}
