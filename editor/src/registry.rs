use winreg::{
  enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
  types::{FromRegValue, ToRegValue},
  RegValue, HKEY,
};

/// A value that can be persisted in the registry.
pub trait RegistryValue: Sized + Clone {
  /// Reads a value of this type from the registry.
  fn from_registry(key: &RegValue) -> surreal::Result<Self>;

  /// Converts a value of this type for use in the registry.
  fn to_registry(&self) -> surreal::Result<RegValue>;
}

/// A reference to a key in the registry that can be read/written.
pub struct RegistryKey<T: RegistryValue> {
  hkey: HKEY,
  path: &'static str,
  default_value: T,
}

impl<T: RegistryValue> RegistryKey<T> {
  /// Creates a [`RegistryKey`] for the local user.
  pub const fn current_user(path: &'static str, default_value: T) -> Self {
    Self::new(HKEY_CURRENT_USER, path, default_value)
  }

  /// Creates a [`RegistryKey`] for the local computer.
  pub const fn local_machine(path: &'static str, default_value: T) -> Self {
    Self::new(HKEY_LOCAL_MACHINE, path, default_value)
  }

  /// Creates a new [`RegistryKey`] from the given raw values.
  pub const fn new(hkey: HKEY, path: &'static str, default_value: T) -> Self {
    Self { hkey, path, default_value }
  }

  /// Reads a [`RegistryValue`] for the key, or returns the default value if it doesn't exist
  pub fn read_or_default(&self) -> T {
    self.read().unwrap_or(self.default_value.clone())
  }

  /// Reads a [`RegistryValue`] for this key.
  pub fn read(&self) -> surreal::Result<T> {
    let (key, value) = self
      .path
      .rsplit_once('\\')
      .ok_or(surreal::anyhow!("An invalid registry path was specified: {}", self.path))?;

    let key = winreg::RegKey::predef(self.hkey).open_subkey(key)?;
    let value = key.get_raw_value(value)?;

    Ok(T::from_registry(&value)?)
  }

  /// Writes the [`RegistryValue`] for this key, or panics.
  pub fn write_or_fail(&self, value: T) {
    self.write(value).expect(&format!("Failed to write value for {}", self.path));
  }

  /// Writes the [`RegistryValue`] for this key.
  pub fn write(&self, value: T) -> surreal::Result<()> {
    let (key, value_name) = self
      .path
      .rsplit_once('\\')
      .ok_or(surreal::anyhow!("An invalid registry path was specified: {}", self.path))?;

    let (key, _) = winreg::RegKey::predef(self.hkey).create_subkey(key)?;

    key.set_raw_value(value_name, &value.to_registry()?)?;

    Ok(())
  }
}

/// Implements rules for registry value conversion for simple numeric types
macro_rules! impl_registry_value_numeric {
  ($type:ty, $nearest:ty) => {
    impl RegistryValue for $type {
      fn from_registry(key: &RegValue) -> surreal::Result<Self> {
        Ok(<$nearest>::from_reg_value(key)? as $type)
      }

      fn to_registry(&self) -> surreal::Result<RegValue> {
        Ok(<$nearest>::to_reg_value(&(*self as $nearest)))
      }
    }
  };
}

impl_registry_value_numeric!(u8, u32);
impl_registry_value_numeric!(u16, u32);
impl_registry_value_numeric!(u32, u32);
impl_registry_value_numeric!(u64, u64);

impl RegistryValue for bool {
  fn from_registry(key: &RegValue) -> surreal::Result<Self> {
    u8::from_registry(key).map(|v| v != 0)
  }

  fn to_registry(&self) -> surreal::Result<RegValue> {
    match self {
      true => 1u8.to_registry(),
      false => 0u8.to_registry(),
    }
  }
}

impl RegistryValue for String {
  fn from_registry(key: &RegValue) -> surreal::Result<Self> {
    Ok(String::from_reg_value(key)?)
  }

  fn to_registry(&self) -> surreal::Result<RegValue> {
    Ok(String::to_reg_value(self))
  }
}

impl RegistryValue for Vec<u8> {
  fn from_registry(key: &RegValue) -> surreal::Result<Self> {
    if key.vtype != winreg::enums::RegType::REG_BINARY {
      return Err(surreal::anyhow!("Invalid registry value type: {:?}", key.vtype));
    }

    Ok(key.bytes.clone())
  }

  fn to_registry(&self) -> surreal::Result<RegValue> {
    Ok(RegValue {
      vtype: winreg::enums::RegType::REG_BINARY,
      bytes: self.clone(),
    })
  }
}
