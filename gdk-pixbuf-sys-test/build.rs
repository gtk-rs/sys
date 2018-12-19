extern crate ctest;
extern crate shell_words;

fn pkg_config_cflags() -> Result<Vec<String>, Box<std::error::Error>> {
    let mut cmd = std::process::Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.arg("gdk-pixbuf-2.0");
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

    cfg.header("gdk-pixbuf/gdk-pixbuf.h");
    cfg.header("gdk-pixbuf/gdk-pixdata.h");

    cfg.skip_const(|c| c == "GDK_COLORSPACE_RGB");
    cfg.skip_fn(|_| true);
    cfg.skip_signededness(|_| true);

    cfg.type_name(|ty, _is_struct, _is_union| {
        // Use Foo, instead of struct Foo, etc.
        ty.to_string()
    });

    cfg.field_name(|_typ, field| match field {
        // Restore original field names:
        "priv_" => "priv",
        _ => field,
    }.to_string());

    cfg.skip_type(|typ| typ == "Colorspace");

    cfg.skip_struct(|typ| match typ {
        // Incomplete types:
        "GdkPixbuf" => true,
        "GdkPixbufAnimation" => true,
        "GdkPixbufAnimationIter" => true,
        "GdkPixbufFormat" => true,
        "GdkPixbufSimpleAnim" => true,
        "GdkPixbufSimpleAnimClass" => true,

        // Not part of public headers ...
        "GdkPixbufSimpleAnimIter" => true,

        _ => false,
    });

    cfg.generate("../gdk-pixbuf-sys/src/lib.rs", "all.rs");
}

