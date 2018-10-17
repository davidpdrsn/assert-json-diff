pub trait Indent {
    fn indent(&self, level: u32) -> String;
}

impl<T> Indent for T
where
    T: ToString,
{
    fn indent(&self, level: u32) -> String {
        let mut indent = String::new();
        for _ in 0..level {
            indent.push_str(" ");
        }

        self.to_string()
            .lines()
            .map(|line| format!("{}{}", indent, line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent() {
        assert_eq!("  foo", "foo".indent(2));
        assert_eq!("  foo\n  bar", "foo\nbar".indent(2));
    }
}
