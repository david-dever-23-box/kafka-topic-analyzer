extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rdkafka;
extern crate uuid;
extern crate chrono;
#[macro_use] extern crate prettytable;
extern crate indicatif;
extern crate rocksdb;

use prettytable::Table;
use std::time::Instant;
use prettytable::row::Row;
use prettytable::cell::Cell;
use clap::{App, Arg};

mod kafka;
mod metric;

fn main() {
    env_logger::init();

    let matches = App::new("Kafka Topic Analyzer")
        .bin_name("kafka-topic-analyzer")

        .arg(Arg::with_name("topic")
            .short("t")
            .long("topic")
            .value_name("TOPIC")
            .help("The topic to analyze")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("bootstrap-server")
            .short("b")
            .long("bootstrap-server")
            .value_name("BOOTSTRAP_SERVER")
            .help("Bootstrap server(s) to work with, comma separated")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("count-alive-keys")
            .short("c")
            .long("count-alive-keys")
            .value_name("LOCAL_ALIVE_KEYS_STORAGE_PATH")
            .help("Counts the effective number of alive keys in a log compacted topic by saving the \
            state for each key in a local file and counting the result at the end of the read operation.\
            THIS MUST BE A SEPARATE DIRECTORY, DON'T USE '.'!")
            .takes_value(true)
            .required(false))
        .get_matches();

    let start_time = Instant::now();

    let mut partitions = Vec::<i32>::new();
    let topic = matches.value_of("topic").unwrap();
    let bootstrap_server = matches.value_of("bootstrap-server").unwrap();
    let mut topic_analyzer = kafka::TopicAnalyzer::new_from_bootstrap_servers(bootstrap_server);
    let (start_offsets, end_offsets) = topic_analyzer.get_topic_offsets(topic);

    for v in start_offsets.keys() {
        partitions.push(*v);
    }

    partitions.sort();

    topic_analyzer.read_topic_into_metrics(topic, &end_offsets);


    let duration_secs = start_time.elapsed().as_secs();

    println!();
    println!("{}", "=".repeat(120));
    println!("Calculating statistics...");
    println!("Topic {}", topic);
    println!("Scanning took: {} seconds", duration_secs);
    println!("Estimated Msg/s: {}", (topic_analyzer.metrics().overall_count() / duration_secs));
    println!("{}", "-".repeat(120));
    println!("Earliest Message: {}", topic_analyzer.metrics().earliest_message());
    println!("Latest Message: {}", topic_analyzer.metrics().latest_message());
    println!("{}", "-".repeat(120));
    println!("Largest Message: {} bytes", topic_analyzer.metrics().largest_message());
    println!("Smallest Message: {} bytes", topic_analyzer.metrics().smallest_message());
    println!("Topic Size: {} bytes", topic_analyzer.metrics().overall_size());
//    match opt_log_compact_tracking {
//        Some(lcm) => {
//            println!("{}", "-".repeat(120));
//            println!("Alive keys: {}", lcm.sum_all_alive());
//            println!("{}", "-".repeat(120));
//            fs::remove_dir_all(matches.value_of("count-alive-keys").unwrap()).unwrap();
//        },
//        None => {},
//    }
    println!("{}", "=".repeat(120));

    let mut table = Table::new();
    table.add_row(row!["P", "|< OS", ">| OS", "Total", "Alive", "Tmb", "DR", "K Null", "K !Null", "P-Bytes", "K-Bytes", "V-Bytes", "A K-Sz", "A V-Sz", "A M-Sz"]);

    for partition in partitions {
        let key_size_avg = topic_analyzer.metrics().key_size_avg(partition);
        table.add_row(Row::new(vec![
            Cell::new(format!("{}", partition).as_str()), // P
            Cell::new(format!("{}", &start_offsets[&partition]).as_str()), // |< OS
            Cell::new(format!("{}", &end_offsets[&partition]).as_str()), // OS >|
            Cell::new(format!("{}", topic_analyzer.metrics().total(partition)).as_str()), // Total
            Cell::new(format!("{}", topic_analyzer.metrics().alive(partition)).as_str()), // Alive
            Cell::new(format!("{}", topic_analyzer.metrics().tombstones(partition)).as_str()), // TB
            Cell::new(format!("{0:.4}", topic_analyzer.metrics().dirty_ratio(partition)).as_str()), // DR
            Cell::new(format!("{}", topic_analyzer.metrics().key_null(partition)).as_str()), // K Null
            Cell::new(format!("{}", topic_analyzer.metrics().key_non_null(partition)).as_str()), // K !Null
            Cell::new(format!("{}", topic_analyzer.metrics().key_size_sum(partition) + topic_analyzer.metrics().value_size_sum(partition)).as_str()), // P-Bytes
            Cell::new(format!("{}", topic_analyzer.metrics().key_size_sum(partition)).as_str()), // K-Bytes
            Cell::new(format!("{}", topic_analyzer.metrics().value_size_sum(partition)).as_str()), // V-Bytes
            Cell::new(format!("{}", key_size_avg).as_str()), // A-Key-Size
            Cell::new(format!("{}", topic_analyzer.metrics().value_size_avg(partition)).as_str()), // A-V-Size
            Cell::new(format!("{}", topic_analyzer.metrics().message_size_avg(partition)).as_str()), // A-M-Size
        ]));
    }

    println!("| K = Key, V = Value, P = Partition, Tmb = Tombstone(s), Sz = Size");
    println!("| DR = Dirty Ratio, A = Average, Lst = last, |< OS = start offset, >| OS = end offset");
    table.printstd();
    println!();
    println!("{}", "=".repeat(120));
}
