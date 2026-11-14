//! Under the datom feature, an input is read from the text a CLI is given,
//! and an output is written as the text a CLI prints.
#![cfg(feature = "datom")]

use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use protos::{Protosizable, ReaderBudget, Textualizable};
use signal_curriculum_nexus::{Input, ModuleIdentity, Output, RoleName, RoleSelection, Surface};

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
        Potential::<Input>::from("ResolveSkills.[ spirit:core:0 ]")
            .actualize(&mut budget())
            .expect("an input"),
        Input::ResolveSkills(vec![ModuleIdentity("spirit:core:0".into())])
    );
}

#[test]
fn an_output_is_written_as_its_text() {
    assert_eq!(
        Output::SkillsResolved(vec![ModuleIdentity("spirit:core:0".into())])
            .datomize(vec![])
            .protosize()
            .textualize(),
        "SkillsResolved.[ spirit:core:0 ]"
    );
}
