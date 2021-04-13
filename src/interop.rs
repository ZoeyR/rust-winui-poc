use bindings::Windows::UI::Xaml::Hosting::DesktopWindowXamlSource;
use windows::RawPtr;

#[link(name = "windowsapp")]
extern "stdcall" {
    fn RoInitialize(init_type: RoInitType) -> windows::HRESULT;
}
#[allow(dead_code)]
#[repr(i32)]
pub enum RoInitType {
    MultiThreaded = 0,
    SingleThreaded = 1,
}

pub fn ro_initialize(init_type: RoInitType) -> windows::Result<()> {
    unsafe { RoInitialize(init_type).ok() }
}

#[repr(C)]
pub struct abi_IDesktopWindowXamlSourceNative {
    __base: [usize; 3],
    attach_to_window: extern "system" fn(
        RawPtr,
        RawPtr, //HWND
    ) -> windows::HRESULT,
    get_window_handle: extern "system" fn(
        RawPtr,
        *mut RawPtr, //HWND
    ) -> windows::HRESULT,
}

unsafe impl windows::Interface for IDesktopWindowXamlSourceNative {
    type Vtable = abi_IDesktopWindowXamlSourceNative;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        1019015615,
        12150,
        20124,
        [150, 171, 232, 75, 55, 151, 37, 84],
    );
}

#[repr(transparent)]
pub struct IDesktopWindowXamlSourceNative {
    ptr: windows::IUnknown,
}

impl IDesktopWindowXamlSourceNative {
    pub fn attach_to_window(&self, hwnd: RawPtr) -> windows::Result<()> {
        use windows::Abi;
        let this = self;
        #[allow(unused_unsafe)]
        unsafe {
            (::windows::Interface::vtable(this).attach_to_window)(this.abi(), hwnd).ok()
        }
    }

    pub fn get_window_handle(&self) -> windows::Result<RawPtr> {
        use windows::Abi;

        let this = self;
        #[allow(unused_unsafe)]
        unsafe {
            let mut result = std::ptr::null_mut();
            (::windows::Interface::vtable(this).get_window_handle)(this.abi(), &mut result)
                .and_then(|| result)
        }
    }
}

impl std::clone::Clone for IDesktopWindowXamlSourceNative {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr.clone(),
        }
    }
}

impl From<&DesktopWindowXamlSource> for IDesktopWindowXamlSourceNative {
    fn from(value: &DesktopWindowXamlSource) -> IDesktopWindowXamlSourceNative {
        <DesktopWindowXamlSource as ::windows::Interface>::cast(value).unwrap()
    }
}

impl From<DesktopWindowXamlSource> for IDesktopWindowXamlSourceNative {
    fn from(value: DesktopWindowXamlSource) -> IDesktopWindowXamlSourceNative {
        std::convert::From::from(&value)
    }
}
