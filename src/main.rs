use humantime::format_duration;
use itertools::Itertools;
use hifijson::token::Lex as _;

use std::collections::HashMap;
use std::fs;

macro_rules! json_bench {
    ($count:expr, $($e:tt)*) => {
        {
        let start_time = cpu_time::ProcessTime::now();
        for _ in 0..$count {

             $($e)*;

        }
        format_duration(start_time.elapsed()).to_string()
    }
    };
}

fn main() {
    let mut measurements = HashMap::new();

    let json: String = fs::read_to_string("test.json").unwrap();
    let measure_count = 1000;

    let _: serde_json::Value = serde_json::from_str(json.as_str()).unwrap();

    // For nanoserde you need to define your own object to deserialize.

    // let nanoserde_measurements = json_bench!(
    //     measure_count,
    //     let _: () = DeJson::deserialize_json(json.as_str()).unwrap()
    // );
    // measurements.insert("nanoserde".to_string(), nanoserde_measurements);

    let serde_measurements = json_bench!(
        measure_count,
        let _: serde_json::Value = serde_json::from_str(json.as_str()).unwrap()
    );
    measurements.insert("serde".to_string(), serde_measurements);

    let sonic_measurements = json_bench!(
        measure_count,
        let _: sonic_rs::Value = sonic_rs::from_str(json.as_str()).unwrap()
    );
    measurements.insert("sonic".to_string(), sonic_measurements);

    let json_measurements = json_bench!(
        measure_count,
        let _ = json::parse(json.as_str()).unwrap()
    );
    measurements.insert("json-rs".to_string(), json_measurements);

    let actson_measurements = json_bench!(
        measure_count,
        let _ = actson::serde_json::from_slice(json.as_bytes()).unwrap()
    );
    measurements.insert("actson".to_string(), actson_measurements);

    // let gjson_measurements = json_bench!(
    //     measure_count,
    //     todo!()
    // );
    // measurements.insert("gjson".to_string(), gjson_measurements);

    // let ajson_measurements = json_bench!(
    //     measure_count,
    //     todo!()
    // );
    // measurements.insert("ajson".to_string(), ajson_measurements);

    // let serde_borrow_measurements = json_bench!(
    //     measure_count,
    //     todo!()
    // );
    // measurements.insert("serde_borrow".to_string(), serde_borrow_measurements);

    let jsonic_measurements = json_bench!(
        measure_count,
        let _ = jsonic::parse(json.as_str()).unwrap();
    );
    measurements.insert("jsonic".to_string(), jsonic_measurements);

    let hifijson_measurements = json_bench!(
        measure_count,
        let mut lexer = hifijson::SliceLexer::new(json.as_bytes());
        let _ = lexer.exactly_one(hifijson::value::parse_unbounded).unwrap();
    );
    measurements.insert("hifijson".to_string(), hifijson_measurements);

    let simd_json_measurements = json_bench!(
        measure_count,
        let mut json = json.clone();
        let mut json = unsafe { json.as_bytes_mut() };
        let _ = simd_json::to_owned_value(&mut json).unwrap();
    );
    measurements.insert("simd_json".to_string(), simd_json_measurements);

    let json_deserializer_measurements = json_bench!(
        measure_count,
        let _ = json_deserializer::parse(json.as_bytes()).unwrap();
    );
    measurements.insert("json_deserializer".to_string(), json_deserializer_measurements);

    let justjson_measurements = json_bench!(
        measure_count,
        let _ = justjson::Value::from_json(json.as_str()).unwrap();
    );
    measurements.insert("justjson".to_string(), justjson_measurements);

    let tinyjson_measurements = json_bench!(
        measure_count,
        let _: tinyjson::JsonValue = json.parse().unwrap();
    );
    measurements.insert("tinyjson".to_string(), tinyjson_measurements);

    for name in measurements.keys().sorted() {
        let time = measurements.get(name).unwrap();
        println!("{}: {}", name, time)
    }
}
