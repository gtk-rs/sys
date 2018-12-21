extern crate ctest;
extern crate shell_words;

static UNIX_TYPES: &[&str] = &[
    "GUnixFDSourceFunc",
];

fn pkg_config_cflags() -> Result<Vec<String>, Box<std::error::Error>> {
    let mut cmd = std::process::Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.arg("glib-2.0");
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}",
                           &cmd, out.status).into());
    }
    let stdout = std::str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

fn main() {
    let target = std::env::var("TARGET").unwrap();
    let windows = target.contains("windows");

    let mut cfg = ctest::TestGenerator::new();

    for flag in pkg_config_cflags().unwrap() {
        cfg.flag(&flag);
    }

    cfg.header("glib.h");
    cfg.header("glib-object.h");
    cfg.header("glib/gstdio.h");
    if !windows {
        cfg.header("glib-unix.h");
    }

    cfg.skip_const(|c| match c {
        // GLib header define it on Windows only.
        "G_WIN32_MSG_HANDLE" => true,

        // GLib uses TRUE and FALSE
        "GTRUE" => true,
        "GFALSE" => true,

        _ => false,
    });
    cfg.skip_fn(|_| true);
    cfg.skip_signededness(|_| true);

    cfg.type_name(|ty, _is_struct, _is_union| {
        // Use Foo, instead of struct Foo, etc.
        ty.to_string()
    });

    cfg.skip_field(|typ, field| match (typ, field) {
        // Bit fields.
        ("GDate", "julian_days") => true,
        ("GIOChannel", "use_buffer") => true,
        ("GScannerConfig", "case_sensitive") => true,

        // Manual bit field replacements.
        ("GHookList", "hook_size_and_setup") => true,

        _ => false,
    });

    cfg.skip_field_type(|typ, field| match (typ, field) {
        // FIXME in gir: const mismatch:
        ("GIOFuncs", "io_read") => true,
        // FIXME in gir: long instead of long double:
        ("GTestLogMsg", "nums") => true,

        // Volatile:
        ("GOnce", "status") => true,
        ("GOnce", "retval") => true,

        // Unnamed types:
        ("GVariantBuilder", _) => true,
        ("GVariantDict", _) => true,

        _ => false,
    });

    cfg.field_name(|_typ, field| match field {
        // Unfix rust keywords in field names.
        "priv_" => "priv",
        "ref_" => "ref",
        _ => field,
    }.to_string());

    cfg.skip_type(move |typ| match typ {
        "ThreadError" => true,
        t if windows => UNIX_TYPES.contains(&t),
        _ => false,
    });

    cfg.skip_struct(move |typ| match typ {
        // Unnamed types:
        "GDoubleIEEE754_mpn" => true,
        "GFloatIEEE754_mpn" => true,
        "GVariantBuilder_u" => true,
        "GVariantBuilder_u_s" => true,
        "GVariantDict_u" => true,
        "GVariantDict_u_s" => true,

        _ => false,
    });

    cfg.generate("../glib-sys/src/lib.rs", "all.rs");
}

