//! Application domain helpers for working with hot-reloadable application code.

use std::ffi::c_void;

// TODO: experiment with this idea some more?

/// The domain for runtime execution of a project.
///
/// Used to provide descriptors to the editor for use in the UI.
#[derive(Default, Debug)]
pub struct RuntimeDomain {
  pub components: Vec<ComponentDescriptor>,
  pub importers: Vec<ImporterDescriptor>,
}

/// A descriptor for an component in project code.
#[derive(Default, Debug)]
pub struct ComponentDescriptor {}

/// A descriptor for an asset importer in project code.
#[derive(Default, Debug)]
pub struct ImporterDescriptor {}

mod ffi {
  use super::*;

  #[allow(dead_code)]
  extern "C" {
    /// Initializes an [`RuntimeDomain`] in the associated module.
    ///
    /// When implemented by a client assembly, allows the client to register
    /// types for use in reflective processes in the editor.
    pub fn initialize_domain(domain: *mut c_void);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn domain_should_pass_between_instances() {
    let domain = RuntimeDomain::default();

    unsafe {
      ffi::initialize_domain(&domain as *const _ as *mut c_void);
    }

    println!("{domain:?}");
  }

  /// An example of a component that can be hot-reloaded.
  #[no_mangle]
  pub extern "C" fn initialize_domain(domain: *mut c_void) {
    let domain = unsafe { &mut *(domain as *mut RuntimeDomain) };

    domain.components.push(ComponentDescriptor {});
    domain.components.push(ComponentDescriptor {});
    domain.components.push(ComponentDescriptor {});

    domain.importers.push(ImporterDescriptor {});
  }
}
