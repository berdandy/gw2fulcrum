use regex::Regex;

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

    let expected_skills = vec!(30488, 10620, 10583, 10685, 30105);
    let expected_traits = vec!(1876, 1844, 782, 875, 894, 893, 2020, 2031, 2021);

    assert_eq!(dep.skills, expected_skills);
    assert_eq!(dep.traits, expected_traits);
}

#[test]
#[ignore = "not yet implemented"]
fn extract_gear_deps_from_buildmd() {
/*
    let build_str = include_str!("sample_build_1.md");

    let re = Regex::new(r"\[&(?<code>[A-Za-z0-9]+=)\]").unwrap();
    let caps = re.captures(build_str).unwrap();
    let codestring = &caps["code"];

    let code = chatr::ChatCode::build(&codestring).unwrap();
    let dep = fulcrum::BuildDep::from_chatcode(&code);

    let expected_skills = vec!(30488, 10620, 10583, 10685, 30105);
    let expected_traits = vec!(1876, 1844, 782, 875, 894, 893, 2020, 2031, 2021);

    assert_eq!(dep.skills, expected_skills);
    assert_eq!(dep.traits, expected_traits);
*/
}
