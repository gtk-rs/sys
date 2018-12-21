extern crate ctest;
extern crate shell_words;

fn pkg_config_cflags() -> Result<Vec<String>, Box<std::error::Error>> {
    let mut cmd = std::process::Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.arg("pango");
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

    cfg.define("PANGO_ENABLE_BACKEND", None);
    cfg.define("PANGO_ENABLE_ENGINE", None);

    for flag in pkg_config_cflags().expect("cflags from pkg-config") {
        cfg.flag(&flag);
    }

    cfg.header("pango/pango.h");
    cfg.header("pango/pango-modules.h");

    cfg.skip_fn(|_| true);
    cfg.skip_signededness(|_| true);

    cfg.type_name(|ty, _is_struct, _is_union| {
        // Use Foo, instead of struct Foo, etc.
        ty.to_string()
    });

    cfg.field_name(|_typ, field| match field {
        // Restore original field names:
        "priv_" => "priv",
        "type_" => "type",
        _ => field,
    }.to_string());

    cfg.skip_type(|typ| typ == "TabAlign");

    cfg.skip_field(|typ,field| match (typ, field) {
        // Bit fields:
        ("PangoAttrSize", "absolute") => true,
        ("PangoGlyphVisAttr", "is_cluster_start") => true,
        _ => false,
    });

    cfg.skip_struct(|typ| match typ {
        // Removed from pango?
        "PangoScriptForLang" => true,

        _ => false,
    });

    cfg.generate("../pango-sys/src/lib.rs", "all.rs");
}
