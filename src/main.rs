use std::{
    i64,
    time::{SystemTime, UNIX_EPOCH},
};

use ffi::{DataSize, PacedPacketInfo};

#[cxx::bridge(namespace = "goog_cc_rs")]
mod ffi {

    #[derive(Debug)]
    struct Timestamp {
        // Number of microseconds since an arbitrary epoch.
        us: i64,
    }

    #[derive(Debug)]
    struct TimeDelta {
        // Microseconds between two timestamps.
        us: i64,
    }

    #[derive(Debug)]
    struct DataSize {
        bytes: i64,
    }

    #[derive(Debug)]
    struct DataRate {
        // Bits per second
        bps: i64,
    }

    #[derive(Debug)]
    struct NetworkControlUpdate {
        congestion_window: DataSize,
        pacer_config: PacerConfig,
        probe_cluster_configs: Vec<ProbeClusterConfig>,
        target_rate: TargetTransferRate,

        congestion_window_exists: bool,
        target_rate_exists: bool,
        pacer_config_exists: bool,
    }

    #[derive(Debug)]
    struct PacerConfig {
        at_time: Timestamp,
        data_window: DataSize,
        time_window: TimeDelta,
        pad_window: DataSize,
    }

    #[derive(Debug)]
    struct ProbeClusterConfig {
        at_time: Timestamp,
        target_data_rate: DataRate,
        target_duration: TimeDelta,
        min_probe_delta: TimeDelta,
        target_probe_count: i32,
        id: i32,
    }

    #[derive(Debug)]
    struct TargetTransferRate {
        at_time: Timestamp,
        network_estimate: NetworkEstimate,
        target_rate: DataRate,
        stable_target_rate: DataRate,
        cwnd_reduce_ratio: f64,
    }

    #[derive(Debug)]
    struct NetworkEstimate {
        at_time: Timestamp,
        bandwidth: DataRate,
        round_trip_time: TimeDelta,
        bwe_period: TimeDelta,
        loss_rate_ratio: f32,
    }

    #[derive(Debug)]
    // These are message inputs
    struct ProcessInterval {
        at_time: Timestamp,

        pacer_queue: DataSize,
        pacer_queue_exists: bool,
    }

    #[derive(Debug)]
    struct RoundTripTimeUpdate {
        receive_time: Timestamp,
        round_trip_time: TimeDelta,
        smoothed: bool,
    }

    #[derive(Debug)]
    struct ReceivedPacket {
        send_time: Timestamp,
        receive_time: Timestamp,
        size: DataSize,
    }

    #[derive(Debug)]
    struct SentPacket {
        send_time: Timestamp,
        size: DataSize,
        prior_unacked_data: DataSize,
        pacing_info: PacedPacketInfo,
        audio: bool,
        sequence_number: i64,
        data_in_flight: DataSize,
    }

    #[derive(Debug)]
    struct PacedPacketInfo {
        send_bitrate: DataRate,
        probe_cluster_id: i32,
        probe_cluster_min_probes: i32,
        probe_cluster_min_bytes: i32,
        probe_cluster_bytes_sent: i32,
    }

    #[derive(Debug)]
    struct TransportLossReport {
        receive_time: Timestamp,
        start_time: Timestamp,
        end_time: Timestamp,
        packets_lost_delta: u64,
        packets_received_delta: u64,
    }

    #[derive(Debug)]
    struct PacketResult {
        sent_packet: SentPacket,
        receive_time: Timestamp,
    }

    #[derive(Debug)]
    struct TransportPacketsFeedback {
        feedback_time: Timestamp,
        data_in_flight: DataSize,
        packet_feedbacks: Vec<PacketResult>,
        sendless_arrival_times: Vec<Timestamp>,
    }

    #[derive(Debug)]
    struct NetworkAvailability {
        at_time: Timestamp,
        network_available: bool,
    }

    #[namespace = "webrtc"]
    unsafe extern "C++" {
        include!("goog_cc-rs/include/webrtc_types.h");
        type GoogCcNetworkController;
    }

    unsafe extern "C++" {
        include!("goog_cc-rs/include/shim.h");

        fn new_goog_cc() -> UniquePtr<GoogCcNetworkController>;

        fn on_network_availability(
            controller: &UniquePtr<GoogCcNetworkController>,
            msg: &NetworkAvailability,
        ) -> NetworkControlUpdate;

        fn on_process_interval(
            controller: &UniquePtr<GoogCcNetworkController>,
            msg: &ProcessInterval,
        ) -> NetworkControlUpdate;

        fn on_round_trip_time_update(
            controller: &UniquePtr<GoogCcNetworkController>,
            msg: &RoundTripTimeUpdate,
        ) -> NetworkControlUpdate;

        fn on_received_packet(
            controller: &UniquePtr<GoogCcNetworkController>,
            msg: &ReceivedPacket,
        ) -> NetworkControlUpdate;

        fn on_sent_packet(
            controller: &UniquePtr<GoogCcNetworkController>,
            msg: &SentPacket,
        ) -> NetworkControlUpdate;

        fn on_transport_loss_report(
            controller: &UniquePtr<GoogCcNetworkController>,
            msg: &TransportLossReport,
        ) -> NetworkControlUpdate;

        fn on_transport_packets_feedback(
            controller: &UniquePtr<GoogCcNetworkController>,
            msg: &TransportPacketsFeedback,
        ) -> NetworkControlUpdate;
    }
}

