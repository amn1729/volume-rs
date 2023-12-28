mod args;
use libpulse_binding::volume::Volume;
use pulsectl::controllers::types::DeviceInfo;
use pulsectl::controllers::{DeviceControl, SinkController};
use std::process::exit;

const MAX_ALLOWED_VOLUME: usize = 200;

fn main() {
    let args = args::get_args().get_matches();
    let mut handler = SinkController::create().unwrap();

    match handler.get_default_device() {
        Err(e) => println!("Could not get default device: {:?}", e),
        Ok(device) => {
            if args.get_flag("get") {
                println!("{:?}", get_device_volume(&device));
                exit(0);
            }

            if args.get_flag("mute") {
                handler.set_device_mute_by_index(device.index, !device.mute);
                exit(0);
            }

            if let Some(val) = args.get_one::<usize>("set") {
                if *val > MAX_ALLOWED_VOLUME {
                    error_exit();
                }

                let v = Volume(655 * *val as u32);
                let mut volume = device.volume;
                volume.set(device.volume.len(), v);
                handler.set_device_volume_by_index(device.index, &volume);
                exit(0);
            }

            if let Some(val) = args.get_one::<usize>("inc") {
                let new_volume = *val + get_device_volume(&device);
                if new_volume > MAX_ALLOWED_VOLUME {
                    error_exit();
                }

                handler.increase_device_volume_by_percent(device.index, *val as f64 / 100.0);
                exit(0);
            }

            if let Some(val) = args.get_one::<usize>("dec") {
                handler.decrease_device_volume_by_percent(device.index, *val as f64 / 100.0);
                exit(0);
            }
        }
    }
}

fn get_device_volume(device: &DeviceInfo) -> usize {
    (device.volume.avg().0 / 655) as usize
}

fn error_exit() {
    println!("Volume can be set only in range 0-{MAX_ALLOWED_VOLUME}");
    exit(1);
}
