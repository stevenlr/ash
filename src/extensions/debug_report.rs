use prelude::*;
use std::ptr;
use std::mem;
use instance::Instance;
use entry::Entry;
use vk;


pub struct DebugReport {
    pub handle: vk::Instance,
    pub debug_report_fn: vk::DebugReportFn,
}

impl DebugReport {
    pub fn new(entry: &Entry, instance: &Instance) -> DebugReport {
        let debug_report_fn = vk::DebugReportFn::load(|name| {
                unsafe {
                    mem::transmute(entry.static_fn
                        .get_instance_proc_addr(instance.handle, name.as_ptr()))
                }
            })
            .unwrap();
        DebugReport {
            handle: instance.handle,
            debug_report_fn: debug_report_fn,
        }
    }

    pub fn destroy_debug_report_callback_ext(&self, debug: vk::DebugReportCallbackEXT) {
        unsafe {
            self.debug_report_fn.destroy_debug_report_callback_ext(self.handle, debug, ptr::null());
        }
    }

    pub fn create_debug_report_callback_ext(&self,
                                            create_info: &vk::DebugReportCallbackCreateInfoEXT)
                                            -> VkResult<vk::DebugReportCallbackEXT> {
        unsafe {
            let mut debug_cb = mem::uninitialized();
            let err_code = self.debug_report_fn
                .create_debug_report_callback_ext(self.handle,
                                                  create_info,
                                                  ptr::null(),
                                                  &mut debug_cb);
            match err_code {
                vk::Result::Success => Ok(debug_cb),
                _ => Err(err_code),
            }
        }
    }
}
