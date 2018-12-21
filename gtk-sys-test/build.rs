extern crate ctest;
extern crate shell_words;

static X11_TYPES: &[&str] = &[
    "Window",
    "GtkPlug",
    "GtkPlugClass",
    "GtkSocket",
    "GtkSocketClass",
];

fn pkg_config_cflags() -> Result<Vec<String>, Box<std::error::Error>> {
    let mut cmd = std::process::Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.arg("gtk+-3.0");
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = std::str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

fn main() {
    let target = std::env::var("TARGET").unwrap();
    let windows = target.contains("windows");

    let mut cfg = ctest::TestGenerator::new();

    for flag in pkg_config_cflags().expect("cflags from pkg-config") {
        cfg.flag(&flag);
    }

    cfg.header("gtk/gtk.h");
    cfg.header("gtk/gtk-a11y.h");
    if !windows {
        cfg.header("gtk/gtkx.h");
    }

    // Removed from gtk.
    cfg.skip_const(|c| c == "GTK_CELL_RENDERER_ACCEL_MODE_MODIFIER_TAP");
    cfg.skip_fn(|_| true);
    cfg.skip_signededness(|_| true);

    cfg.type_name(|ty, _is_struct, _is_union| {
        // Use Foo, instead of struct Foo, etc.
        ty.to_string()
    });

    cfg.skip_field(|typ, field| match (typ, field) {
        // Bit fields:
        ("GtkAccelKey", "accel_flags") => true,
        ("GtkBindingSet", "parsed") => true,
        ("GtkContainerClass", "_handle_border_width") => true,
        ("GtkMenuItemClass", "hide_on_activate") => true,
        ("GtkMenuShellClass", "submenu_placement") => true,
        ("GtkRcStyle", "engine_specified") => true,

        _ => false,
    });

    cfg.skip_field_type(|typ, field| match (typ, field) {
        // FIXME: void* / void**:
        ("GtkBuildableIface", "custom_tag_end") => true,
        // FIXME: int / int*
        ("GtkMenuItemClass", "toggle_size_request") => true,

        // FIXME: const mismatch:
        ("GtkCellAreaClass", "set_cell_property") => true,
        ("GtkColorChooserInterface", "get_rgba") => true,
        ("GtkRecentFilterInfo", "applications") => true,
        ("GtkRecentFilterInfo", "groups") => true,
        ("GtkRecentFilterInfo", "set_child_property") => true,
        ("GtkContainerClass", "set_child_property") => true,

        // FIXME: functions pointer vs void**:
        (_, "__gtk_reserved1") => true,
        (_, "__gtk_reserved2") => true,
        (_, "__gtk_reserved3") => true,
        (_, "__gtk_reserved4") => true,

        // Unnamed types:
        ("GtkBindingArg", "d") => true,

        _ => false,
    });

    cfg.field_name(|_typ, field| {
        match field {
            // Restore original field names:
            "box_" => "box",
            "move_" => "move",
            "priv_" => "priv",
            "type_" => "type",
            _ => field,
        }
        .to_string()
    });

    cfg.skip_type(move |typ| match typ {
        t if windows => X11_TYPES.contains(&t),
        _ => false,
    });

    cfg.skip_struct(move |typ| match typ {
        // Unnamed type
        "GtkBindingArg_d" => true,
        "GtkTextAppearance_u1" => true,
        "GtkTextAttributes_u1" => true,

        t if windows => X11_TYPES.contains(&t),

        _ => false,
    });

    cfg.generate("../gtk-sys/src/lib.rs", "all.rs");
}
