use chatr::BuildTemplate as BuildTemplate;
use chatr::GearTemplate as GearTemplate;
use chatr::ChatCode as ChatCode;

use std::collections::HashSet as HashSet;
use std::collections::HashMap as HashMap;
use regex::Regex;
use csv;

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

type UpdateElement = (u32, String);

#[derive(Default, Debug)]
pub struct BalanceUpdate {
    pub skills: Vec<u32>,
    pub traits: Vec<u16>,
}

impl BalanceUpdate {
    /// returns true if any of the provided dependencies are present in the update
    pub fn affects(&self, dependencies: &BuildDependencies) -> bool {
		for trait_id in &self.traits {
            for changed_trait in &dependencies.traits {
                if trait_id == changed_trait {
                    return true
                }
            }
        }
		for skill_id in &self.skills {
            for changed_skill in &dependencies.skills {
                if skill_id == changed_skill {
                    return true
                }
            }
        }
        false
    }

    pub fn parse_notes(notes :&str) -> BalanceUpdate {
        // skills to id mapping
        let mut skillmap = HashMap::new();
		let all_skills_str = include_str!("skills.csv");
        let mut rdr = csv::Reader::from_reader(all_skills_str.as_bytes());
        for result in rdr.deserialize::<UpdateElement>() {
            if let Ok(ue) = result {
                if ue.1 != "" {
                    skillmap.insert(ue.1, ue.0);
                }
            }
        }
        // println!("Skillmap {:?}", skillmap.len());

        let mut traitmap = HashMap::new();
		let all_traits_str = include_str!("traits.csv");
        let mut rdr = csv::Reader::from_reader(all_traits_str.as_bytes());
        for result in rdr.deserialize::<UpdateElement>() {
            if let Ok(ue) = result {
                if ue.1 != "" {
                    traitmap.insert(ue.1, ue.0);
                }
            }
        }
        // println!("Traitmap {:?}", traitmap.len());

        // skills & traits are combined in the balance notes
        let mut skills = HashSet::new();
        let mut traits = HashSet::new();
        let re = Regex::new(r"(?<NAME>[\w' ]+):.*").unwrap();
        for caps in re.captures_iter(notes) {
            if let Some(name) = caps.name("NAME") {
                if let Some(id) = skillmap.get(name.as_str()) {
                    skills.insert(id.clone());
                }
                if let Some(id) = traitmap.get(name.as_str()) {
                    traits.insert(id.clone() as u16);
                }
            }
        }
        // println!("{:?}", skills.len());
        // println!("{:?}", traits.len());

        BalanceUpdate {
            skills: Vec::from_iter(skills),
            traits: Vec::from_iter(traits),
        }
    }
}
