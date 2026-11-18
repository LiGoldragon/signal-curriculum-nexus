//! Every input and output crosses the wire as an rkyv archive and is read
//! back whole.

use signal_curriculum_nexus::{
    BodySha256, CallerMetaflow, ContextPayloadSha256, ContextProvenance, ContextText,
    DeclaredPlacement, DescriptionPayloadSha256, DescriptionText, Effort, FlowId, Input, ModelId,
    Module, ModuleDescription, ModuleFile, ModuleIdentity, ModuleKind, ModuleSubject, Output,
    Permission, RecordsPayloadSha256, RecordsText, RenderedSha256, RepositoryName, ResolvedModule,
    RoleCallers, RoleComposition, RoleName, RoleSelection, RoleTools, SourceRelativePath,
    SourceRevision, SourceRoot, SourceSha256, StartupComposition, StartupPlacement,
    StartupSelection, Surface, SystemModules, SystemPayloadSha256, SystemText, Target, UserModules,
    UserPayloadSha256, UserText, Version, Versions,
};

fn identity(text: &str) -> ModuleIdentity {
    let mut parts = text.split(':');
    let module_kind = match parts.next().unwrap() {
        "spirit" => ModuleKind::Spirit,
        "intent" => ModuleKind::Intent,
        "vision" => ModuleKind::Vision,
        "knowledge" => ModuleKind::Knowledge,
        "inquiry" => ModuleKind::Inquiry,
        "compensation" => ModuleKind::Compensation,
        "role" => ModuleKind::Role,
        "raw" => ModuleKind::Raw,
        "panel" => ModuleKind::Panel,
        _ => panic!(),
    };
    let module_subject = ModuleSubject(parts.next().unwrap().into());
    let version = parts.next().unwrap();
    let versions = match version.split_once('-') {
        Some((first, last)) => Versions::Span(signal_curriculum_nexus::Span_Data {
            first: signal_curriculum_nexus::First(Version(first.parse().unwrap())),
            last: signal_curriculum_nexus::Last(Version(last.parse().unwrap())),
        }),
        None => Versions::Single(Version(version.parse().unwrap())),
    };
    ModuleIdentity {
        module_kind,
        module_subject,
        versions,
    }
}

fn caller() -> CallerMetaflow {
    CallerMetaflow {
        first_string: "Mind".into(),
        second_string: "curriculum".into(),
        third_string: "Opus".into(),
    }
}

fn provenance() -> ContextProvenance {
    ContextProvenance {
        module_identity: identity("spirit:core:0"),
        repository_name: RepositoryName("psyche-belief".into()),
        source_revision: SourceRevision("0".repeat(40)),
        source_relative_path: SourceRelativePath("spirit/core/0.md".into()),
        source_sha256: SourceSha256("a".repeat(64)),
        body_sha256: BodySha256("b".repeat(64)),
        rendered_sha256: RenderedSha256("c".repeat(64)),
        startup_placement: StartupPlacement::System,
        integer: 0,
    }
}

fn inputs() -> Vec<Input> {
    vec![
        Input::Modules,
        Input::ModulesVerbose,
        Input::ResolveSkills(vec![identity("spirit:core:0")]),
        Input::ResolveSources(vec![
            identity("intent:design:0"),
            identity("vision:ethos:0"),
            identity("vision:topic:0-1"),
        ]),
        Input::ComposeStartup(StartupSelection {
            caller_metaflow: caller(),
            target: Target::Codex,
            system_modules: SystemModules(vec![identity("spirit:core:0")]),
            user_modules: UserModules(vec![identity("knowledge:psyche:0")]),
            flow_id_vector: vec![FlowId("896bac".into())],
        }),
        Input::ComposeRole(RoleSelection {
            role_name: RoleName("read-trivial".into()),
            surface: Surface::PiAgent,
        }),
    ]
}

