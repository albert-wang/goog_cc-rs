#pragma once
#include <memory>

#include <api/environment/environment.h>
#include <api/environment/environment_factory.h>
#include <goog_cc/goog_cc_network_control.h>
#include <api/transport/network_control.h>


std::unique_ptr<webrtc::GoogCcNetworkController> new_goog_cc();

void update_network_availability(const std::unique_ptr<webrtc::GoogCcNetworkController> &controller, webrtc::NetworkAvailability msg);
