extern crate ctest;
extern crate shell_words;

fn pkg_config_cflags() -> Result<Vec<String>, Box<std::error::Error>> {
    let mut cmd = std::process::Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.arg("pangocairo");
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

    cfg.header("pango/pangocairo.h");

    cfg.type_name(|ty, _is_struct, _is_union| {
        // Use Foo, instead of struct Foo, etc.
        ty.to_string()
    });

    cfg.field_name(|_typ, field| match field {
        // Restore original field names:
        //"type_" => "type",
        _ => field,
    }.to_string());

    cfg.skip_fn(|fun| match fun {
        // Not part of public headers (on Debian):
        "pango_cairo_fc_font_map_get_type" => true,
        _ => false,
    });

    cfg.skip_field_type(|typ, field| match (typ, field) {
        _ => false,
    });

    cfg.skip_struct(|typ| match typ {
        // Not part of public headers (on Debian):
        "PangoCairoFcFontMap" => true,

        _ => false,
    });

    cfg.generate("../pangocairo-sys/src/lib.rs", "all.rs");
}

