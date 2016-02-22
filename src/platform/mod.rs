//! This module reexports the OS-specific module that actually implements Platform.
pub mod common;
pub use self::common::*;

#[cfg(target_os = "freebsd")]
pub mod freebsd;
#[cfg(target_os = "freebsd")]
pub use self::freebsd::PlatformImpl;

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_cpu_load() {
        let load = PlatformImpl::new().cpu_load().unwrap();
        thread::sleep(Duration::from_millis(300));
        let load = load.done().unwrap();
        assert!(load.len() >= 1);
        for cpu in load.iter() {
            let sum = cpu.user + cpu.nice + cpu.system + cpu.interrupt + cpu.idle;
            assert!(sum > 0.95 && sum < 1.05);
        }
    }

    #[test]
    fn test_cpu_load_aggregate() {
        let cpu = PlatformImpl::new().cpu_load_aggregate().unwrap();
        thread::sleep(Duration::from_millis(300));
        let cpu = cpu.done().unwrap();
        let sum = cpu.user + cpu.nice + cpu.system + cpu.interrupt + cpu.idle;
        assert!(sum > 0.95 && sum < 1.05);
    }

    #[test]
    fn test_load_average() {
        let load = PlatformImpl::new().load_average().unwrap();
        assert!(load.one > 0.00001 && load.five > 0.00001 && load.fifteen > 0.00001);
    }

    #[test]
    fn test_memory() {
        let mem = PlatformImpl::new().memory().unwrap();
        assert!(mem.free_kb > 100 && mem.active_kb > 100);
    }

    #[test]
    fn test_mounts() {
        let mounts = PlatformImpl::new().mounts().unwrap();
        assert!(mounts.len() > 0);
        assert!(mounts.iter().find(|m| m.fs_mounted_on == "/").unwrap().fs_mounted_on == "/");
    }

    #[test]
    fn test_mount_at() {
        let mount = PlatformImpl::new().mount_at("/").unwrap();
        assert!(mount.fs_mounted_on == "/");
    }

    #[test]
    fn test_networks() {
        let networks = PlatformImpl::new().networks().unwrap();
        assert!(networks.values().find(|n| n.name == "lo0").unwrap().addrs.len() > 0);
    }

}
