use cargo_lock::Lockfile;

fn main() {
    println!("cargo:rerun-if-changed=Cargo.lock");
    let lockfile = Lockfile::load("../Cargo.lock").expect("a valid lockfile");
    let mut iter = lockfile
        .packages
        .into_iter()
        .filter(|p| p.name.as_str() == "cedar-policy");
    let version = iter
        .next()
        .expect("cedar-policy is not found in manifest")
        .version;

    assert!(iter.next().is_none());

    println!("cargo:rustc-env=CEDAR_VERSION={version}");
}