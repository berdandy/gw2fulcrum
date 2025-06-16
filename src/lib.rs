use chatr::BuildTemplate as BuildTemplate;
use chatr::ChatCode as ChatCode;

pub struct BuildDep {
    pub skills: Vec<u16>,
    pub traits: Vec<u16>,
}

impl BuildDep  {
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
}
