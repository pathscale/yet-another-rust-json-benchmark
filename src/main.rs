use std::collections::HashMap;
use std::fs;
use std::time::{Duration, SystemTime};

use humantime::format_duration;

fn low_high_mid(vec: Vec<u128>) -> (String, String, String) {
    let mut min = vec[0];
    let mut max = 0;
    let mut mid = 0;

    let len = vec.len() as u128;

    for v in vec {
        if v > max {
            max = v;
        }

        if v < min {
            min = v;
        }

        mid += v;
    }

    mid = mid / len;

    let min_duration = Duration::from_nanos(min as u64);
    let max_duration = Duration::from_nanos(max as u64);
    let mid_duration = Duration::from_nanos(mid as u64);

    (
        format_duration(min_duration).to_string(),
        format_duration(max_duration).to_string(),
        format_duration(mid_duration).to_string(),
    )
}

macro_rules! json_bench {
    ($count:expr, $($e:tt)*) => {
        {
        let mut vec = Vec::new();

        for _ in 0..$count {
            let start_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap().as_nanos();

             $($e)*;

            let end_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap().as_nanos();

            vec.push(end_time - start_time);
        }

        low_high_mid(vec)
    }
    };
}

fn main() {
    let mut measurements = HashMap::new();

    let json: String = fs::read_to_string("big.json").unwrap();
    let measure_count = 1000;

    let serde_measurements = json_bench!(
        measure_count,
        let _: serde_json::Value = serde_json::from_str(json.as_str()).unwrap()
    );
    measurements.insert("serde".to_string(), serde_measurements);

    // For nanoserde you need to define your own object to deserialize.

    // let nanoserde_measurements = json_bench!(
    //     measure_count,
    //     let _: () = DeJson::deserialize_json(json.as_str()).unwrap()
    // );
    // measurements.insert("nanoserde".to_string(), nanoserde_measurements);

    let sonic_measurements = json_bench!(
        measure_count,
        let _: sonic_rs::Value = sonic_rs::from_str(json.as_str()).unwrap()
    );
    measurements.insert("sonic".to_string(), sonic_measurements);

    let json_measurements = json_bench!(
        measure_count,
        let _ = json::parse(json.as_str()).unwrap()
    );
    measurements.insert("json".to_string(), json_measurements);

    for (name, (min, max, mid)) in measurements {
        println!("{}: min: {}, max: {}, mid: {}", name, min, max, mid)
    }
}
