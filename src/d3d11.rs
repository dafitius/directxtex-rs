use crate::ffi::MutNonNull;
use crate::{HResultError, Result};
use core::{marker::PhantomData, ptr::NonNull};
use windows::core::Interface;
use windows::Win32::Foundation::HMODULE;
#[cfg(all(windows, feature = "d3d11"))]
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDevice,
    ID3D11Device,
    D3D11_CREATE_DEVICE_FLAG,
    D3D11_CREATE_DEVICE_DEBUG,
    D3D11_SDK_VERSION,
};

#[cfg(all(windows, feature = "d3d11"))]
use windows::Win32::Graphics::Direct3D::{
    D3D_DRIVER_TYPE_HARDWARE,
    D3D_FEATURE_LEVEL,
    D3D_FEATURE_LEVEL_10_0,
    D3D_FEATURE_LEVEL_10_1,
    D3D_FEATURE_LEVEL_11_0,
};

pub struct D3D11Device {
    #[cfg(all(windows, feature = "d3d11"))]
    device: Option<ID3D11Device>,
    #[cfg(not(all(windows, feature = "d3d11")))]
    _pd: PhantomData<()>,
}

impl D3D11Device {
    pub const fn is_available() -> bool {
        cfg!(all(windows, feature = "d3d11"))
    }

    pub const fn is_initialized(&self) -> bool {
        #[cfg(all(windows, feature = "d3d11"))]
        {
            self.device.is_some()
        }

        #[cfg(not(all(windows, feature = "d3d11")))]
        {
            false
        }
    }

    #[cfg(all(windows, feature = "d3d11"))]
    pub fn is_healthy(&self) -> bool {
        if let Some(dev) = &self.device {
            unsafe { dev.GetDeviceRemovedReason().is_ok() }
        } else {
            false
        }
    }

    pub(crate) fn as_ffi_ptr(&mut self) -> Option<MutNonNull<windows::Win32::Graphics::Direct3D11::ID3D11Device>> {
        #[cfg(all(windows, feature = "d3d11"))]
        return self.device.as_mut().map(|dev| { dev.into() });
        #[cfg(not(all(windows, feature = "d3d11")))]
        return None;
    }

    #[cfg(all(windows, feature = "d3d11"))]
    pub fn new() -> Result<Self> {
        let mut device = std::mem::MaybeUninit::zeroed();

        let feature_levels: [D3D_FEATURE_LEVEL; 3] = [
            D3D_FEATURE_LEVEL_11_0,
            D3D_FEATURE_LEVEL_10_1,
            D3D_FEATURE_LEVEL_10_0,
        ];

        let mut flags = D3D11_CREATE_DEVICE_FLAG::default();
        #[cfg(debug_assertions)]
        {
            flags |= D3D11_CREATE_DEVICE_DEBUG;
        }

        let device = unsafe {
            windows::Win32::Graphics::Direct3D11::D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                HMODULE::default(),
                flags,
                Some(&feature_levels),
                D3D11_SDK_VERSION,
                Some(device.as_mut_ptr()),
                None,
                None,
            ).map_err(|e| HResultError::from(e.code()))?;
            device.assume_init()
        };

        Ok(Self { device })
    }

    #[cfg(not(all(windows, feature = "d3d11")))]
    pub fn new() -> Self {
        Self { _pd: PhantomData }
    }

}

#[cfg(test)]
mod tests {
    use crate::d3d11::{D3D11Device};

    #[test]
    fn verify_d3d11_device() {
        assert!(D3D11Device::is_available());
        let d3d11_device = D3D11Device::new().unwrap();
        assert!(d3d11_device.is_initialized());
        assert!(d3d11_device.is_healthy());
    }
}
