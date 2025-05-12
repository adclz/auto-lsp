/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

use super::utils::Result;
use crate::snap;

#[test]
fn matching_specific_values() -> Result {
    snap!(
        r#"
match command.split():
    case ["quit"]:
        print("Goodbye!")
        quit_game()
    case ["look"]:
        current_room.describe()
    case ["get", obj]:
        character.get(obj, current_room)
    case ["go", direction]:
        current_room = current_room.neighbor(direction)
    # The rest of your commands go here
"#
    )
}

#[test]
fn matching_multiple_values() -> Result {
    snap!(
        r#"
match command.split():
    case ["drop", *objects]:
        for obj in objects:
            character.drop(obj, current_room)
"#
    )
}

#[test]
fn adding_a_wild_card() -> Result {
    snap!(
        r#"
match command.split():
# ^ conditional
    case ["quit"]: ... # Code omitted for brevity
    case ["go", direction]: pass
    case ["drop", *objects]: pass
    case _:
        print(f"Sorry, I couldn't understand {command!r}")
"#
    )
}

#[test]
fn or_patterns() -> Result {
    snap!(
        r#"
match command.split():
    case ["north"] | ["go", "north"]:
        current_room = current_room.neighbor("north")
    case ["get", obj] | ["pick", "up", obj] | ["pick", obj, "up"]:
        pass
"#
    )
}

#[test]
fn as_patterns() -> Result {
    snap!(
        r#"
match command.split():
    case ["go", ("north" | "south" | "east" | "west") as direction]:
        current_room = current_room.neighbor(direction)
"#
    )
}

#[test]
fn actually_not_match() -> Result {
    snap!(
        r#"
match = 2
match, a = 2, 3
match: int = secret
x, match = 2, "hey, what's up?"
*match, last = [1, 2, 3]
def foo(**match): pass
"#
    )
}

#[test]
fn match_is_match_but_not_pattern_matching() -> Result {
    snap!(
        r#"
a = [match]
match = [match]
"#
    )
}

#[test]
fn match_kwargs() -> Result {
    snap!(
        r#"
field = call(match=r".*\.txt$")
"#
    )
}

#[test]
fn match_kwargs_2() -> Result {
    snap!(
        r#"
field = match(match=match, match)
"#
    )
}

#[test]
fn case_used_as_identifier() -> Result {
    snap!(
        r#"
a = [case]
case = [case]
just_in_case = call_me(case=True)
"#
    )
}

#[test]
fn if_guards() -> Result {
    snap!(
        r#"
match 0:
    case 0 if False:
        x = False
    case 0 if True:
        x = True
"#
    )
}

#[test]
fn literals() -> Result {
    snap!(
        r#"
match xxx:
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
"#
    )
}

#[test]
fn comma_separated_cases() -> Result {
    snap!(
        r#"
match (0, 1, 2):
    case 0,1:
        x = 0
    case 0, *x:
        x = 0
"#
    )
}

#[test]
fn case_terminating_in_comma() -> Result {
    snap!(
        r#"
match x,:
    case *x,:
        y = 0
"#
    )
}

#[test]
fn multiple_match_patterns() -> Result {
    snap!(
        r#"
match ..., ...:
    case a, b:
        return locals()
"#
    )
}

#[test]
fn match_match_case_case() -> Result {
    snap!(
        r#"
match = case = 0
match match:
    case case:
        x = 0
"#
    )
}

#[test]
fn walrus_match_issue_150() -> Result {
    snap!(
        r#"
if match := re.fullmatch(r"(-)?(\d+:)?\d?\d:\d\d(\.\d*)?", time, flags=re.ASCII):
    return 42
"#
    )
}

#[test]
fn matching_objects() -> Result {
    snap!(
        r#"
match event.get():
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
"#
    )
}

#[test]
fn positional_arguments() -> Result {
    snap!(
        r#"
match event.get():
    case Click((x, y)):
        handle_click_at(x, y)
"#
    )
}

#[test]
fn constants_and_enums() -> Result {
    snap!(
        r#"
match event.get():
    case Click((x, y), button=Button.LEFT):  # This is a left click
        handle_click_at(x, y)
    case Click():
        pass  # ignore other clicks
"#
    )
}
