use crate::spec::{
    BinaryFormat, Os, PanicStrategy, RelocModel, RelroLevel, StackProbeType, TargetOptions,
    crt_objects,
};

use crate::spec::LinkerFlavor;
use crate::spec::{Cc, Lld};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: Os::Kolibri,
        cpu: "pentium".into(),
        plt_by_default: false,
        dynamic_linking: false,
        relocation_model: RelocModel::Static,
        relro_level: RelroLevel::Off,
        panic_strategy: PanicStrategy::Abort,
        stack_probes: StackProbeType::None,

        binary_format: BinaryFormat::Elf,

        atomic_cas: true,

        singlethread: true, // Remove that when I add threads support
        has_thread_local: false,
        // tls_model: crate::spec::TlsModel::InitialExec,
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
