use crate::{
    ffi::{self, prelude::*},
    DDSMetaData, HResultError, Image, TexMetadata, CP_FLAGS, DDS_FLAGS, DXGI_FORMAT,
};
use core::ptr;

type Result<T> = core::result::Result<T, HResultError>;

#[derive(Debug)]
#[repr(C)]
pub struct ScratchImage {
    m_nimages: usize,
    m_size: usize,
    m_metadata: TexMetadata,
    m_image: *mut Image,
    m_memory: *mut u8,
}

impl Default for ScratchImage {
    fn default() -> Self {
        Self {
            m_nimages: 0,
            m_size: 0,
            m_metadata: TexMetadata::default(),
            m_image: ptr::null_mut(),
            m_memory: ptr::null_mut(),
        }
    }
}

impl Drop for ScratchImage {
    fn drop(&mut self) {
        self.release();
    }
}

impl ScratchImage {
    pub fn initialize(&mut self, mdata: &TexMetadata, flags: CP_FLAGS) -> Result<()> {
        let result =
            unsafe { ffi::DirectXTexFFI_ScratchImage_Initialize(self.into(), mdata.into(), flags) };
        result.success()
    }

    pub fn initialize_1d(
        &mut self,
        fmt: DXGI_FORMAT,
        length: usize,
        array_size: usize,
        mip_levels: usize,
        flags: CP_FLAGS,
    ) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_Initialize1D(
                self.into(),
                fmt,
                length,
                array_size,
                mip_levels,
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_2d(
        &mut self,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        array_size: usize,
        mip_levels: usize,
        flags: CP_FLAGS,
    ) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_Initialize2D(
                self.into(),
                fmt,
                width,
                height,
                array_size,
                mip_levels,
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_3d(
        &mut self,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        depth: usize,
        mip_levels: usize,
        flags: CP_FLAGS,
    ) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_Initialize3D(
                self.into(),
                fmt,
                width,
                height,
                depth,
                mip_levels,
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_cube(
        &mut self,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        cubes: usize,
        mip_levels: usize,
        flags: CP_FLAGS,
    ) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_InitializeCube(
                self.into(),
                fmt,
                width,
                height,
                cubes,
                mip_levels,
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_from_image(
        &mut self,
        src_image: &Image,
        allow_1d: bool,
        flags: CP_FLAGS,
    ) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_InitializeFromImage(
                self.into(),
                src_image.into(),
                allow_1d,
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_array_from_images(
        &mut self,
        images: &[Image],
        allow_1d: bool,
        flags: CP_FLAGS,
    ) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_InitializeArrayFromImages(
                self.into(),
                images.as_ffi_ptr(),
                images.len(),
                allow_1d,
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_cube_from_images(&mut self, images: &[Image], flags: CP_FLAGS) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_InitializeCubeFromImages(
                self.into(),
                images.as_ffi_ptr(),
                images.len(),
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_3d_from_images(&mut self, images: &[Image], flags: CP_FLAGS) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_Initialize3DFromImages(
                self.into(),
                images.as_ffi_ptr(),
                images.len(),
                flags,
            )
        };
        result.success()
    }

    pub fn release(&mut self) {
        unsafe { ffi::DirectXTexFFI_ScratchImage_Release(self.into()) }
    }

    #[must_use]
    pub fn override_format(&mut self, f: DXGI_FORMAT) -> Option<()> {
        let result = unsafe { ffi::DirectXTexFFI_ScratchImage_OverrideFormat(self.into(), f) };
        result.then_some(())
    }

    #[must_use]
    pub fn get_metadata(&self) -> &TexMetadata {
        &self.m_metadata
    }

    #[must_use]
    pub fn get_image(&self, mip: usize, item: usize, slice: usize) -> Option<&Image> {
        let result =
            unsafe { ffi::DirectXTexFFI_ScratchImage_GetImage(self.into(), mip, item, slice) };
        unsafe { result.as_ref() }
    }

    #[must_use]
    pub fn get_images(&self) -> &[Image] {
        unsafe { ffi::from_raw_ffi_parts(self.m_image, self.m_nimages) }
    }

    #[must_use]
    pub fn get_pixels(&self) -> &[u8] {
        unsafe { ffi::from_raw_ffi_parts(self.m_memory, self.m_size) }
    }

    #[must_use]
    pub fn get_pixels_mut(&mut self) -> &mut [u8] {
        unsafe { ffi::from_raw_ffi_parts_mut(self.m_memory, self.m_size) }
    }

    #[must_use]
    pub fn is_alpha_all_opaque(&self) -> bool {
        unsafe { ffi::DirectXTexFFI_ScratchImage_IsAlphaAllOpaque(self.into()) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ffi, ScratchImage};
    use core::mem;

    #[test]
    fn verify_layout() {
        assert_eq!(mem::size_of::<ScratchImage>(), unsafe {
            ffi::DirectXTexFFI_ScratchImage_Sizeof()
        });
        assert_eq!(mem::align_of::<ScratchImage>(), unsafe {
            ffi::DirectXTexFFI_ScratchImage_Alignof()
        });
    }
}
