//! The `tinker-vad` library provides Voice Activity Detection (VAD) functionality.

use candle_core::{
  utils::{cuda_is_available, metal_is_available},
  Device,
};

pub mod vad;
pub use vad::*;

/// Selects and returns an available `Device`.
/// If `force_cpu` is true, it will always return a CPU device.
/// Otherwise, it will attempt to return a CUDA or Metal device if available,
/// falling back to a CPU device if neither is available.
pub fn select_device(force_cpu: bool) -> Result<Device> {
  if force_cpu {
    Ok(Device::Cpu)
  } else if cuda_is_available() {
    Ok(Device::new_cuda(0)?)
  } else if metal_is_available() {
    Ok(Device::new_metal(0)?)
  } else {
    Ok(Device::Cpu)
  }
}
