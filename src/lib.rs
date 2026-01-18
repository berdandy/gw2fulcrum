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
pub struct BalanceChange {
    pub sk: u32,
    pub tr: u16,
    pub note: String,
}

#[derive(Default, Debug)]
pub struct BalanceUpdate {
    pub changes: Vec<BalanceChange>,
}

impl BalanceUpdate {
    pub fn new() -> BalanceUpdate {
        BalanceUpdate {
            changes: Vec::new()
        }
    }

    pub fn add_change(&mut self, sk: u32, tr: u16, note: &str) {
        self.changes.push(BalanceChange{
            sk: sk,
            tr: tr,
            note: String::from(note),
        })
    }

    pub fn print(&self)
    {
        for change in &self.changes {
            if change.sk > 0 {
                println!("S: {} - {}", change.sk, change.note)
            } else if change.tr > 0 {
                println!("T: {} - {}", change.tr, change.note)
            }
        }
    }

    /// returns Some("change note") if any of the provided dependencies are present in the update
    pub fn affects(&self, dependencies: &BuildDependencies) -> Option<&str> {
        for change in &self.changes {
            for dep_trait in &dependencies.traits {
                if change.tr == *dep_trait {
                    return Some(&change.note)
                }
            }
            for dep_skill in &dependencies.skills {
                if change.sk == *dep_skill {
                    return Some(&change.note)
                }
            }
        }
        None
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
        let mut update = BalanceUpdate::new();
        let re = Regex::new(r"(?<NAME>[\w' ]+): (?<NOTE>.*)").unwrap();
        for caps in re.captures_iter(notes) {
            if let (Some(name), Some(note)) = (caps.name("NAME"), caps.name("NOTE")) {
                let mut sk = 0;
                if let Some(id) = skillmap.get(name.as_str()) {
                    sk = id.clone();
                }
                let mut tr = 0;
                if let Some(id) = traitmap.get(name.as_str()) {
                    tr = id.clone() as u16;
                }
                update.add_change(sk, tr, note.into());
            }
        }
        update
    }
}
