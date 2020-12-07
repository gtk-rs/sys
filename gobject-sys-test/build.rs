extern crate ctest;
extern crate shell_words;

fn pkg_config_cflags() -> Result<Vec<String>, Box<std::error::Error>> {
    let mut cmd = std::process::Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.arg("gobject-2.0");
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = std::str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    for flag in pkg_config_cflags().expect("cflags from pkg-config") {
        cfg.flag(&flag);
    }

    cfg.header("glib-object.h");
    cfg.header("gobject/gvaluecollector.h");

    cfg.skip_fn(|_| true);
    cfg.skip_signededness(|_| true);

    cfg.type_name(|ty, _is_struct, _is_union| {
        // Use Foo, instead of struct Foo, etc.
        ty.to_string()
    });

    cfg.skip_field_type(|typ, field| match (typ, field) {
        // FIXME in gir: const mismatch:
        ("GInitiallyUnownedClass", "set_property") => true,
        ("GObjectClass", "set_property") => true,
        ("GTypeValueTable", "collect_value") => true,

        // Const:
        ("GSignalQuery", "param_types") => true,

        // Volatile:
        ("GInitiallyUnowned", "ref_count") => true,
        ("GObject", "ref_count") => true,
        ("GWeakRef", "priv_") => true,

        // Unnamed type:
        ("GValue", "data") => true,

        _ => false,
    });

    cfg.field_name(|_typ, field| {
        match field {
            // Unfix rust keywords in field names.
            "priv_" => "priv",
            "type_" => "type",
            _ => field,
        }
        .to_string()
    });

    cfg.skip_struct(|typ| match typ {
        // Unnamed types:
        "GValue_data" => true,
        "GWeakRef_priv" => true,

        _ => false,
    });

    cfg.generate("../gobject-sys/src/lib.rs", "all.rs");
}
