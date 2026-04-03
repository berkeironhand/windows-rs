use windows_rdl::*;

/// Verify that un-nested (anonymous) inner types inherit the
/// `SupportedArchitecture` attribute from their enclosing struct or union.
#[test]
pub fn nested_arches() {
    writer()
        .input("../../../libs/bindgen/default/Windows.Win32.winmd")
        .output("tests/nested-arches.rdl")
        .filter("Windows.Win32.System.Kernel.SLIST_HEADER")
        .write()
        .unwrap();
}

/// Verify that un-nested (anonymous) inner types inherit the `packed` attribute
/// from their enclosing struct or union.
#[test]
pub fn nested_packing() {
    writer()
        .input("../../../libs/bindgen/default/Windows.Win32.winmd")
        .output("tests/nested-packing.rdl")
        .filter("Windows.Win32.Devices.Bluetooth.BTH_INFO_RSP")
        .write()
        .unwrap();
}
