# Integration tests

Cross-team integration and conformance tests live here (architecture §5).

| Kind            | Purpose                                              |
|-----------------|------------------------------------------------------|
| Pipeline tests  | HTML → CSS → layout → paint (or JS execution) paths  |
| WPT subset      | Web Platform Tests we choose to track                |
| test262 subset  | JavaScript conformance                               |
| CSS test suite  | Style/layout conformance                             |

Unit tests stay inside each crate (`cargo test -p <crate>`).

Add new integration crates or test binaries under this directory as vertical slices land.
