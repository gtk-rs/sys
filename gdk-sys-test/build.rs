extern crate ctest;
extern crate shell_words;

fn pkg_config_cflags() -> Result<Vec<String>, Box<std::error::Error>> {
    let mut cmd = std::process::Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.arg("gdk-3.0");
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}",
                           &cmd, out.status).into());
    }
    let stdout = std::str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    for flag in pkg_config_cflags().unwrap() {
        cfg.flag(&flag);
    }

    cfg.header("gdk/gdk.h");

    cfg.skip_fn(|_| true);
    cfg.skip_signededness(|_| true);

    cfg.type_name(|ty, _is_struct, _is_union| {
        // Use Foo, instead of struct Foo, etc.
        ty.to_string()
    });

    cfg.field_name(|_typ, field| match field {
        // Restore original field names:
        "in_" => "in",
        "type_" => "type",
        _ => field,
    }.to_string());

    cfg.skip_field(|typ,field| match (typ, field) {
        // Bit fields:
        ("GdkEventKey", "is_modifier") => true,
        ("GdkEventScroll", "is_stop") => true,

        _ => false,
    });

    cfg.generate("../gdk-sys/src/lib.rs", "all.rs");
}

