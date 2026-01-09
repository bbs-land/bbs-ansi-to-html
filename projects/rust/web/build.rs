use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    // Navigate up to find the target directory (OUT_DIR is deeply nested)
    // OUT_DIR is typically: target/debug/build/<pkg>/out
    let target_dir = out_path
        .ancestors()
        .nth(3)
        .expect("Could not find target directory");

    let wwwroot_dest = target_dir.join("wwwroot");

    // Source wwwroot directory (in project root, three levels up from crate)
    // Path: projects/rust/web -> projects/rust -> projects -> repo-root/wwwroot
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = Path::new(&manifest_dir).parent().unwrap(); // projects/rust/
    let projects_dir = workspace_root.parent().unwrap(); // projects/
    let repo_root = projects_dir.parent().unwrap(); // repo root
    let wwwroot_src = repo_root.join("wwwroot");

    // Copy wwwroot to target directory
    if wwwroot_src.exists() {
        copy_dir_recursive(&wwwroot_src, &wwwroot_dest).expect("Failed to copy wwwroot");
        println!("cargo:warning=Copied wwwroot to {:?}", wwwroot_dest);
    }

    // Tell Cargo to rerun this script if wwwroot contents change
    println!("cargo:rerun-if-changed=../../../wwwroot");
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
