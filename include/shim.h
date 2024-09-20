#pragma once
#include <memory>

#include <api/environment/environment.h>
#include <api/environment/environment_factory.h>
#include <goog_cc/goog_cc_network_control.h>
#include <api/transport/network_control.h>

namespace goog_cc_rs {
    struct NetworkControlUpdate;
    struct NetworkAvailability;
    struct ProcessInterval;
    struct RoundTripTimeUpdate;
    struct ReceivedPacket;
    struct SentPacket;
    struct TransportLossReport;
    struct TransportPacketsFeedback;

    std::unique_ptr<webrtc::GoogCcNetworkController> new_goog_cc();

    NetworkControlUpdate on_network_availability(const std::unique_ptr<webrtc::GoogCcNetworkController> &controller, const NetworkAvailability& msg);
    NetworkControlUpdate on_process_interval(const std::unique_ptr<webrtc::GoogCcNetworkController> &controller, const ProcessInterval& msg);
    NetworkControlUpdate on_round_trip_time_update(const std::unique_ptr<webrtc::GoogCcNetworkController>& controller, const RoundTripTimeUpdate& msg);
    NetworkControlUpdate on_received_packet(const std::unique_ptr<webrtc::GoogCcNetworkController>& controller, const ReceivedPacket& msg);
    NetworkControlUpdate on_sent_packet(const std::unique_ptr<webrtc::GoogCcNetworkController>& controller, const SentPacket& msg);
    NetworkControlUpdate on_transport_loss_report(const std::unique_ptr<webrtc::GoogCcNetworkController>& controller, const TransportLossReport& msg);
    NetworkControlUpdate on_transport_packets_feedback(const std::unique_ptr<webrtc::GoogCcNetworkController>& controller, const TransportPacketsFeedback& msg);
}