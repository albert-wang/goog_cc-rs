#[cxx::bridge(namespace = "webrtc")]
mod ffi {
    struct NetworkAvailability {
        network_available: bool
    }

    #[derive(Debug)]
    struct NetworkControlUpdate {
        test: bool
    }

    unsafe extern "C++" {
        include!("goog_cc-rs/include/shim.h");

        type NetworkControllerConfig;
        type GoogCcNetworkController;
        type NetworkAvailability;
        type NetworkControlUpdate;

        fn new_goog_cc() -> UniquePtr<GoogCcNetworkController>;

        fn OnNetworkAvailability(self: Pin<&mut GoogCcNetworkController>, msg: NetworkAvailability) -> NetworkControlUpdate;
    }
}

fn main() {
    let mut goog_cc: cxx::UniquePtr<ffi::GoogCcNetworkController> = ffi::new_goog_cc();

    let network_availability = ffi::NetworkAvailability {
        network_available: true
    };

    let network_control_update = goog_cc.as_mut().unwrap().OnNetworkAvailability(network_availability);
    println!("NetworkControlUpdate: {:?}", network_control_update);
}
