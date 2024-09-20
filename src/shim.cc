#include <iostream>

#include "goog_cc-rs/src/main.rs.h"

namespace goog_cc_rs {

  const uint32_t kInitialBitrateKbps = 60;

  webrtc::NetworkControllerConfig InitialConfig(
      int starting_bandwidth_kbps = kInitialBitrateKbps,
      int min_data_rate_kbps = 0,
      int max_data_rate_kbps = 5 * kInitialBitrateKbps) {
    webrtc::NetworkControllerConfig config(webrtc::CreateEnvironment());
    config.constraints.at_time = webrtc::Timestamp::Zero();
    config.constraints.min_data_rate = webrtc::DataRate::KilobitsPerSec(min_data_rate_kbps);
    config.constraints.max_data_rate = webrtc::DataRate::KilobitsPerSec(max_data_rate_kbps);
    config.constraints.starting_rate = webrtc::DataRate::KilobitsPerSec(starting_bandwidth_kbps);
    return config;
  }

  Timestamp unit_cast(const webrtc::Timestamp& ts) {
    if (ts.IsInfinite()) {
      return Timestamp {
        .us = std::numeric_limits<int64_t>::max(),
      };
    }

    return Timestamp {
      .us = ts.us()
    };
  }

  webrtc::Timestamp unit_cast(const Timestamp& ts)  {
    if (ts.us == std::numeric_limits<int64_t>::max()) {
      return webrtc::Timestamp::PlusInfinity();
    }

    return webrtc::Timestamp::Micros(ts.us);
  }

  TimeDelta unit_cast(const webrtc::TimeDelta& td) {
    if (td.IsInfinite()) {
      return TimeDelta {
        .us = std::numeric_limits<int64_t>::max(),
      };
    }

    return TimeDelta {
      .us = td.us()
    };
  }

  webrtc::TimeDelta unit_cast(const TimeDelta& td) {
    if (td.us == std::numeric_limits<int64_t>::max()) {
      return webrtc::TimeDelta::PlusInfinity();
    }

    return webrtc::TimeDelta::Micros(td.us);
  }

  DataSize unit_cast(const webrtc::DataSize& ds) {
    if (ds.IsInfinite()) {
      return DataSize {
        .bytes = std::numeric_limits<int64_t>::max(),
      };
    }

    return DataSize {
      .bytes = ds.bytes()
    };
  }

  webrtc::DataSize unit_cast(const DataSize& ds) {
    if (ds.bytes == std::numeric_limits<int64_t>::max()) {
      return webrtc::DataSize::Infinity();
    }

    return webrtc::DataSize::Bytes(ds.bytes);
  }

  DataRate unit_cast(const webrtc::DataRate& dr) {
    if (dr.IsInfinite()) {
      return DataRate {
        .bps = std::numeric_limits<int64_t>::max(),
      };
    }

    return DataRate {
      .bps = dr.bps()
    };
  }

  webrtc::DataRate unit_cast(const DataRate& dr) {
    if (dr.bps == std::numeric_limits<int64_t>::max()) {
      return webrtc::DataRate::PlusInfinity();
    }

    return webrtc::DataRate::BitsPerSec(dr.bps);
  }

  template<typename T>
  std::optional<T> create_optional(T optional, bool exists) {
    if (!exists) {
      return std::nullopt;
    }

    return std::optional<T>(optional);
  }

  NetworkControlUpdate from_webrtc_ncu(const webrtc::NetworkControlUpdate& upd) {
    NetworkControlUpdate result{0};

    result.congestion_window_exists = !!upd.congestion_window;
    if (upd.congestion_window) {
      result.congestion_window.bytes = upd.congestion_window->bytes();
    }

    result.pacer_config_exists = !!upd.pacer_config;
    if (upd.pacer_config) {
      const webrtc::PacerConfig& pc = *upd.pacer_config;
      result.pacer_config = PacerConfig {
        .at_time = unit_cast(pc.at_time),
        .data_window = unit_cast(pc.data_window),
        .time_window = unit_cast(pc.time_window),
        .pad_window = unit_cast(pc.pad_window)
      };
    }

    result.target_rate_exists = !!upd.target_rate;
    if (upd.target_rate) {
      const webrtc::TargetTransferRate& ttr = *upd.target_rate;

      result.target_rate = TargetTransferRate{
        .at_time = unit_cast(ttr.at_time),
        .network_estimate = NetworkEstimate {
          .at_time = unit_cast(ttr.network_estimate.at_time),
          .bandwidth = unit_cast(ttr.network_estimate.bandwidth),
          .round_trip_time = unit_cast(ttr.network_estimate.round_trip_time),
          .bwe_period = unit_cast(ttr.network_estimate.bwe_period),
          .loss_rate_ratio = ttr.network_estimate.loss_rate_ratio,
        },
        .target_rate = unit_cast(ttr.target_rate),
        .stable_target_rate = unit_cast(ttr.stable_target_rate),
        .cwnd_reduce_ratio = ttr.cwnd_reduce_ratio,
      };
    }

    for (size_t i = 0; i < upd.probe_cluster_configs.size(); ++i) {
      const webrtc::ProbeClusterConfig& element = upd.probe_cluster_configs[i];
      result.probe_cluster_configs.emplace_back(ProbeClusterConfig{
        .at_time = unit_cast(element.at_time),
        .target_data_rate = unit_cast(element.target_data_rate),
        .target_duration = unit_cast(element.target_duration),
        .min_probe_delta = unit_cast(element.min_probe_delta),
        .target_probe_count = element.target_probe_count,
        .id = element.id,
      });
    }

    return result;
  }

