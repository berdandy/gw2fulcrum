# GW2 Fulcrum - A Balance Update tool for Accessibility Wars

This tool does the following:
- parses the balance update notes from ArenaNet into a list of changed skills and traits
- parses accessibility wars builds into a list of skills and traits
- highlights builds that have been affected by balance changes

```
Usage: gw2fulcrum [OPTIONS] <UPDATE_PATH> [BUILDS]...

Arguments:
  <UPDATE_PATH>  update note file, in the format of: "(SKILL_OR_TRAIT_NAME): (NOTES)"
  [BUILDS]...    build markdown filenames (in https://aw2.help format)

Options:
  -v, --verbose  
  -i, --invert   
  -h, --help     Print help
  -V, --version  Print version
```

UPDATE_PATH file must be in the form:
```
name of trait: notes
name of skill: notes
```
See `tests/` for an example
