#include <iostream>

#include "goog_cc-rs/include/shim.h"

namespace webrtc {

const uint32_t kInitialBitrateKbps = 60;

NetworkControllerConfig InitialConfig(
    int starting_bandwidth_kbps = kInitialBitrateKbps,
    int min_data_rate_kbps = 0,
    int max_data_rate_kbps = 5 * kInitialBitrateKbps) {
  NetworkControllerConfig config(CreateEnvironment());
  config.constraints.at_time = Timestamp::Zero();
  config.constraints.min_data_rate = DataRate::KilobitsPerSec(min_data_rate_kbps);
  config.constraints.max_data_rate = DataRate::KilobitsPerSec(max_data_rate_kbps);
  config.constraints.starting_rate = DataRate::KilobitsPerSec(starting_bandwidth_kbps);
  return config;
}

std::unique_ptr<GoogCcNetworkController> new_goog_cc() {
  NetworkControllerConfig config = InitialConfig();

  GoogCcConfig goog_cc_config;

  return std::unique_ptr<GoogCcNetworkController>(new GoogCcNetworkController(config, std::move(goog_cc_config)));
}

void update_network_availability(const std::unique_ptr<GoogCcNetworkController> &controller, NetworkAvailability msg) {
  std::cout << "update_network_availability:" << msg.network_available << std::endl;
  auto ret = controller->OnNetworkAvailability(msg);
  std::cout << "update_network_availability: " << ret.target_rate->target_rate.kbps() << std::endl;
}



}