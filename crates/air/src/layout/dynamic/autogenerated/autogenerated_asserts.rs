use crate::{domains::StarkDomains, dynamic::DynamicParams, layout::CheckAssertsError};

pub fn is_power_of_2(x: usize) -> Result<(), CheckAssertsError> {
    if x != 0 && (x & (x - 1)) == 0 {
        Ok(())
    } else {
        Err(CheckAssertsError::ValueNotPowerOfTwo)
    }
}

pub fn check_asserts(
    _dynamic_params: &DynamicParams,
    stark_domains: &StarkDomains,
) -> Result<(), CheckAssertsError> {
    let _log_trace_length = stark_domains.log_trace_domain_size;
    let _trace_length = stark_domains.trace_domain_size;

    Ok(())
}
