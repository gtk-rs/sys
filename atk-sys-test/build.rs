extern crate ctest;
extern crate shell_words;

fn pkg_config_cflags() -> Result<Vec<String>, Box<std::error::Error>> {
    let mut cmd = std::process::Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.arg("atk");
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

    cfg.header("atk/atk.h");

    cfg.skip_const(|_| true);
    cfg.skip_fn(|_| true);
    cfg.skip_signededness(|_| true);

    cfg.type_name(|ty, _is_struct, _is_union| {
        // Use Foo, instead of struct Foo, etc.
        ty.to_string()
    });

    cfg.field_name(|_typ, field| match field {
        // Restore original field names:
        "type_" => "type",
        _ => field,
    }.to_string());

    cfg.skip_field_type(|typ, field| match (typ, field) {
        // FIXME in gir generator: int vs int**
        ("AtkTableIface", "get_column_at_index") => true,
        ("AtkTableIface", "get_row_at_index") => true,

        // FIXME: void* / void**
        ("AtkObjectClass", "initialize") => true,

        _ => false,
    });

    cfg.skip_struct(|typ| match typ {
        // Incomplete types:
        "AtkAction" => true,
        "AtkComponent" => true,
        "AtkDocument" => true,
        "AtkEditableText" => true,
        "AtkHyperlinkImpl" => true,
        "AtkHypertext" => true,
        "AtkImage" => true,
        "AtkImplementor" => true,
        "AtkImplementorIface" => true,
        "AtkRange" => true,
        "AtkSelection" => true,
        "AtkStreamableContent" => true,
        "AtkTable" => true,
        "AtkTableCell" => true,
        "AtkText" => true,
        "AtkValue" => true,
        "AtkWindow" => true,

        // Interface was changed upstream
        "AtkComponentIface" => true,

        _ => false,
    });

    cfg.generate("../atk-sys/src/lib.rs", "all.rs");
}

