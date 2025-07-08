use chatr::BuildTemplate as BuildTemplate;
use chatr::GearTemplate as GearTemplate;
use chatr::ChatCode as ChatCode;

use std::collections::HashSet as HashSet;

#[derive(Default, Debug, PartialEq)]
pub struct BuildDependencies {
    pub skills: Vec<u32>,
    pub traits: Vec<u16>,
}

impl BuildDependencies  {
    pub fn from_templates(gear: &GearTemplate, build: &BuildTemplate) -> BuildDependencies {
        let mut skillset = build.get_skill_ids().unwrap().to_vec();
        let gearskills = Vec::from_iter(gear.get_weapon_skills(build.profession));
        skillset.extend(gearskills);

        BuildDependencies {
            skills: skillset,
            traits: build.get_traits().to_vec(),
        }
    }

    pub fn from_build(build: &BuildTemplate) -> BuildDependencies {
        BuildDependencies {
            skills: build.get_skill_ids().unwrap().to_vec(),
            traits: build.get_traits().to_vec(),
        }
    }

    pub fn from_chatcode(code: &ChatCode) -> BuildDependencies {
        let build = BuildTemplate::try_from_chatcode(code).unwrap();
        Self::from_build(&build)
    }

    pub fn traitset(self) -> HashSet<u16> {
        HashSet::from_iter(self.traits.iter().cloned())
    }

    pub fn skillset(self) -> HashSet<u32> {
        HashSet::from_iter(self.skills.iter().cloned())
    }
}

#[derive(Default, Debug)]
pub struct BalanceUpdate {
    pub skills: Vec<u32>,
    pub traits: Vec<u16>,
}

impl BalanceUpdate {
    pub fn parse_notes(_notes :&str) -> BalanceUpdate {
        BalanceUpdate {
            skills: vec!(1, 2, 3),
            traits: vec!(4, 5, 6),
        }
    }
}
