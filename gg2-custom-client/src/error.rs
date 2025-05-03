use crate::prelude::*;

pub type Result<T> = std::result::Result<T, Error>;

/// All of Gang Garrison's errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Winit Error: {0}")]
    WinitEventLoop(#[from] winit::error::EventLoopError),
    #[error("WGPU Error: {0}")]
    WgpuRequestAdaptor(#[from] wgpu::RequestAdapterError),
    #[error("WGPU Error: {0}")]
    WgpuRequestDevice(#[from] wgpu::RequestDeviceError),
    #[error("WGPU Error: {0}")]
    WgpuCreateSurface(#[from] wgpu::CreateSurfaceError),
    #[error("Network Error: {0}")]
    Network(#[from] gg2_common::networking::error::Error),
}
