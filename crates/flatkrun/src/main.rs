use clap::{crate_version, Arg, ArgMatches, Command};
use flatkrun::start::start;
use flatkrun::utils::{mount_container, unmount_container, FlatkrunContext};

fn check_unshare() {
    let uid = unsafe { libc::getuid() };
    if uid != 0
        && std::env::vars()
            .find(|(key, _)| key == "BUILDAH_ISOLATION")
            .is_none()
    {
        println!("Please re-run flatkrun inside a \"buildah unshare\" session");
        std::process::exit(-1);
    }
}

fn setup(flatkrun_ctx: &mut FlatkrunContext, matches: &ArgMatches) {
    flatkrun_ctx.container_id = matches
        .get_one::<String>("containerID")
        .unwrap()
        .to_string();
    flatkrun_ctx.app_name = matches.get_one::<String>("APP").unwrap().to_string();
}

fn main() {
    let app = Command::new("flatkrun")
        .version(crate_version!())
        .author("Athul Raj Kollareth <krathul3152@gmail.com>")
        .about("Run flatpak apps in a microVM")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .about("run a flatpak app")
                .arg(
                    Arg::new("containerID")
                        .help("ID of the container")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("APP")
                        .help("flatpak app to run")
                        .index(2)
                        .required(true),
                ),
        );

    let matches = app.get_matches();
    check_unshare();
    let mut flatkrun_ctx = FlatkrunContext::default();
    if let Some(matching) = matches.subcommand_matches("run") {
        setup(&mut flatkrun_ctx, matching);
        unmount_container(&flatkrun_ctx.container_id).expect("Unable to unmount the container");
        let rootdir =
            mount_container(&flatkrun_ctx.container_id).expect("Unable to mount container");
        start(&flatkrun_ctx, &rootdir);
        unmount_container(&flatkrun_ctx.container_id).expect("Unable to unmount the container");
    }
}
