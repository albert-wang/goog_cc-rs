#include <iostream>

#include "goog_cc-rs/src/main.rs.h" 

using namespace webrtc;

namespace goog_cc_rs {

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

  goog_cc_rs::NetworkControlUpdate update_network_availability(const std::unique_ptr<GoogCcNetworkController> &controller, webrtc::NetworkAvailability msg) {
    auto ret = controller->OnNetworkAvailability(msg);

    return goog_cc_rs::NetworkControlUpdate {
      target_rate: {
        at_time: 0,
        target_rate: ret.target_rate->target_rate.bps()
      }
    };
  }

}