  webrtc::SentPacket from_rs_sent_packet(const SentPacket& msgrs) {
    webrtc::PacedPacketInfo info;
    info.send_bitrate = unit_cast(msgrs.pacing_info.send_bitrate);
    info.probe_cluster_id = msgrs.pacing_info.probe_cluster_id;
    info.probe_cluster_min_probes = msgrs.pacing_info.probe_cluster_min_probes;
    info.probe_cluster_min_bytes = msgrs.pacing_info.probe_cluster_min_bytes;
    info.probe_cluster_bytes_sent = msgrs.pacing_info.probe_cluster_bytes_sent;

    return webrtc::SentPacket{
      .send_time = unit_cast(msgrs.send_time),
      .size = unit_cast(msgrs.size),
      .prior_unacked_data = unit_cast(msgrs.prior_unacked_data),
      .pacing_info = info,
      .audio = msgrs.audio,
      .sequence_number = msgrs.sequence_number,
      .data_in_flight = unit_cast(msgrs.data_in_flight),
    };
  }

  std::unique_ptr<webrtc::GoogCcNetworkController> new_goog_cc() {
    webrtc::NetworkControllerConfig config = InitialConfig();
    webrtc::GoogCcConfig goog_cc_config;

    return std::unique_ptr<webrtc::GoogCcNetworkController>(new webrtc::GoogCcNetworkController(config, std::move(goog_cc_config)));
  }

  NetworkControlUpdate on_network_availability(const std::unique_ptr<webrtc::GoogCcNetworkController>& controller, const NetworkAvailability& msgrs) {
    webrtc::NetworkAvailability msg {
      .at_time = unit_cast(msgrs.at_time),
      .network_available = msgrs.network_available,
    };

    webrtc::NetworkControlUpdate ret = controller->OnNetworkAvailability(msg);
    return from_webrtc_ncu(ret);
  }

  NetworkControlUpdate on_process_interval(const std::unique_ptr<webrtc::GoogCcNetworkController>& controller, const ProcessInterval& msgrs) {
    webrtc::ProcessInterval msg {
      .at_time = unit_cast(msgrs.at_time),
      .pacer_queue = create_optional(unit_cast(msgrs.pacer_queue), msgrs.pacer_queue_exists),
    };

    return from_webrtc_ncu(controller->OnProcessInterval(msg));
  }

  NetworkControlUpdate on_round_trip_time_update(const std::unique_ptr<webrtc::GoogCcNetworkController>& controller, const RoundTripTimeUpdate& msgrs) {
    webrtc::RoundTripTimeUpdate msg {
      .receive_time = unit_cast(msgrs.receive_time),
      .round_trip_time = unit_cast(msgrs.round_trip_time),
      .smoothed = msgrs.smoothed,
    };

    return from_webrtc_ncu(controller->OnRoundTripTimeUpdate(msg));
  }

  NetworkControlUpdate on_received_packet(const std::unique_ptr<webrtc::GoogCcNetworkController>& controller, const ReceivedPacket& msgrs) {
    webrtc::ReceivedPacket msg {
      .send_time = unit_cast(msgrs.send_time),
      .receive_time = unit_cast(msgrs.receive_time),
      .size = unit_cast(msgrs.size),
    };

    return from_webrtc_ncu(controller->OnReceivedPacket(msg));
  }

  NetworkControlUpdate on_sent_packet(const std::unique_ptr<webrtc::GoogCcNetworkController>& controller, const SentPacket& msgrs) {
    return from_webrtc_ncu(controller->OnSentPacket(from_rs_sent_packet(msgrs)));
  }

  NetworkControlUpdate on_transport_loss_report(const std::unique_ptr<webrtc::GoogCcNetworkController>& controller, const TransportLossReport& msgrs) {
    webrtc::TransportLossReport msg {
      .receive_time = unit_cast(msgrs.receive_time),
      .start_time = unit_cast(msgrs.start_time),
      .end_time = unit_cast(msgrs.end_time),
      .packets_lost_delta = msgrs.packets_lost_delta,
      .packets_received_delta = msgrs.packets_received_delta,
    };

    return from_webrtc_ncu(controller->OnTransportLossReport(msg));
  }

  NetworkControlUpdate on_transport_packets_feedback(const std::unique_ptr<webrtc::GoogCcNetworkController>& controller, const TransportPacketsFeedback& msgrs) {
    webrtc::TransportPacketsFeedback msg;

    msg.feedback_time = unit_cast(msgrs.feedback_time);
    msg.data_in_flight = unit_cast(msgrs.data_in_flight);

    for (size_t i = 0; i < msgrs.packet_feedbacks.size(); ++i) {
      webrtc::PacketResult packet_result;

      packet_result.sent_packet = from_rs_sent_packet(msgrs.packet_feedbacks[i].sent_packet);
      packet_result.receive_time = unit_cast(msgrs.packet_feedbacks[i].receive_time);
      msg.packet_feedbacks.emplace_back(std::move(packet_result));
    }

    for (size_t i = 0; i < msgrs.sendless_arrival_times.size(); ++i) {
      msg.sendless_arrival_times.push_back(unit_cast(msgrs.sendless_arrival_times[i]));
    }

    return from_webrtc_ncu(controller->OnTransportPacketsFeedback(msg));
  }
}