use crate::prelude::*;

/// All of Gang Garrison's errors
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Winit Error: {0}")]
    WinitEventLoop(#[from] winit::error::EventLoopError),
    #[error("WGPU Error: {0}")]
    WgpuRequestAdaptor(#[from] wgpu::RequestAdapterError),
    #[error("WGPU Error: {0}")]
    WgpuRequestDevice(#[from] wgpu::RequestDeviceError),
    #[error("WGPU Error: {0}")]
    WgpuCreateSurface(#[from] wgpu::CreateSurfaceError),
    #[error("Network Error: {0}")]
    Network(#[from] NetworkError),
    #[error("Asset Error: {0}")]
    Asset(#[from] AssetError),
    #[error("Common Error: {0}")]
    Common(#[from] CommonError),
    #[error("Client player unset")]
    ClientPlayerLookup,
}
