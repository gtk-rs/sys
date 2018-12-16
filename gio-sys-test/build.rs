extern crate ctest;
extern crate shell_words;

fn pkg_config_cflags() -> Result<Vec<String>, Box<std::error::Error>> {
    let mut cmd = std::process::Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.arg("gio-2.0");
    cmd.arg("gio-unix-2.0");
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

    cfg.header("gio/gio.h");
    cfg.define("G_SETTINGS_ENABLE_BACKEND", None);
    cfg.header("gio/gsettingsbackend.h");
    cfg.header("gio/gdesktopappinfo.h");
    cfg.header("gio/gfiledescriptorbased.h");
    cfg.header("gio/gunixconnection.h");
    cfg.header("gio/gunixcredentialsmessage.h");
    cfg.header("gio/gunixfdmessage.h");
    cfg.header("gio/gunixinputstream.h");
    cfg.header("gio/gunixoutputstream.h");
    cfg.header("gio/gunixsocketaddress.h");

    cfg.skip_const(|_| true);
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

    cfg.skip_struct(|typ| match typ {
        // Incomplete types, need manual exclusion since ctype
        // doesn't understand "pub struct Type(c_void)" pattern:
        "GAction" => true,
        "GActionGroup" => true,
        "GActionMap" => true,
        "GAppInfo" => true,
        "GAppInfoMonitor" => true,
        "GAppLaunchContextPrivate" => true,
        "GApplicationCommandLinePrivate" => true,
        "GApplicationPrivate" => true,
        "GAsyncInitable" => true,
        "GAsyncResult" => true,
        "GBufferedInputStreamPrivate" => true,
        "GBufferedOutputStreamPrivate" => true,
        "GBytesIcon" => true,
        "GCancellablePrivate" => true,
        "GCharsetConverter" => true,
        "GConverter" => true,
        "GConverterInputStreamPrivate" => true,
        "GConverterOutputStreamPrivate" => true,
        "GCredentials" => true,
        "GCredentialsClass" => true,
        "GDBusActionGroup" => true,
        "GDBusAuthObserver" => true,
        "GDBusConnection" => true,
        "GDBusInterface" => true,
        "GDBusInterfaceSkeletonPrivate" => true,
        "GDBusMenuModel" => true,
        "GDBusMessage" => true,
        "GDBusMethodInvocation" => true,
        "GDBusObject" => true,
        "GDBusObjectManager" => true,
        "GDBusObjectManagerClientPrivate" => true,
        "GDBusObjectManagerServerPrivate" => true,
        "GDBusObjectProxyPrivate" => true,
        "GDBusObjectSkeletonPrivate" => true,
        "GDBusProxyPrivate" => true,
        "GDBusServer" => true,
        "GDataInputStreamPrivate" => true,
        "GDataOutputStreamPrivate" => true,
        "GDatagramBased" => true,
        "GDesktopAppInfo" => true,
        "GDesktopAppInfoLookup" => true,
        "GDrive" => true,
        "GDtlsClientConnection" => true,
        "GDtlsConnection" => true,
        "GDtlsServerConnection" => true,
        "GEmblem" => true,
        "GEmblemClass" => true,
        "GEmblemedIconPrivate" => true,
        "GFile" => true,
        "GFileAttributeMatcher" => true,
        "GFileDescriptorBased" => true,
        "GFileEnumeratorPrivate" => true,
        "GFileIOStreamPrivate" => true,
        "GFileIcon" => true,
        "GFileIconClass" => true,
        "GFileInfo" => true,
        "GFileInfoClass" => true,
        "GFileInputStreamPrivate" => true,
        "GFileMonitorPrivate" => true,
        "GFileOutputStreamPrivate" => true,
        "GFilenameCompleter" => true,
        "GIOExtension" => true,
        "GIOExtensionPoint" => true,
        "GIOModule" => true,
        "GIOModuleClass" => true,
        "GIOModuleScope" => true,
        "GIOSchedulerJob" => true,
        "GIOStreamAdapter" => true,
        "GIOStreamPrivate" => true,
        "GIcon" => true,
        "GInetAddressMaskPrivate" => true,
        "GInetAddressPrivate" => true,
        "GInetSocketAddressPrivate" => true,
        "GInitable" => true,
        "GInputStreamPrivate" => true,
        "GListModel" => true,
        "GListStore" => true,
        "GLoadableIcon" => true,
        "GMemoryInputStreamPrivate" => true,
        "GMemoryOutputStreamPrivate" => true,
        "GMenu" => true,
        "GMenuAttributeIterPrivate" => true,
        "GMenuItem" => true,
        "GMenuLinkIterPrivate" => true,
        "GMenuModelPrivate" => true,
        "GMount" => true,
        "GMountOperationPrivate" => true,
        "GNativeSocketAddress" => true,
        "GNetworkAddressPrivate" => true,
        "GNetworkMonitor" => true,
        "GNetworkServicePrivate" => true,
        "GNotification" => true,
        "GOutputStreamPrivate" => true,
        "GPermissionPrivate" => true,
        "GPollableInputStream" => true,
        "GPollableOutputStream" => true,
        "GPropertyAction" => true,
        "GProxy" => true,
        "GProxyAddressEnumeratorPrivate" => true,
        "GProxyAddressPrivate" => true,
        "GProxyResolver" => true,
        "GRemoteActionGroup" => true,
        "GResolverPrivate" => true,
        "GResource" => true,
        "GSeekable" => true,
        "GSettingsBackendPrivate" => true,
        "GSettingsPrivate" => true,
        "GSettingsSchema" => true,
        "GSettingsSchemaKey" => true,
        "GSettingsSchemaSource" => true,
        "GSimpleAction" => true,
        "GSimpleActionGroupPrivate" => true,
        "GSimpleAsyncResult" => true,
        "GSimpleAsyncResultClass" => true,
        "GSimpleIOStream" => true,
        "GSimplePermission" => true,
        "GSimpleProxyResolverPrivate" => true,
        "GSocketClientPrivate" => true,
        "GSocketConnectable" => true,
        "GSocketConnectionPrivate" => true,
        "GSocketControlMessagePrivate" => true,
        "GSocketListenerPrivate" => true,
        "GSocketPrivate" => true,
        "GSocketServicePrivate" => true,
        "GSrvTarget" => true,
        "GSubprocess" => true,
        "GSubprocessLauncher" => true,
        "GTask" => true,
        "GTaskClass" => true,
        "GTcpConnectionPrivate" => true,
        "GTcpWrapperConnectionPrivate" => true,
        "GTestDBus" => true,
        "GThemedIcon" => true,
        "GThemedIconClass" => true,
        "GThreadedSocketServicePrivate" => true,
        "GTlsBackend" => true,
        "GTlsCertificatePrivate" => true,
        "GTlsClientConnection" => true,
        "GTlsConnectionPrivate" => true,
        "GTlsDatabasePrivate" => true,
        "GTlsFileDatabase" => true,
        "GTlsInteractionPrivate" => true,
        "GTlsPasswordPrivate" => true,
        "GTlsServerConnection" => true,
        "GUnixConnectionPrivate" => true,
        "GUnixCredentialsMessagePrivate" => true,
        "GUnixFDListPrivate" => true,
        "GUnixFDMessagePrivate" => true,
        "GUnixInputStreamPrivate" => true,
        "GUnixMountEntry" => true,
        "GUnixMountMonitor" => true,
        "GUnixMountMonitorClass" => true,
        "GUnixMountPoint" => true,
        "GUnixOutputStreamPrivate" => true,
        "GUnixSocketAddressPrivate" => true,
        "GVolume" => true,
        "GZlibCompressor" => true,
        "GZlibDecompressor" => true,

        _ => false,
    });

    cfg.generate("../gio-sys/src/lib.rs", "all.rs");
}

