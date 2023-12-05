use sysinfo::{CpuExt, System, SystemExt, ProcessExt, DiskExt, NetworksExt, NetworkExt};
use std::{thread, time};
use serde_json::json;
use std::fs::File;
use std::io::Write;
use serde::ser::{Serialize, Serializer, SerializeStruct};

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



        for (pid, process) in sys.processes() {
            println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
        }

        let processes_info: Vec<_> = sys.processes()
            .iter()
            .map(|(&pid, process)| {
                json!({
                    "pid": serialize_pid(&pid),
                    "name": process.name(),
                    "disk_usage": {
                        "read_bytes": process.disk_usage().read_bytes,
                        "written_bytes": process.disk_usage().written_bytes
                    }

                })

            })
            .collect();

        let json_string = serde_json::to_string(&processes_info).unwrap();
        let mut file = File::create("processes_info.json").unwrap();

        writeln!(file, "{}", json_string).unwrap();



        thread::sleep(time::Duration::from_secs(5));
    }
    
}

fn serialize_pid<S>(pid: &sysinfo::Pid, serializer: S) -> Result<S::Ok, S::Error>
where 
    S: Serializer,
{
    let pid = *pid as i32;
    serializer.serialize_i32(pid)

}