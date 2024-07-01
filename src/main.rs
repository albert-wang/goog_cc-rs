#[cxx::bridge(namespace="goog_cc_rs")]
mod ffi {
    struct NetworkAvailability {
        network_available: bool
    }

    #[derive(Debug)]
    struct NetworkControlUpdate {
        target_rate: TargetTransferRate
    }

    #[derive(Debug)]
    struct TargetTransferRate {
        at_time: u64,
        target_rate: i64
    }

    #[namespace = "webrtc"]
    unsafe extern "C++" {
        include!("goog_cc-rs/include/webrtc_types.h");

        type NetworkControllerConfig;
        type GoogCcNetworkController;
        type NetworkAvailability;
        type ProbeClusterConfig;
    }

    unsafe extern "C++" {
        include!("goog_cc-rs/include/shim.h");

        fn new_goog_cc() -> UniquePtr<GoogCcNetworkController>;

        fn update_network_availability(controller: &UniquePtr<GoogCcNetworkController>, msg: NetworkAvailability) -> NetworkControlUpdate;
    }
}

fn main() {
    let goog_cc: cxx::UniquePtr<ffi::GoogCcNetworkController> = ffi::new_goog_cc();

    let network_availability = ffi::NetworkAvailability {
        network_available: true
    };

    let update = ffi::update_network_availability(&goog_cc, network_availability);

    println!("{:?}", update);
}

