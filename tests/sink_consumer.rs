use async_trait::async_trait;
use logkit::adapters::sinks::{BoundedSink, Sink};
use logkit::{Level, LogEntry, LogError, Logger, LoggerBuilder};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
struct RecordingSink(Arc<Mutex<Vec<String>>>);

#[async_trait]
impl Sink for RecordingSink {
    async fn write(&self, entry: &LogEntry) -> Result<(), LogError> {
        self.0.lock().unwrap().push(entry.message.clone());
        Ok(())
    }
}

#[tokio::test]
async fn public_builder_routes_filtered_entries_through_bounded_sink() {
    let sink = RecordingSink::default();
    let logger = LoggerBuilder::new("consumer")
        .level(Level::Warn)
        .build_with_sink(BoundedSink::new(sink.clone(), 1));
    logger
        .log(LogEntry::new(Level::Info, "filtered"))
        .await
        .unwrap();
    logger
        .log(LogEntry::new(Level::Warn, "retained"))
        .await
        .unwrap();
    assert_eq!(*sink.0.lock().unwrap(), vec!["retained"]);
    assert_eq!(logger.level(), Level::Warn);
}

struct FailedSink;

#[async_trait]
impl Sink for FailedSink {
    async fn write(&self, _: &LogEntry) -> Result<(), LogError> {
        Err(LogError::Io("sink unavailable".into()))
    }
}

#[tokio::test]
async fn public_builder_returns_sink_failure_without_reporting_success() {
    let logger = LoggerBuilder::new("consumer").build_with_sink(FailedSink);
    assert!(matches!(
        logger.log(LogEntry::new(Level::Error, "failure")).await,
        Err(LogError::Io(message)) if message == "sink unavailable"
    ));
    assert!(logger
        .log(LogEntry::new(Level::Debug, "filtered"))
        .await
        .is_ok());
}
