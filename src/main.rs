
#![allow(warnings)]

use std::fmt::Debug;

fn main() {
    let keytimes = [(0, 2), (1, 5), (0, 9), (2, 10)];
    slowest_key(&keytimes);
}

fn slowest_key(keytimes: &[(i32, i32)]) -> i32 {
    let mut prev_time = 0;
    let max = keytimes
        .iter()
        .map(|&(key, time)| {
            let diff = time - prev_time;
            prev_time = time;
            diff
        })
        .enumerate()
        .max_by(|e1, e2| e1.1.cmp(&e2.1));
    dbg!(&max);
    0
}

/*
public void recordTweet(String tweetName, long time);

public long[] getTweetCountsPerFrequency(Frequency freq, String tweetName, long startTime, long endTime);

enum Frequency {
	MINUTE,
	HOUR,
	DAY
}
*/

enum EventType {
    TWEET,
    RETWEET,
    LIKE
}

enum Granularity {
	MINUTE,
	HOUR,
	DAY
}

fn record_tweet(event_type: EventType, timestamp: u64) {

}

fn getEventCount(event_type: EventType, start: u64, end: u64, granularity: Granularity) {

}
