fn main() {
  if pkg_config::probe_library("appindicator3").is_ok()
    || pkg_config::probe_library("appindicator3-0.1").is_ok()
  {
    println!("cargo:rustc-link-lib=appindicator3");
  } else if pkg_config::probe_library("ayatana-appindicator3").is_ok()
    || pkg_config::probe_library("ayatana-appindicator3-0.1").is_ok()
  {
    println!("cargo:rustc-link-lib=ayatana-appindicator3");
  } else {
    panic!("libappindicator3 or libayatana-appindicator3 library not found!");
  };
}
