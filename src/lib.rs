use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Context,
    Left,
    Common,
    Right,
}

pub fn run(file: String) -> anyhow::Result<String> {
    use std::fmt::Write;
    let mut output = String::new();
    let mut state = State::Context;
    let mut conflict = Conflict::default();
    for l in file.lines() {
        if l.len() >= 7 {
            match &l[..7] {
                "<<<<<<<" => {
                    if state == State::Context {
                        state = State::Left;
                        continue;
                    }
                }
                "|||||||" => {
                    if state == State::Left {
                        state = State::Common;
                        continue;
                    }
                }
                "=======" => {
                    if state == State::Common {
                        state = State::Right;
                        continue;
                    }
                }
                ">>>>>>>" => {
                    if state == State::Right {
                        state = State::Context;

                        conflict.minimise();
                        write!(output, "{conflict}")?;
                        conflict.clear();

                        continue;
                    }
                }
                _ => (),
            }
        }
        match state {
            State::Left => conflict.left.push(l.to_string()),
            State::Common => conflict.common.push(l.to_string()),
            State::Right => conflict.right.push(l.to_string()),
            State::Context => writeln!(output, "{l}")?,
        }
    }
    match state {
        State::Context => (),
        State::Left => {
            writeln!(output, "<<<<<<<")?;
            for l in conflict.left {
                writeln!(output, "{l}")?;
            }
        }
        State::Common => {
            writeln!(output, "<<<<<<<")?;
            for l in conflict.left {
                writeln!(output, "{l}")?;
            }
            writeln!(output, "|||||||")?;
            for l in conflict.common {
                writeln!(output, "{l}")?;
            }
        }
        State::Right => {
            writeln!(output, "<<<<<<<")?;
            for l in conflict.left {
                writeln!(output, "{l}")?;
            }
            writeln!(output, "|||||||")?;
            for l in conflict.common {
                writeln!(output, "{l}")?;
            }
            writeln!(output, "=======")?;
            for l in conflict.right {
                writeln!(output, "{l}")?;
            }
        }
    }
    if let Some(c) = file.as_bytes().last() {
        if *c != b'\n' {
            output.remove(output.len() - 1);
        }
    }
    Ok(output)
}

#[derive(Default, Debug, Clone)]
struct Conflict {
    pre: Vec<String>,
    left: Vec<String>,
    common: Vec<String>,
    right: Vec<String>,
    post: Vec<String>,
}

impl Display for Conflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.pre {
            writeln!(f, "{l}")?;
        }
        if !self.is_resolved() {
            writeln!(f, "<<<<<<<")?;
            for l in &self.left {
                writeln!(f, "{l}")?;
            }
            writeln!(f, "|||||||")?;
            for l in &self.common {
                writeln!(f, "{l}")?;
            }
            writeln!(f, "=======")?;
            for l in &self.right {
                writeln!(f, "{l}")?;
            }
            writeln!(f, ">>>>>>>")?;
        }
        for l in &self.post {
            writeln!(f, "{l}")?;
        }
        Ok(())
    }
}

impl Conflict {
    fn minimise(&mut self) {
        let mut prefix = 0;
        for ((l, c), r) in self
            .left
            .iter()
            .zip(self.common.iter())
            .zip(self.right.iter())
        {
            if l == c && c == r {
                prefix += 1;
            } else {
                break;
            }
        }
        self.pre.extend(self.left.drain(..prefix));
        self.common.drain(..prefix);
        self.right.drain(..prefix);

        let mut suffix = 0;
        for ((l, c), r) in self
            .left
            .iter()
            .rev()
            .zip(self.common.iter().rev())
            .zip(self.right.iter().rev())
        {
            if l == c && c == r {
                suffix += 1;
            } else {
                break;
            }
        }
        self.post
            .extend(self.left.drain(self.left.len() - suffix..));
        self.common.drain(self.common.len() - suffix..);
        self.right.drain(self.right.len() - suffix..);

        if self.left == self.common {
            self.pre.extend(self.right.drain(..));
            self.left.clear();
            self.common.clear();
        }
        if self.right == self.common {
            self.pre.extend(self.left.drain(..));
            self.right.clear();
            self.common.clear();
        }
    }

    fn is_resolved(&self) -> bool {
        self.left.is_empty() && self.common.is_empty() && self.right.is_empty()
    }

    fn clear(&mut self) {
        self.pre.clear();
        self.left.clear();
        self.common.clear();
        self.right.clear();
        self.post.clear();
    }
}
