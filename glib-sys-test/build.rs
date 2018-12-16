extern crate ctest;
extern crate shell_words;

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
    let mut cfg = ctest::TestGenerator::new();

    for flag in pkg_config_cflags().unwrap() {
        cfg.flag(&flag);
    }

    cfg.header("glib-object.h");
    cfg.header("glib-unix.h");
    cfg.header("glib.h");

    cfg.skip_const(|_| true);
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

    cfg.skip_type(|t| {
        t == "ThreadError"
    });

    cfg.skip_struct(|typ| match typ {
        // Incomplete types, need manual exclusion since ctype
        // doesn't understand "pub struct Type(c_void)" pattern:
        "GAsyncQueue" => true,
        "GBookmarkFile" => true,
        "GBytes" => true,
        "GChecksum" => true,
        "GData" => true,
        "GDate" => true,
        "GDateTime" => true,
        "GDir" => true,
        "GDoubleIEEE754" => true,
        "GDoubleIEEE754_mpn" => true,
        "GFloatIEEE754" => true,
        "GFloatIEEE754_mpn" => true,
        "GHashTable" => true,
        "GHmac" => true,
        "GIOChannel" => true,
        "GKeyFile" => true,
        "GMainContext" => true,
        "GMainLoop" => true,
        "GMappedFile" => true,
        "GMarkupParseContext" => true,
        "GMatchInfo" => true,
        "GOptionContext" => true,
        "GOptionGroup" => true,
        "GPatternSpec" => true,
        "GRand" => true,
        "GRegex" => true,
        "GSequence" => true,
        "GSequenceIter" => true,
        "GSourcePrivate" => true,
        "GStatBuf" => true,
        "GStringChunk" => true,
        "GTestCase" => true,
        "GTestSuite" => true,
        "GThread" => true,
        "GTimeZone" => true,
        "GTimer" => true,
        "GTree" => true,
        "GVariant" => true,
        "GVariantBuilder_u" => true,
        "GVariantBuilder_u_s" => true,
        "GVariantDict_u" => true,
        "GVariantDict_u_s" => true,
        "GVariantType" => true,
        "_GIConv" => true,
        // Unnamed types:

        _ => false,
    });

    cfg.generate("../glib-sys/src/lib.rs", "all.rs");
}

