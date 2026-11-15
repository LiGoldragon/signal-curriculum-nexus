//! Under the datom feature, an input is read from the text a CLI is given,
//! and an output is written as the text a CLI prints.
#![cfg(feature = "datom")]

use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use protos::{Protosizable, ReaderBudget, Textualizable};
use signal_curriculum_nexus::{
    DescriptionText, Input, Module, ModuleDescription, ModuleFile, ModuleIdentity, ModuleKind,
    ModuleSubject, Output, RoleName, RoleSelection, Surface, Version, Versions,
};

fn identity() -> ModuleIdentity {
    ModuleIdentity {
        module_kind: ModuleKind::Spirit,
        module_subject: ModuleSubject("core".into()),
        versions: Versions::Single(Version(0)),
    }
}

fn budget() -> Budget {
    Budget {
        remaining: 1_048_576,
        reader: ReaderBudget::try_from(1_048_576).expect("a constant reader budget"),
        depth: 0,
        maximum_depth: 1024,
    }
}

#[test]
fn an_input_is_read_from_its_text() {
    assert_eq!(
        Potential::<Input>::from("ComposeRole.{ read-trivial ClaudeAgent }")
            .actualize(&mut budget())
            .expect("an input"),
        Input::ComposeRole(RoleSelection {
            role_name: RoleName("read-trivial".into()),
            surface: Surface::ClaudeAgent,
        })
    );
    assert_eq!(
        Potential::<Input>::from("ResolveSkills.[ { Spirit core Single.0 } ]")
            .actualize(&mut budget())
            .expect("an input"),
        Input::ResolveSkills(vec![identity()])
    );
    assert_eq!(
        Potential::<Input>::from("ResolveSources.[ { Vision topic Span.{ 0 1 } } ]")
            .actualize(&mut budget())
            .expect("a span input"),
        Input::ResolveSources(vec![ModuleIdentity {
            module_kind: ModuleKind::Vision,
            module_subject: ModuleSubject("topic".into()),
            versions: Versions::Span(signal_curriculum_nexus::Span_Data {
                first: signal_curriculum_nexus::First(Version(0)),
                last: signal_curriculum_nexus::Last(Version(1)),
            }),
        }])
    );
}

#[test]
fn an_output_is_written_as_its_text() {
    assert_eq!(
        Output::SkillsResolved(vec![identity()])
            .datomize(vec![])
            .protosize()
            .textualize(),
        "SkillsResolved.[ { Spirit\n                   core\n                   Single.0 } ]"
    );
}

#[test]
fn listing_inputs_and_empty_outputs_are_typed() {
    assert_eq!(
        Potential::<Input>::from("Modules")
            .actualize(&mut budget())
            .unwrap(),
        Input::Modules
    );
    assert_eq!(
        Potential::<Input>::from("ModulesVerbose")
            .actualize(&mut budget())
            .unwrap(),
        Input::ModulesVerbose
    );
    assert_eq!(
        Output::Modules(Vec::new())
            .datomize(vec![])
            .protosize()
            .textualize(),
        "Modules.[]"
    );
    assert_eq!(
        Output::ModulesVerbose(Vec::new())
            .datomize(vec![])
            .protosize()
            .textualize(),
        "ModulesVerbose.[]"
    );
}

#[test]
fn listing_records_preserve_singleton_and_span_identity_and_file() {
    let singleton = Module {
        module_identity: identity(),
        module_file: ModuleFile("/belief/spirit/core/0.md".into()),
    };
    let span = ModuleDescription {
        module: Module {
            module_identity: ModuleIdentity {
                module_kind: ModuleKind::Vision,
                module_subject: ModuleSubject("topic".into()),
                versions: Versions::Span(signal_curriculum_nexus::Span_Data {
                    first: signal_curriculum_nexus::First(Version(0)),
                    last: signal_curriculum_nexus::Last(Version(1)),
                }),
            },
            module_file: ModuleFile("/belief/vision/topic/0-1.md".into()),
        },
        description_text: DescriptionText("A span.".into()),
    };
    assert_eq!(
        Output::Modules(vec![singleton])
            .datomize(vec![])
            .protosize()
            .textualize(),
        "Modules.[ { { Spirit\n              core\n              Single.0 }\n            /belief/spirit/core/0.md } ]"
    );
    assert_eq!(
        Output::ModulesVerbose(vec![span])
            .datomize(vec![])
            .protosize()
            .textualize(),
        "ModulesVerbose.[ { { { Vision\n                       topic\n                       Span.{ 0 1 } }\n                     /belief/vision/topic/0-1.md }\n                   «A span.» } ]"
    );
}
