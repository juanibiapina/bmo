mod support;

use support::Bmo;

#[test]
fn any_command_puts_the_skill_where_agents_find_it() {
    let bmo = Bmo::new();

    let listed = bmo.run(&["list"]);

    assert!(listed.success, "{}", listed.output());
    let skill = std::fs::read_to_string(bmo.skills().join("bmo/SKILL.md")).expect("the skill");
    assert!(skill.starts_with("---\nname: bmo\n"), "{skill}");
    assert!(skill.contains("bmo guide"), "{skill}");
}

#[test]
fn an_old_skill_is_replaced() {
    let bmo = Bmo::new();
    let file = bmo.skills().join("bmo/SKILL.md");
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(&file, "from an older bmo").unwrap();

    bmo.run(&["help"]);

    let skill = std::fs::read_to_string(&file).expect("the skill");
    assert!(skill.starts_with("---\nname: bmo\n"), "{skill}");
}

#[test]
fn a_skills_folder_that_cannot_be_written_does_not_stop_bmo() {
    let bmo = Bmo::new().with("BMO_SKILLS", "/dev/null/skills");

    let listed = bmo.run(&["list"]);

    assert!(listed.success, "{}", listed.output());
}
