CREATE DATABASE IF NOT EXISTS metricbox;

USE metricbox;

CREATE TABLE IF NOT EXISTS events_template (
    id String,
    event String,
    timestamp DateTime64(3),
    user_id Nullable(String),
    payload String,
    received_at DateTime64(3) DEFAULT now()
) ENGINE = MergeTree()
ORDER BY (timestamp, event)
PARTITION BY toYYYYMM(timestamp);