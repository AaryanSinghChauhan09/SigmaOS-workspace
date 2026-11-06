//! SigmaOS-workspace — Cargo workspace consolidation for core components.
//!
//! The workspace `Cargo.toml` at the repo root unifies independently
//! maintained component repositories into one build graph. This crate
//! exposes the canonical member list and workspace metadata helpers so
//! tooling (meta build runner, initramfs packer, CI) can discover the
//! core set without hardcoding names.

/// Canonical core-component crate directories (relative to ~).
pub const CORE_MEMBERS: &[&str] = &[
    "SigmaOS-kernel",
    "SigmaOS-vfs",
    "SigmaOS-scheduler",
    "SigmaOS-driver-framework",
    "SigmaOS-interrupt",
    "SigmaOS-init",
    "SigmaOS-allocators",
    "SigmaOS-posix",
    "SigmaOS-sys",
    "SigmaOS-initramfs",
    "SigmaOS-coreutils",
    "SigmaOS-boot-manager",
    "SigmaOS-memory-manager",
];

/// Workspace layers — the SigmaOS architectural stack each member serves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    /// Kernel-space mechanisms: scheduling, memory, interrupts, drivers.
    Kernel,
    /// System integration: init, boot, POSIX/syscall ABI.
    System,
    /// Userland baseline: coreutils, initramfs packaging.
    Userland,
}

/// Map a member directory to its architectural layer.
pub fn layer_of(member: &str) -> Option<Layer> {
    match member {
        "SigmaOS-kernel" | "SigmaOS-vfs" | "SigmaOS-scheduler"
        | "SigmaOS-driver-framework" | "SigmaOS-interrupt" | "SigmaOS-allocators" => {
            Some(Layer::Kernel)
        }
        "SigmaOS-init" | "SigmaOS-boot-manager" | "SigmaOS-posix" | "SigmaOS-sys"
        | "SigmaOS-memory-manager" => Some(Layer::System),
        "SigmaOS-coreutils" | "SigmaOS-initramfs" => Some(Layer::Userland),
        _ => None,
    }
}

/// Members grouped by layer, in declaration order.
pub fn members_by_layer() -> Vec<(Layer, Vec<&'static str>)> {
    let mut out: Vec<(Layer, Vec<&'static str>)> = Vec::new();
    for m in CORE_MEMBERS {
        if let Some(l) = layer_of(m) {
            match out.iter_mut().find(|(layer, _)| *layer == l) {
                Some((_, v)) => v.push(m),
                None => out.push((l, vec![m])),
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn every_member_is_classified() {
        for m in CORE_MEMBERS {
            assert!(layer_of(m).is_some(), "unclassified member: {m}");
        }
    }

    #[test]
    fn members_exist_on_disk() {
        let home = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
        for m in CORE_MEMBERS {
            assert!(home.join(m).join("Cargo.toml").exists(), "missing {m}");
        }
    }

    #[test]
    fn layers_cover_all_members() {
        let total: usize = members_by_layer().iter().map(|(_, v)| v.len()).sum();
        assert_eq!(total, CORE_MEMBERS.len());
    }
}
