#[repr(C)]
pub struct GPUInfo {
    pub gpu_usage: std::ffi::c_double,
    pub gpu_mem_total: std::ffi::c_int,
    pub gpu_mem_used: std::ffi::c_int,
}

impl GPUInfo {
    pub fn from_vec(vec: Vec<&str>) -> Self {
        Self {
            gpu_usage: vec[0].parse::<f64>().unwrap() / 100.0,
            gpu_mem_total: vec[1].parse::<i32>().unwrap(),
            gpu_mem_used: vec[2].parse::<i32>().unwrap(),
        }
    }
}

impl Default for GPUInfo {
    fn default() -> Self {
        Self {
            gpu_usage: 0.0,
            gpu_mem_total: 0,
            gpu_mem_used: 0,
        }
    }
}

enum GPUVendor {
    Nvidia,
    AMD,
}

/// GPU_VENDOR is used to cache the GPU vendor.
/// It is used to avoid calling get_gpu_vendor() multiple times.
static mut GPU_VENDOR: Option<GPUVendor> = None;

fn get_gpu_vendor() -> GPUVendor {
    // TODO: evaluate if the GPU is Nvidia or AMD.
    return GPUVendor::Nvidia;
}

fn nvidia_gpu_info() -> Result<GPUInfo, std::io::Error> {
    let data = std::process::Command::new("nvidia-smi")
        .arg("--query-gpu=utilization.gpu,memory.total,memory.used")
        .arg("--format=csv,noheader,nounits")
        .output();

    match data {
        Ok(data) => {
            let string = data
                .stdout
                .iter()
                .map(|&x| x as char)
                .collect::<String>()
                .trim()
                .to_string()
                .to_owned()
                .clone();

            let info = string.split(", ").collect::<Vec<&str>>();

            return Ok(GPUInfo::from_vec(info));
        }
        Err(err) => {
            return Err(err);
        }
    }
}

/// Get GPU info returns a GPUInfo struct what contains the GPU usage, total memory and used memory.
/// If the GPU is not found, it returns a default GPUInfo struct.
///
/// For now, only Nvidia GPUs are supported. If you want to add support for other GPUs, please open a PR.
#[no_mangle]
pub extern "C" fn get_gpu_info() -> GPUInfo {
    unsafe {
        if GPU_VENDOR.is_none() {
            // Avoid calling get_gpu_vendor() multiple times.
            GPU_VENDOR = Some(get_gpu_vendor());
        }

        match GPU_VENDOR {
            Some(GPUVendor::Nvidia) => {
                if let Ok(gpu_info) = nvidia_gpu_info() {
                    return gpu_info;
                } else {
                    return GPUInfo::default();
                }
            }
            _ => {
                return GPUInfo::default();
            }
        }
    }
}
