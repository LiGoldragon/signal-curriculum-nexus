//! An input is answered by position: each lands on its own capability, which
//! yields the output at the input's position or a refusal, any Output.

use signal_curriculum_nexus::{
    Answerable, Answering, Input, ModuleIdentity, Output, ResolvedModule, RoleComposition,
    RoleName, RoleSelection, StartupComposition, StartupSelection, Surface,
};

/// Answers every module identity back, and refuses everything else with the
/// capability's name.
struct Echo;

impl Answering for Echo {
    fn resolve_skills(
        &self,
        module_identity_vector: &Vec<ModuleIdentity>,
    ) -> Result<Vec<ModuleIdentity>, Output> {
        Ok(module_identity_vector.clone())
    }

    fn resolve_sources(
        &self,
        _module_identity_vector: &Vec<ModuleIdentity>,
    ) -> Result<Vec<ResolvedModule>, Output> {
        Ok(Vec::new())
    }

    fn compose_startup(
        &self,
        _startup_selection: &StartupSelection,
    ) -> Result<StartupComposition, Output> {
        Err(Output::Rejected("compose_startup".into()))
    }

    fn compose_role(&self, role_selection: &RoleSelection) -> Result<RoleComposition, Output> {
        Err(Output::Rejected(role_selection.role_name.0.clone()))
    }
}

#[test]
fn each_input_is_answered_by_its_own_capability() {
    let identities = vec![ModuleIdentity("spirit:core:0".into())];
    assert_eq!(
        Input::ResolveSkills(identities.clone()).answer(&Echo),
        Output::ResolvedSkills(identities.clone())
    );
    assert_eq!(
        Input::ResolveSources(identities).answer(&Echo),
        Output::ResolvedSources(Vec::new())
    );
}

#[test]
fn a_refusal_is_answered_whole() {
    assert_eq!(
        Input::ComposeRole(RoleSelection {
            role_name: RoleName("read-trivial".into()),
            surface: Surface::ClaudeAgent,
        })
        .answer(&Echo),
        Output::Rejected("read-trivial".into())
    );
}
