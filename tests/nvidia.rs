pub mod common;

#[cfg(test)]
mod test{
    use tracing::info;


    #[test]
    fn nvidia_test(){
        use nvml_wrapper::Nvml;
        crate::common::setup();
        let nvml = Nvml::init();;
        info!("{:?}",nvml)
    //     // Get the first `Device` (GPU) in the system
    //     let device = nvml.device_by_index(0)?;

    //     let brand = device.brand()?; // GeForce on my system
    //     let fan_speed = device.fan_speed(0)?; // Currently 17% on my system
    //     let power_limit = device.enforced_power_limit()?; // 275k milliwatts on my system
    //     let encoder_util = device.encoder_utilization()?; // Currently 0 on my system; Not encoding anything
    //     let memory_info = device.memory_info()?; 
    // }
    }
}
