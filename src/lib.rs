use chatr::BuildTemplate as BuildTemplate;
use chatr::GearTemplate as GearTemplate;
use chatr::ChatCode as ChatCode;

use std::collections::HashSet as HashSet;

pub struct BuildDep {
    pub skills: Vec<u32>,
    pub traits: Vec<u16>,
}

impl BuildDep  {
    pub fn from_templates(gear: &GearTemplate, build: &BuildTemplate) -> BuildDep {
        let mut skillset = build.get_skill_ids().unwrap().to_vec();
        let gearskills = gear.get_weapon_skills(build.profession);
        skillset.extend(gearskills);

        BuildDep {
            skills: skillset,
            traits: build.get_traits().to_vec(),
        }
    }

    pub fn from_build(build: &BuildTemplate) -> BuildDep {
        BuildDep {
            skills: build.get_skill_ids().unwrap().to_vec(),
            traits: build.get_traits().to_vec(),
        }
    }

    pub fn from_chatcode(code: &ChatCode) -> BuildDep {
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
