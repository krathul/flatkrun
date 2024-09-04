use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process;

#[derive(Debug, Serialize, Deserialize)]
pub struct VMconfig {
    pub num_vcpus: u8,
    pub ram_mib: u32,
    pub workdir: String,
    pub mapped_volumes: HashMap<String, String>,
    pub mapped_ports: HashMap<String, String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FlatkrunContext {
    pub container_id: String,
    pub app_name: String,
    pub vmconfig: VMconfig,
}

impl Default for VMconfig {
    fn default() -> Self {
        VMconfig {
            num_vcpus: 2,
            ram_mib: 500,
            workdir: String::new(),
            mapped_volumes: HashMap::new(),
            mapped_ports: HashMap::new(),
        }
    }
}

pub fn mount_container(container_id: &str) -> Result<String, std::io::Error> {
    let args = vec!["mount", container_id];
    let output = match process::Command::new("buildah")
        .args(&args)
        .stderr(std::process::Stdio::inherit())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                println!("buildah was not found");
            } else {
                println!("Error execting \"buildah mount\" : {}", err.to_string());
            }
            std::process::exit(-1);
        }
    };
    if !output.status.success() {
        println!(
            "buildah returned an error: {}",
            std::str::from_utf8(&output.stdout).unwrap()
        );
        std::process::exit(-1);
    }
    let rootfs = std::str::from_utf8(&output.stdout).unwrap().trim();
    return Ok(rootfs.to_string());
}

pub fn unmount_container(container_id: &str) -> Result<(), std::io::Error> {
    let args = vec!["unmount", container_id];
    let output = match process::Command::new("buildah")
        .args(&args)
        .stderr(std::process::Stdio::inherit())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                println!("buildah was not found");
            } else {
                println!("Error execting \"buildah mount\" : {}", err.to_string());
            }
            std::process::exit(-1);
        }
    };
    if !output.status.success() {
        println!(
            "buildah returned an error: {}",
            std::str::from_utf8(&output.stdout).unwrap()
        );
        std::process::exit(-1);
    }
    return Ok(());
}
