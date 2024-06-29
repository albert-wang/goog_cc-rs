#include "goog_cc-rs/include/shim.h"

namespace webrtc {
std::unique_ptr<GoogCcNetworkController> new_goog_cc() {

  NetworkControllerConfig config(CreateEnvironment());

  GoogCcConfig goog_cc_config;

  return std::unique_ptr<GoogCcNetworkController>(new GoogCcNetworkController(config, std::move(goog_cc_config)));
}
}