use chatr::BuildTemplate as BuildTemplate;
use chatr::GearTemplate as GearTemplate;
use chatr::ChatCode as ChatCode;

use std::collections::HashSet as HashSet;

pub struct BuildDep {
    pub gear: GearTemplate,

    pub skills: Vec<u16>,
    pub traits: Vec<u16>,
}

impl BuildDep  {
    pub fn from_templates(gear: &GearTemplate, build: &BuildTemplate) -> BuildDep {
        BuildDep {
            gear: gear.clone(),
            skills: build.get_skill_ids().unwrap().to_vec(),
            traits: build.get_traits().to_vec(),
        }
    }

    pub fn from_build(build: &BuildTemplate) -> BuildDep {
        BuildDep {
            gear: GearTemplate::default(),
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

    pub fn skillset(self) -> HashSet<u16> {
        HashSet::from_iter(self.skills.iter().cloned())
    }
}
