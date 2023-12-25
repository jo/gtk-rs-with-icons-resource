fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources.gresource.xml",
        "gtk-rs-with-icons-resource.gresource",
    );
}