impl ffi::Timestamp {
    fn now() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros();

        ffi::Timestamp { us: now as i64 }
    }

    fn micros(us: i64) -> Self {
        ffi::Timestamp { us }
    }

    fn positive_infinity() -> Self {
        ffi::Timestamp { us: i64::MAX }
    }
}

impl Default for ffi::PacedPacketInfo {
    fn default() -> Self {
        ffi::PacedPacketInfo {
            send_bitrate: ffi::DataRate { bps: 0 },
            probe_cluster_id: -1,
            probe_cluster_min_probes: -1,
            probe_cluster_min_bytes: -1,
            probe_cluster_bytes_sent: -1,
        }
    }
}

fn main() {
    let goog_cc: cxx::UniquePtr<ffi::GoogCcNetworkController> = ffi::new_goog_cc();

    let update = ffi::on_network_availability(
        &goog_cc,
        &ffi::NetworkAvailability {
            at_time: ffi::Timestamp::now(),
            network_available: true,
        },
    );
    println!("{:?}", update);

    let update = ffi::on_process_interval(
        &goog_cc,
        &&ffi::ProcessInterval {
            at_time: ffi::Timestamp::now(),
            pacer_queue: ffi::DataSize { bytes: 1024 * 1024 },
            pacer_queue_exists: true,
        },
    );
    println!("{:?}", update);

    let update = ffi::on_process_interval(
        &goog_cc,
        &&ffi::ProcessInterval {
            at_time: ffi::Timestamp::now(),
            pacer_queue: ffi::DataSize { bytes: 0 },
            pacer_queue_exists: false,
        },
    );
    println!("{:?}", update);

    let update = ffi::on_round_trip_time_update(
        &goog_cc,
        &ffi::RoundTripTimeUpdate {
            receive_time: ffi::Timestamp::now(),
            round_trip_time: ffi::TimeDelta { us: 1000 * 140 },
            smoothed: false,
        },
    );
    println!("{:?}", update);

    let update = ffi::on_received_packet(
        &goog_cc,
        &ffi::ReceivedPacket {
            receive_time: ffi::Timestamp::now(),
            send_time: ffi::Timestamp {
                us: ffi::Timestamp::now().us - 1000 * 20,
            },
            size: ffi::DataSize { bytes: 1024 },
        },
    );
    println!("{:?}", update);

    let update = ffi::on_sent_packet(
        &goog_cc,
        &ffi::SentPacket {
            send_time: ffi::Timestamp {
                us: ffi::Timestamp::now().us - 1000 * 20,
            },
            size: ffi::DataSize { bytes: 1024 },
            prior_unacked_data: ffi::DataSize { bytes: 0 },
            pacing_info: PacedPacketInfo::default(),
            audio: true,
            sequence_number: 42,
            data_in_flight: DataSize { bytes: 1024 },
        },
    );
    println!("{:?}", update);

    let update = ffi::on_transport_loss_report(
        &goog_cc,
        &ffi::TransportLossReport {
            receive_time: ffi::Timestamp::now(),
            start_time: ffi::Timestamp {
                us: ffi::Timestamp::now().us - 1000 * 20,
            },
            end_time: ffi::Timestamp::now(),
            packets_lost_delta: 4,
            packets_received_delta: 500,
        },
    );
    println!("{:?}", update);

    let update = ffi::on_transport_packets_feedback(
        &goog_cc,
        &ffi::TransportPacketsFeedback {
            feedback_time: ffi::Timestamp::now(),
            data_in_flight: ffi::DataSize { bytes: 1024 },
            packet_feedbacks: vec![ffi::PacketResult {
                sent_packet: ffi::SentPacket {
                    send_time: ffi::Timestamp {
                        us: ffi::Timestamp::now().us - 1000 * 20,
                    },
                    size: ffi::DataSize { bytes: 1024 },
                    prior_unacked_data: ffi::DataSize { bytes: 0 },
                    pacing_info: PacedPacketInfo::default(),
                    audio: true,
                    sequence_number: 42,
                    data_in_flight: DataSize { bytes: 1024 },
                },
                receive_time: ffi::Timestamp::now(),
            }],
            sendless_arrival_times: vec![],
        },
    );
    println!("{:?}", update);
}