fn outputs() -> Vec<Output> {
    vec![
        Output::Modules(vec![Module {
            module_identity: identity("spirit:core:0"),
            module_file: ModuleFile("/belief/spirit/core/0.md".into()),
        }]),
        Output::ModulesVerbose(vec![ModuleDescription {
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
        }]),
        Output::SkillsResolved(vec![identity("spirit:core:0")]),
        Output::SourcesResolved(vec![ResolvedModule {
            module_identity: identity("spirit:core:0"),
            repository_name: RepositoryName("psyche-belief".into()),
            source_revision: SourceRevision("0".repeat(40)),
            source_root: SourceRoot("/belief/psyche-belief".into()),
            source_relative_path: SourceRelativePath("spirit/core/0.md".into()),
            declared_placement: DeclaredPlacement("system".into()),
            source_sha256: SourceSha256("a".repeat(64)),
            body_sha256: BodySha256("b".repeat(64)),
            string: "The body.\n".into(),
            string_vector: vec!["intent:design:0".into()],
        }]),
        Output::StartupComposed(StartupComposition {
            merged_panels: signal_curriculum_nexus::MergedPanels(vec![
                signal_curriculum_nexus::PanelProvenance {
                    module_identity: identity("panel:native:0"),
                    repository_name: RepositoryName("mind-belief".into()),
                    source_revision: SourceRevision("d".repeat(40)),
                    source_relative_path: SourceRelativePath("panel/native/0.md".into()),
                    source_sha256: SourceSha256("e".repeat(64)),
                    body_sha256: BodySha256("f".repeat(64)),
                    rendered_sha256: RenderedSha256("a".repeat(64)),
                },
            ]),
            missing_panels: signal_curriculum_nexus::MissingPanels(vec![identity(
                "panel:missing:0",
            )]),
            system_text: SystemText("system".into()),
            user_text: UserText("user".into()),
            records_text: RecordsText(String::new()),
            system_payload_sha256: SystemPayloadSha256("d".repeat(64)),
            user_payload_sha256: UserPayloadSha256("e".repeat(64)),
            records_payload_sha256: RecordsPayloadSha256("f".repeat(64)),
            context_provenance_vector: vec![provenance()],
        }),
        Output::RoleComposed(RoleComposition {
            merged_panels: signal_curriculum_nexus::MergedPanels(vec![
                signal_curriculum_nexus::PanelProvenance {
                    module_identity: identity("panel:native:0"),
                    repository_name: RepositoryName("mind-belief".into()),
                    source_revision: SourceRevision("d".repeat(40)),
                    source_relative_path: SourceRelativePath("panel/native/0.md".into()),
                    source_sha256: SourceSha256("e".repeat(64)),
                    body_sha256: BodySha256("f".repeat(64)),
                    rendered_sha256: RenderedSha256("a".repeat(64)),
                },
            ]),
            missing_panels: signal_curriculum_nexus::MissingPanels(vec![identity(
                "panel:missing:0",
            )]),
            role_name: RoleName("read-trivial".into()),
            surface: Surface::ClaudeAgent,
            model_id: ModelId("claude-haiku".into()),
            effort: Effort::Low,
            permission: Permission::Restricted,
            description_text: DescriptionText("Reads.".into()),
            context_text: ContextText("Context.".into()),
            role_tools: RoleTools::Explicit(vec!["Read".into()]),
            role_callers: RoleCallers::Only(vec![caller()]),
            context_provenance_vector: vec![provenance()],
            description_payload_sha256: DescriptionPayloadSha256("1".repeat(64)),
            context_payload_sha256: ContextPayloadSha256("2".repeat(64)),
        }),
        Output::Rejected("unknown role/surface".into()),
    ]
}

#[test]
fn every_input_reads_back_from_its_archive() {
    for input in inputs() {
        let frame = rkyv::to_bytes::<rkyv::rancor::Error>(&input).expect("archive input");
        assert_eq!(
            rkyv::from_bytes::<Input, rkyv::rancor::Error>(&frame).expect("read input"),
            input
        );
    }
}

#[test]
fn every_output_reads_back_from_its_archive() {
    for output in outputs() {
        let frame = rkyv::to_bytes::<rkyv::rancor::Error>(&output).expect("archive output");
        assert_eq!(
            rkyv::from_bytes::<Output, rkyv::rancor::Error>(&frame).expect("read output"),
            output
        );
    }
}

#[test]
fn an_input_frame_is_not_read_as_an_output() {
    let frame = rkyv::to_bytes::<rkyv::rancor::Error>(&Input::ComposeRole(RoleSelection {
        role_name: RoleName("read-trivial".into()),
        surface: Surface::PiAgent,
    }))
    .expect("archive input");
    assert!(rkyv::from_bytes::<Output, rkyv::rancor::Error>(&frame).is_err());
}
