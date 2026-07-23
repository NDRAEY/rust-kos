use crate::spec::{Os, PanicStrategy, RelroLevel, TargetOptions, crt_objects};

use crate::spec::LinkerFlavor;
use crate::spec::{Cc, Lld};


pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: Os::Kolibri,
        cpu: "pentium".into(),
        plt_by_default: false,
        max_atomic_width: Some(32),
        dynamic_linking: false,
        relocation_model: crate::spec::RelocModel::Static,
        relro_level: RelroLevel::Off,
        singlethread: true, // Remove that when I add threads support
        panic_strategy: PanicStrategy::Abort,
        stack_probes: crate::spec::StackProbeType::None,
        
        pre_link_objects: crt_objects::pre_kolibri(),
        post_link_objects: crt_objects::post_kolibri(),
        pre_link_objects_self_contained: crt_objects::pre_kolibri(),
        post_link_objects_self_contained: crt_objects::post_kolibri(),
        
        linker: Some("ld.lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),

        exe_suffix: ".kex".into(),

        ..Default::default()
    }
}
