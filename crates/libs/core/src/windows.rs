#[cfg(not(feature = "kernel"))]
mod agile_reference;
#[cfg(not(feature = "kernel"))]
pub use agile_reference::*;

#[cfg(not(feature = "kernel"))]
mod array;
#[cfg(not(feature = "kernel"))]
pub use array::*;

#[cfg(all(feature = "std", not(feature = "kernel")))]
mod event;
#[cfg(all(feature = "std", not(feature = "kernel")))]
pub use event::*;

mod resources;
pub use resources::*;

#[cfg(not(feature = "kernel"))]
pub use windows_strings::*;

/// Attempts to load the factory object for the given WinRT class.
/// This can be used to access COM interfaces implemented on a Windows Runtime class factory.
#[cfg(not(feature = "kernel"))]
pub fn factory<C: RuntimeName, I: Interface>() -> Result<I> {
    imp::load_factory::<C, I>()
}

#[cfg(not(feature = "kernel"))]
impl Param<PCWSTR> for &BSTR {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.as_ptr()))
    }
}

#[cfg(not(feature = "kernel"))]
impl Param<PCWSTR> for &HSTRING {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.as_ptr()))
    }
}

#[cfg(not(feature = "kernel"))]
impl Param<PCWSTR> for PWSTR {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.0))
    }
}

#[cfg(not(feature = "kernel"))]
impl Param<PCSTR> for PSTR {
    unsafe fn param(self) -> ParamValue<PCSTR> {
        ParamValue::Owned(PCSTR(self.0))
    }
}

#[cfg(not(feature = "kernel"))]
impl RuntimeType for HSTRING {
    const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"string");
}

#[cfg(not(feature = "kernel"))]
impl TypeKind for PWSTR {
    type TypeKind = CopyType;
}

#[cfg(not(feature = "kernel"))]
impl TypeKind for PSTR {
    type TypeKind = CopyType;
}

#[cfg(not(feature = "kernel"))]
impl TypeKind for PCWSTR {
    type TypeKind = CopyType;
}

#[cfg(not(feature = "kernel"))]
impl TypeKind for PCSTR {
    type TypeKind = CopyType;
}

#[cfg(not(feature = "kernel"))]
impl TypeKind for HSTRING {
    type TypeKind = CloneType;
}

#[cfg(not(feature = "kernel"))]
impl TypeKind for BSTR {
    type TypeKind = CloneType;
}
