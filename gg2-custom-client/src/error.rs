use crate::prelude::*;

pub type Result<T> = std::result::Result<T, Error>;

/// All of Gang Garrison's errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Winit error: {0}")]
    WinitEventLoop(#[from] winit::error::EventLoopError),
    #[error("WGPU error: {0}")]
    WgpuRequestAdaptor(#[from] wgpu::RequestAdapterError),
    #[error("WGPU error: {0}")]
    WgpuRequestDevice(#[from] wgpu::RequestDeviceError),
    #[error("WGPU error: {0}")]
    WgpuCreateSurface(#[from] wgpu::CreateSurfaceError),
}
