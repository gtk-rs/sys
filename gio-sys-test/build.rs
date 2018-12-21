extern crate ctest;
extern crate shell_words;


static UNIX_TYPES: &[&str] = &[
    "GDesktopAppInfoClass",
    "GDesktopAppInfoLookupIface",
    "GDesktopAppLaunchCallback",
    "GDesktopAppLaunchCallback",
    "GFileDescriptorBasedIface",
    "GUnixConnection",
    "GUnixConnectionClass",
    "GUnixCredentialsMessage",
    "GUnixCredentialsMessageClass",
    "GUnixFDList",
    "GUnixFDListClass",
    "GUnixFDMessage",
    "GUnixFDMessageClass",
    "GUnixInputStream",
    "GUnixInputStreamClass",
    "GUnixOutputStream",
    "GUnixOutputStreamClass",
    "GUnixSocketAddress",
    "GUnixSocketAddressClass",
];

fn pkg_config_cflags(windows: bool) -> Result<Vec<String>, Box<std::error::Error>> {
    let mut cmd = std::process::Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.arg("gio-2.0");
    if !windows {
        cmd.arg("gio-unix-2.0");
    }
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

    for flag in pkg_config_cflags(windows).unwrap() {
        cfg.flag(&flag);
    }

    cfg.header("gio/gio.h");
    cfg.define("G_SETTINGS_ENABLE_BACKEND", None);
    cfg.header("gio/gsettingsbackend.h");

    if !windows {
        cfg.header("gio/gdesktopappinfo.h");
        cfg.header("gio/gfiledescriptorbased.h");
        cfg.header("gio/gunixconnection.h");
        cfg.header("gio/gunixcredentialsmessage.h");
        cfg.header("gio/gunixfdmessage.h");
        cfg.header("gio/gunixinputstream.h");
        cfg.header("gio/gunixoutputstream.h");
        cfg.header("gio/gunixsocketaddress.h");
    }

    cfg.skip_fn(|_| true);
    cfg.skip_signededness(|_| true);

    cfg.type_name(|typ, _is_struct, _is_union| match typ {
        // FIXME in gir:
        "TlsCertificateRequestFlags" => "GTlsCertificateRequestFlags",

        // Use Foo, instead of struct Foo, etc.
        _ => typ,
    }.to_string());

    cfg.skip_field_type(|typ, field| match (typ, field) {
        // FIXME in gir: const mismatch:
        ("GAppInfoIface", "get_supported_types") => true,
        ("GMountIface", "get_name") => true,
        ("GMountIface", "get_uuid") => true,
        ("GMountOperationClass", "show_processes") => true,
        ("GVolumeIface", "get_name") => true,
        ("GVolumeIface", "get_uuid") => true,
        ("GDriveIface", "get_name") => true,

        // FIXME in gir: void* / uint8_t:
        ("GConverterIface", "convert") => true,
        ("GInputStreamClass", "read_async") => true,
        ("GOutputStreamClass", "write_async") => true,
        ("GOutputStreamClass", "write_fn") => true,
        ("GPollableInputStreamInterface", "read_nonblocking") => true,
        ("GPollableOutputStreamInterface", "write_nonblocking") => true,

        // FIXME in gir: GObject* / void*
        ("GListModelInterface", "get_item") => true,

        // FIXME in gir: * instead of **:
        ("GMountOperationClass", "ask_question") => true,

        // Volatile:
        (_, "ref_count") => true,

        _ => false,
    });

    cfg.field_name(|_typ, field| match field {
        // Unfix rust keywords in field names.
        "move_" => "move",
        "priv_" => "priv",
        "type_" => "type",
        _ => field,
    }.to_string());

    cfg.skip_type(move |typ| match typ {
        t if windows => UNIX_TYPES.contains(&t),
        _ => false,
    });

    cfg.skip_struct(move |typ| match typ {
        t if windows => UNIX_TYPES.contains(&t),

        _ => false,
    });

    cfg.generate("../gio-sys/src/lib.rs", "all.rs");
}

