#pragma once
#include <memory>

#include <api/environment/environment.h>
#include <api/environment/environment_factory.h>
#include <goog_cc/goog_cc_network_control.h>
#include <api/transport/network_control.h>

namespace goog_cc_rs {
    struct NetworkControlUpdate;

    std::unique_ptr<webrtc::GoogCcNetworkController> new_goog_cc();

    NetworkControlUpdate update_network_availability(const std::unique_ptr<webrtc::GoogCcNetworkController> &controller, webrtc::NetworkAvailability msg);
}