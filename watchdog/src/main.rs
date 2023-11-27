use sysinfo::{CpuExt, System, SystemExt, ProcessExt, DiskExt, NetworksExt, NetworkExt};
use std::{thread, time};

fn main() {
    let mut sys = System::new_all();

    loop {

        println!("\x1B[2J\x1B[1;1H");

        sys.refresh_all();

        println!("=> disk: ");
        for disk in sys.disks() {
            println!("{:?}", disk);
        }

        println!("=> networks: ");
        for (interface_name, data) in sys.networks() {
            println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
        }

        println!("=> components: ");
        for component in sys.components() {
            println!("{:?}", component);
        }

        println!("=> system: ");

        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory:  {} bytes", sys.used_memory());
        println!("total swap:   {} bytes", sys.total_swap());
        println!("used swap:    {} bytes", sys.used_swap());

        println!("System name:                  {:?}", sys.name());
        println!("System kernel version:        {:?}", sys.kernel_version());
        println!("System OS version:            {:?}", sys.os_version());
        println!("System host name:             {:?}", sys.host_name());

        println!("NB CPUs: {}", sys.cpus().len());

        /*

        for (pid, process) in sys.processes() {
            println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
        }
        */

        println!("{:<20} {:>15} {:>15}", "Network Interface", "Received (KB)", "Transmitted (KB)");
        for (interface_name, data) in sys.networks() {
            println!("{:<20} {:>15} {:>15}",
                interface_name,
                data.received() / 1024,
                data.transmitted() / 1024
                );


        }

        thread::sleep(time::Duration::from_secs(5));
    }
    
}
