fn main() {
    let mut build = cxx_build::bridge("src/main.rs"); // returns a cc::Build
    build.file("src/shim.cc")

        // system wrappers
        .file("third_party/webrtc/system_wrappers/source/clock.cc")
        .file("third_party/webrtc/system_wrappers/source/metrics.cc")
        .file("third_party/webrtc/system_wrappers/source/field_trial.cc")

        // rtc_base
        .file("third_party/webrtc/rtc_base/checks.cc")
        .file("third_party/webrtc/rtc_base/logging.cc")
        .file("third_party/webrtc/rtc_base/race_checker.cc")
        .file("third_party/webrtc/rtc_base/platform_thread_types.cc")
        .file("third_party/webrtc/rtc_base/system_time.cc")
        .file("third_party/webrtc/rtc_base/time_utils.cc")
        .file("third_party/webrtc/rtc_base/string_utils.cc")
        .file("third_party/webrtc/rtc_base/string_encode.cc")
        .file("third_party/webrtc/rtc_base/strings/string_builder.cc")
        .file("third_party/webrtc/rtc_base/task_queue_stdlib.cc") // Change per platform???
        .file("third_party/webrtc/rtc_base/platform_thread.cc")
        .file("third_party/webrtc/rtc_base/event.cc")
        .file("third_party/webrtc/rtc_base/synchronization/yield_policy.cc")

        // experiments
        .file("third_party/webrtc/rtc_base/experiments/field_trial_parser.cc")
        .file("third_party/webrtc/rtc_base/experiments/field_trial_units.cc")
        .file("third_party/webrtc/rtc_base/experiments/struct_parameters_parser.cc")
        .file("third_party/webrtc/rtc_base/experiments/rate_control_settings.cc")
        .file("third_party/webrtc/rtc_base/experiments/alr_experiment.cc")
        .file("third_party/webrtc/rtc_base/experiments/field_trial_list.cc")

        // events
        .file("third_party/webrtc/logging/rtc_event_log/events/rtc_event_alr_state.cc")
        .file("third_party/webrtc/logging/rtc_event_log/events/rtc_event_probe_result_failure.cc")
        .file("third_party/webrtc/logging/rtc_event_log/events/rtc_event_probe_result_success.cc")
        .file("third_party/webrtc/logging/rtc_event_log/events/rtc_event_bwe_update_delay_based.cc")
        .file("third_party/webrtc/logging/rtc_event_log/events/rtc_event_probe_cluster_created.cc")
        .file("third_party/webrtc/logging/rtc_event_log/events/rtc_event_bwe_update_loss_based.cc")

        // api
        .file("third_party/webrtc/api/field_trials_registry.cc")
        .file("third_party/webrtc/api/transport/field_trial_based_config.cc")
        .file("third_party/webrtc/api/transport/network_types.cc")
        .file("third_party/webrtc/api/environment/environment_factory.cc")
        .file("third_party/webrtc/api/task_queue/default_task_queue_factory_stdlib.cc") // Change per platform???
        .file("third_party/webrtc/api/task_queue/task_queue_base.cc")
        .file("third_party/webrtc/api/rtc_event_log/rtc_event_log.cc")
        .file("third_party/webrtc/api/rtc_event_log/rtc_event.cc")
        .file("third_party/webrtc/api/units/data_rate.cc")
        .file("third_party/webrtc/api/units/data_size.cc")
        .file("third_party/webrtc/api/units/time_delta.cc")
        .file("third_party/webrtc/api/units/timestamp.cc")

        // pacing
        .file("third_party/webrtc/modules/pacing/interval_budget.cc")

        // remote_bitrate_estimator
        .file("third_party/webrtc/modules/remote_bitrate_estimator/bwe_defines.cc")
        .file("third_party/webrtc/modules/remote_bitrate_estimator/aimd_rate_control.cc")

        // congestion_controller
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/bitrate_estimator.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/inter_arrival_delta.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/goog_cc_network_control.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/alr_detector.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/delay_based_bwe.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/probe_controller.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/trendline_estimator.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/link_capacity_estimator.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/probe_bitrate_estimator.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/send_side_bandwidth_estimation.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/loss_based_bwe_v2.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/loss_based_bandwidth_estimation.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/congestion_window_pushback_controller.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/acknowledged_bitrate_estimator_interface.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/acknowledged_bitrate_estimator.cc")
        .file("third_party/webrtc/modules/congestion_controller/goog_cc/robust_throughput_estimator.cc")

        .include("third_party/abseil-cpp/")
        .include("third_party/webrtc/")
        .include("third_party/webrtc/modules/congestion_controller/");

    // OS define flags
    if cfg!(target_os = "linux") {
        build.define("WEBRTC_LINUX", None);
        build.define("WEBRTC_POSIX", None);
    } else if cfg!(target_os = "windows") {
        build.define("WEBRTC_WIN", None);
    } else if cfg!(target_os = "macos") {
        build.define("WEBRTC_MAC", None);
        build.define("WEBRTC_POSIX", None);
    }

    build.std("c++17").compile("goog_cc-rs");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/shim.cc");
    println!("cargo:rerun-if-changed=include/shim.h");
}
