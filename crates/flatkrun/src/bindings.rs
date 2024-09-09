#[link(name = "krun")]
extern "C" {
    pub fn krun_set_log_level(level: u32) -> i32;
    pub fn krun_create_ctx() -> i32;
    pub fn krun_free_ctx(ctx_id: i32) -> i32;
    pub fn krun_set_vm_config(ctx_id: u32, num_vcpus: u8, ram_mib: u32) -> i32;
    pub fn krun_set_root(ctx_id: u32, root_path: *const i8) -> i32;
    pub fn krun_set_mapped_volumes(ctx_id: u32, mapped_volumes: *const *const i8) -> i32;
    pub fn krun_set_port_map(ctx_id: u32, port_map: *const *const i8) -> i32;
    pub fn krun_set_workdir(ctx_id: u32, workdir_path: *const i8) -> i32;
    pub fn krun_set_exec(
        ctx_id: u32,
        exec_path: *const i8,
        argv: *const *const i8,
        envp: *const *const i8,
    ) -> i32;
    pub fn krun_add_vsock_port(ctx_id: u32, port: u32, c_filepath: *const i8) -> i32;
    pub fn krun_start_enter(ctx_id: u32) -> i32;
}
