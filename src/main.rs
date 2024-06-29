use cxx::{CxxString, CxxVector};

#[cxx::bridge(namespace = "webrtc")]
mod ffi {
    struct NetworkAvailability {
        network_available: bool
    }

    unsafe extern "C++" {
        include!("goog_cc-rs/include/shim.h");

        type NetworkControllerConfig;
        type GoogCcNetworkController;
        type NetworkAvailability;
        type NetworkControlUpdate;
        type ProbeClusterConfig;

        fn new_goog_cc() -> UniquePtr<GoogCcNetworkController>;

       // fn OnNetworkAvailability(self: Pin<&mut GoogCcNetworkController>, msg: NetworkAvailability) -> NetworkControlUpdate;

        fn update_network_availability(controller: &UniquePtr<GoogCcNetworkController>, msg: NetworkAvailability);

        
    }
}

fn main() {
    let mut goog_cc: cxx::UniquePtr<ffi::GoogCcNetworkController> = ffi::new_goog_cc();

    let network_availability = ffi::NetworkAvailability {
        network_available: true
    };

    ffi::update_network_availability(&goog_cc, network_availability);

   // let network_control_update = goog_cc.as_mut().unwrap().OnNetworkAvailability(network_availability);
   
    //println!("NetworkControlUpdate: {:?}", network_control_update.probe_cluster_configs.len());
}
