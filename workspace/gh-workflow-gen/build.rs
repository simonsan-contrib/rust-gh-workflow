use gh_workflow::*;
use gh_workflow_release_plz::{Command, ReleasePlz};
use toolchain::Toolchain;

fn main() {
    let flags = RustFlags::deny("warnings");

    let build = Job::new("Build and Test")
        .add_step(Step::checkout())
        .add_step(
            Toolchain::default()
                .add_stable()
                .add_nightly()
                .add_clippy()
                .add_fmt(),
        )
        .add_step(
            Cargo::new("test")
                .args("--all-features --workspace")
                .name("Cargo Test"),
        )
        .add_step(
            Cargo::new("fmt")
                .nightly()
                .args("--check")
                .name("Cargo Fmt"),
        )
        .add_step(
            Cargo::new("clippy")
                .nightly()
                .args("--all-features --workspace -- -D warnings")
                .name("Cargo Clippy"),
        );

    let event = Event::default()
        .push(Push::default().add_branch("main"))
        .pull_request_target(
            PullRequestTarget::default()
                .open()
                .synchronize()
                .reopen()
                .add_branch("main"),
        );

    let release = Job::new("Release")
        .needs("build")
        .add_env(("GITHUB_TOKEN", "${{ secrets.GH_TOKEN }}"))
        .add_step(Step::checkout())
        .add_step(ReleasePlz::default().command(Command::ReleasePR));

    Workflow::new("Build and Test")
        .add_env(flags)
        .permissions(Permissions::read())
        .on(event)
        .add_job("build", build)
        .add_job("release", release)
        .generate()
        .unwrap();
}
