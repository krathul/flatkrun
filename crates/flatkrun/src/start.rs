use crate::bindings;
use crate::utils::{FlatkrunContext, VMconfig};
use std::ffi::CString;
// use std::fs::File;

unsafe fn exec_vm(vmcfg: &VMconfig, rootfs: &str, cmd: &str, args: Vec<CString>) {
    bindings::krun_set_log_level(1);

    let krun_ctx = bindings::krun_create_ctx() as u32;
    let ret = bindings::krun_set_vm_config(krun_ctx, vmcfg.num_vcpus, vmcfg.ram_mib);
    if ret < 0 {
        println!("Unable to set VM configuration");
        std::process::exit(-1);
    }

    let vm_rootfs = CString::new(rootfs).unwrap();
    let ret = bindings::krun_set_root(krun_ctx, vm_rootfs.as_ptr() as *const i8);
    if ret < 0 {
        println!("Unable to set root file system");
        std::process::exit(-1);
    }

    let c_workdir = CString::new("/home/kark").unwrap();
    let ret = bindings::krun_set_workdir(krun_ctx, c_workdir.as_ptr() as *const i8);
    if ret < 0 {
        println!("Unable to set working directory");
        std::process::exit(-1);
    }

    let wayland_path = CString::new("/run/user/1000/wayland-0").unwrap();
    let ret = bindings::krun_add_vsock_port(krun_ctx, 6000, wayland_path.as_ptr() as *const i8);
    if ret < 0 {
        println!("Unable to set vsock port");
        std::process::exit(-1);
    }

    let c_cmd = CString::new(cmd).unwrap();
    let mut argv: Vec<*const i8> = Vec::new();
    for a in args.iter() {
        argv.push(a.as_ptr() as *const i8);
    }
    argv.push(std::ptr::null());

    let hostname = CString::new("HOSTNAME=oblivion").unwrap();
    let home = CString::new("HOME=/root").unwrap();
    let path = CString::new("PATH=/bin:/sbin:/usr/bin:/usr/sbin:/usr/local/bin").unwrap();
    let guest_display = CString::new("WAYLAND_DISPLAY=wayland-host").unwrap();
    let guest_xdg_runtime_dir = CString::new("XDG_RUNTIME_DIR=/home/kark").unwrap();
    let host_display_port = CString::new(format!("HOST_DISPLAY_PORT={}", 6000)).unwrap();

    let env: [*const i8; 7] = [
        hostname.as_ptr() as *const i8,
        home.as_ptr() as *const i8,
        path.as_ptr() as *const i8,
        guest_display.as_ptr() as *const i8,
        guest_xdg_runtime_dir.as_ptr() as *const i8,
        host_display_port.as_ptr() as *const i8,
        std::ptr::null(),
    ];

    let ret = bindings::krun_set_exec(
        krun_ctx,
        c_cmd.as_ptr() as *const i8,
        argv.as_ptr() as *const *const i8,
        env.as_ptr() as *const *const i8,
    );

    if ret < 0 {
        println!("Error setting executable");
        std::process::exit(-1);
    }

    let ret = bindings::krun_start_enter(krun_ctx);
    if ret < 0 {
        println!("Error launching the VM");
        std::process::exit(-1);
    }
}

pub fn start(flatkrun_ctx: &FlatkrunContext, rootfs: &str) {
    let cmd = "flatkrun-agent".to_string();
    let args: Vec<CString> = vec![CString::new(flatkrun_ctx.app_name.clone()).unwrap()];
    unsafe { exec_vm(&flatkrun_ctx.vmconfig, rootfs, &cmd, args) };
}
