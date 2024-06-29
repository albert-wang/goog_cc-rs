#[cxx::bridge]
mod ffi {

    struct NetworkAvailability {
        network_available: bool
    }

    #[namespace = "webrtc"]
    unsafe extern "C++" {
        include!("goog_cc-rs/include/types.h");

        type NetworkControllerConfig;
        type GoogCcNetworkController;
        type NetworkAvailability;
        type NetworkControlUpdate;
        type ProbeClusterConfig;
    }

    unsafe extern "C++" {
        include!("goog_cc-rs/include/shim.h");

        fn new_goog_cc() -> UniquePtr<GoogCcNetworkController>;

        fn update_network_availability(controller: &UniquePtr<GoogCcNetworkController>, msg: NetworkAvailability);
    }
}

fn main() {
    let goog_cc: cxx::UniquePtr<ffi::GoogCcNetworkController> = ffi::new_goog_cc();

    let network_availability = ffi::NetworkAvailability {
        network_available: true
    };

    ffi::update_network_availability(&goog_cc, network_availability);
}
