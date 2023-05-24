use crate::{error::Error, Enclave, EnclaveResult};

use frame_support::ensure;
use itp_enclave_api_ffi as ffi;
use sgx_types::*;

pub trait OAuth: Send + Sync + 'static {
	fn start_oauth(
		&self
	) -> EnclaveResult<()>;
}

impl OAuth for Enclave {
	fn start_oauth(
		&self
	) -> EnclaveResult<()> {
		let mut retval = sgx_status_t::SGX_SUCCESS;
		let result = unsafe {
			ffi::start_oauth(
				self.eid,
				&mut retval,
			)
		};
		match result {
			sgx_status_t::SGX_SUCCESS => {},
			_ => {
				println!("[-] ECALL Enclave Failes {}!", result.as_str());
			}
		}
		ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
        ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));
		Ok(())
	}
}
