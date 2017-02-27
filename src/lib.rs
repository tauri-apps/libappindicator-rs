extern crate libappindicator_sys;
extern crate glib;
extern crate gtk;
extern crate gtk_sys;

pub use libappindicator_sys::*;
use libappindicator_sys::{AppIndicator as AppIndicatorRaw};
use glib::translate::{ToGlibPtr};
use std::ffi::{CString, NulError};

pub struct AppIndicator {
    air: *mut AppIndicatorRaw
}

impl AppIndicator {
    pub fn new(title: &str, icon: &str) -> Result<AppIndicator, NulError> {
        let ai;
        let title = match CString::new(title) {
            Ok(t) => t,
            Err(e) => return Err(e)
        };
        let icon = match CString::new(icon) {
            Ok(i) => i,
            Err(e) => return Err(e)
        };
        unsafe {
            ai = app_indicator_new(title.as_ptr(),
                                   icon.as_ptr(),
                                   AppIndicatorCategory::APP_INDICATOR_CATEGORY_APPLICATION_STATUS);
        }
        Ok(AppIndicator {
            air: ai
        })
    }

    pub fn set_status(&mut self, status: AppIndicatorStatus) {
        unsafe {
            app_indicator_set_status(self.air, status);
        }
    }

    pub fn set_menu(&mut self, menu: &mut gtk::Menu) {
        let mp = menu.to_glib_none();
        unsafe {
            app_indicator_set_menu(self.air, mp.0);
        }
    }

    pub fn set_label(&mut self, label: &str, guide: &str) -> Result<(), NulError> {
        let label = match CString::new(label) {
            Ok(t) => t,
            Err(e) => return Err(e)
        };
        let guide = match CString::new(guide) {
            Ok(i) => i,
            Err(e) => return Err(e)
        };
        unsafe {
            app_indicator_set_label(self.air, label.as_ptr(), guide.as_ptr());
        }
        Ok(())
    }

    pub fn set_icon_full(&mut self, name: &str, desc: &str) -> Result<(), NulError> {
        let name = match CString::new(name) {
            Ok(t) => t,
            Err(e) => return Err(e)
        };
        let desc = match CString::new(desc) {
            Ok(i) => i,
            Err(e) => return Err(e)
        };
        unsafe {
            app_indicator_set_icon_full(self.air, name.as_ptr(), desc.as_ptr());
        }
        Ok(())
    }

    pub fn set_attention_icon_full(&mut self, name: &str, desc: &str) -> Result<(), NulError>  {
        let name = match CString::new(name) {
            Ok(t) => t,
            Err(e) => return Err(e)
        };
        let desc = match CString::new(desc) {
            Ok(i) => i,
            Err(e) => return Err(e)
        };
        unsafe {
            app_indicator_set_attention_icon_full(self.air, name.as_ptr(), desc.as_ptr());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
