use std::env;

/// Describes the operating system and architecture targeted by this binary.
///
/// Values come from [`std::env::consts`] and are determined at compile time.
/// They do not identify the OS version, Linux distribution, or CPU model.
///
/// # Examples
///
/// ```
/// use kopos::api::PlatformInfo;
///
/// let info = PlatformInfo::current();
/// println!("Operating system: {}", info.os());
/// println!("Platform: {info}");
/// ```
pub struct PlatformInfo {
    os: &'static str,
    arch: &'static str,
    family: &'static str,
}

impl PlatformInfo {
    /// Returns platform information for this binary's compilation target.
    ///
    /// This reads static constants; it performs no runtime OS detection or I/O.
    pub fn current() -> Self {
        let os: &str = env::consts::OS;
        let arch: &str = env::consts::ARCH;
        let family: &str = env::consts::FAMILY;
        Self { os, arch, family }
    }

    /// Returns the target OS identifier, such as `"linux"`, `"windows"`, or `"macos"`.
    pub fn os(&self) -> &'static str {
        self.os
    }
    /// Returns the target CPU architecture, such as `"x86_64"` or `"aarch64"`.
    pub fn arch(&self) -> &'static str {
        self.arch
    }
    /// Returns the target OS family, such as `"unix"` or `"windows"`.
    ///
    /// This classification does not establish support for particular host APIs.
    pub fn family(&self) -> &'static str {
        self.family
    }
}

impl std::fmt::Display for PlatformInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} / {} ({})", self.os, self.arch, self.family)
    }
}
