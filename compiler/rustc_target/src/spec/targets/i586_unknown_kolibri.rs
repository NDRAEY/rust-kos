use crate::spec::{Arch, Cc, LinkerFlavor, Lld, Target, TargetMetadata, base};

const LINKER_SCRIPT: &str = include_str!("./i586_unknown_kolibri.ld");

pub(crate) fn target() -> Target {
    let mut base = base::kolibri::opts();
    
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m32", "-nostdlib"]);

    base.link_script = Some(LINKER_SCRIPT.into());

    Target {
        llvm_target: "i586-unknown-kolibri".into(),
        metadata: TargetMetadata { description: None, tier: Some(1), host_tools: Some(false), std: Some(false) },
        pointer_width: 32,
        data_layout:
            "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-i128:128-f64:32:64-f80:32-n8:16:32-S128"
                .into(),
        arch: Arch::X86,
        options: base,
    }
}
