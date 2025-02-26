use auto_lsp_core::build::{TryParse, TestParseResult};

use super::super::python_workspace::*;
use crate::python::ast::{IfStatement, MatchStatement};

#[test]
fn matching_specific_values() -> TestParseResult {
    MatchStatement::test_parse(
        r#"match command.split():
    case ["quit"]:
        print("Goodbye!")
        quit_game()
    case ["look"]:
        current_room.describe()
    case ["get", obj]:
        character.get(obj, current_room)
    case ["go", direction]:
        current_room = current_room.neighbor(direction)"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn matching_multiple_values() -> TestParseResult {
    MatchStatement::test_parse(
        r#"match command.split():
    case ["drop", *objects]:
        for obj in objects:
            character.drop(obj, current_room)"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn adding_a_wildcard() -> TestParseResult {
    MatchStatement::test_parse(
        r#"match command.split():
# ^ conditional
    case ["quit"]: ... # Code omitted for brevity
    case ["go", direction]: pass
    case ["drop", *objects]: pass
    case _:
        print(f"Sorry, I couldn't understand {command!r}")"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn or_patterns() -> TestParseResult {
    MatchStatement::test_parse(
        r#"match command.split():
    case ["north"] | ["go", "north"]:
        current_room = current_room.neighbor("north")
    case ["get", obj] | ["pick", "up", obj] | ["pick", obj, "up"]:
        pass
"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn as_patterns() -> TestParseResult {
    MatchStatement::test_parse(
        r#"match command.split():
    case ["go", ("north" | "south" | "east" | "west") as direction]:
        current_room = current_room.neighbor(direction)"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn if_guards() -> TestParseResult {
    MatchStatement::test_parse(
        r#"match 0:
    case 0 if False:
        x = False
    case 0 if True:
        x = True"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn literals() -> TestParseResult {
    MatchStatement::test_parse(
        r#"match xxx:
    case 3 | -3:
      pass
    case "something":
      pass
    case "something" "else":
      pass
    case 1.0 | -1.0:
      pass
    case True | False:
      pass
    case None:
      pass
"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn coma_separated_cases() -> TestParseResult {
    MatchStatement::test_parse(
        r#"match x,:
    case *x,:
        y = 0
"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn case_terminating_in_comma() -> TestParseResult {
    MatchStatement::test_parse(
        r#"match ..., ...:
    case a, b:
        return locals()
"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn multiple_match_patterns() -> TestParseResult {
    MatchStatement::test_parse(
        r#"match ..., ...:
    case a, b:
        return locals()
"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn walrus_match() -> TestParseResult {
    IfStatement::test_parse(
        r#"if match := re.fullmatch(r"(-)?(\d+:)?\d?\d:\d\d(\.\d*)?", time, flags=re.ASCII):
    return 42
"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn matching_objects() -> TestParseResult {
    MatchStatement::test_parse(
        r#"match event.get():
    case Click(position=(x, y)):
        handle_click_at(x, y)
    case KeyPress(key_name="Q") | Quit():
        game.quit()
    case KeyPress(key_name="up arrow"):
        game.go_north()
        ...
    case KeyPress():
        pass # Ignore other keystrokes
    case other_event:
        raise ValueError(f"Unrecognized event: {other_event}")
"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn positional_arguments() -> TestParseResult {
    MatchStatement::test_parse(
        r#"match event.get():
    case Click((x, y)):
        handle_click_at(x, y)"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}

#[test]
fn constant_and_enums() -> TestParseResult {
    MatchStatement::test_parse(
        r#"match event.get():
    case Click((x, y), button=Button.LEFT):  # This is a left click
        handle_click_at(x, y)
    case Click():
        pass  # ignore other clicks"#,
        PYTHON_PARSERS.get("python").unwrap(),
    )
}
