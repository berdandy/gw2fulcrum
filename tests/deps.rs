use regex::Regex;
use std::collections::HashSet;

use chatr;
use fulcrum;

#[test]
fn extract_build_deps_from_buildmd() {
    let build_str = include_str!("sample_build_1.md");

    let re = Regex::new(r"\[&(?<code>[A-Za-z0-9]+=)\]").unwrap();
    let caps = re.captures(build_str).unwrap();
    let codestring = &caps["code"];

    let code = chatr::ChatCode::build(&codestring).unwrap();
    let dep = fulcrum::BuildDep::from_chatcode(&code);

    // TODO: ADD weapon skills from gs & sword to this
    // TODO: sort skills and traits
    let expected_skills = vec!(30488, 10620, 10583, 10685, 30105);
    let expected_traits = vec!(1876, 1844, 782, 875, 894, 893, 2020, 2031, 2021);

    assert_eq!(dep.skills, expected_skills);
    assert_eq!(dep.traits, expected_traits);
}

#[test]
fn extract_gear_from_buildmd() {
    let build_str = include_str!("sample_build_1.md");

    let build = chatr::BuildTemplate::parse_string(build_str).unwrap();
    let gear = chatr::GearTemplate::parse_string(build_str).unwrap();

    let dep = fulcrum::BuildDep::from_templates(&gear, &build);

    let expected_gear = chatr::GearTemplate{
        armor: [161; 6],                        // berserker's
        rune:  24836,                           // scholar
        weapon_types: [0; 4],                   // ? gs, sw/sw
        weapons: [161; 4],                      // berserker's
        sigils: [24615, 24597, 24615, 24554],   // force/hydro, force/air

        trinkets: [161; 6],                     // berserker's
        relic: 101580                           // thief
    };
    assert_eq!(dep.gear, expected_gear);
}

#[test]
fn get_dep_traitset() {
    let code = chatr::ChatCode::build("[&DQYpGyU+OD90AAAAywAAAI8AAACRAAAAJgAAAAAAAAAAAAAAAAAAAAAAAAA=]").unwrap();
    let dep = fulcrum::BuildDep::from_chatcode(&code);

    let expected = HashSet::from([2170, 232, 214, 266, 226, 1511, 257, 2115, 2138]);
    assert_eq!(dep.traitset(), expected);
}

#[test]
fn get_dep_skillset() {
    let code = chatr::ChatCode::build("[&DQYpGyU+OD90AAAAywAAAI8AAACRAAAAJgAAAAAAAAAAAAAAAAAAAAAAAAA=]").unwrap();
    let dep = fulcrum::BuildDep::from_chatcode(&code);

    let expected = HashSet::from([5571, 5666, 5570, 5503, 5542]);
    assert_eq!(dep.skillset(), expected);
}

/*
#[test]
fn dependency_trait_skill_changed_build() {
    let code = chatr::BuildTemplate::from_string("[&DQYpGyU+OD90AAAAywAAAI8AAACRAAAAJgAAAAAAAAAAAAAAAAAAAAAAAAA=]");
    let dep = fulcrum::BuildDep::from_chatcode(&code); // hashmap?

    // these traits and skills are in the template
    let breaking_update = Update {
        id: String::from("Breaking 2025"),
        traits: vec!(226, 2115),
        skills: vec!(5570),
    };
    assert!(dep.contains_trait(&breaking_update));
}
*/

/*
#[test]
fn dependency_no_change_build() {
    let code = chatr::BuildTemplate::from_string("[&DQYpGyU+OD90AAAAywAAAI8AAACRAAAAJgAAAAAAAAAAAAAAAAAAAAAAAAA=]");
    let dep = fulcrum::BuildDep::from_chatcode(&code);

    let safe_update = Update {
        id: String::from("Safe 20xx"),
        traits: vec!(123),
        skills: vec!(5000),
    };
    assert_eq!(false, build.did_change(&safe_update));
}
*/
