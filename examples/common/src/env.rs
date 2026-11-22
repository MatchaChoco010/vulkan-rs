use anyhow::{Context, Result, anyhow, bail, ensure};
use std::env::VarError;

/// The environment variable `name` parsed with `parse`, or `None` when it is not set.
/// A value that `parse` rejects, the empty one included, is an error that names the
/// variable, never a silent default.
pub fn parsed_var<T>(name: &str, parse: impl FnOnce(&str) -> Result<T>) -> Result<Option<T>> {
    match std::env::var(name) {
        Err(VarError::NotPresent) => Ok(None),
        Err(VarError::NotUnicode(value)) => bail!("{name}={value:?} is not valid Unicode"),
        Ok(value) => parse(&value)
            .map(Some)
            .with_context(|| format!("{name}={value:?}")),
    }
}

/// The value of the choice named `value`. An error that lists the names otherwise.
pub fn one_of<T: Copy>(value: &str, choices: &[(&str, T)]) -> Result<T> {
    choices
        .iter()
        .find(|(name, _)| *name == value)
        .map(|&(_, choice)| choice)
        .ok_or_else(|| {
            let names = choices
                .iter()
                .map(|(name, _)| format!("`{name}`"))
                .collect::<Vec<_>>();
            anyhow!("expected one of {}", names.join(", "))
        })
}

/// A count of at least 1, written in decimal digits.
pub fn positive_count(value: &str) -> Result<u32> {
    let well_formed = !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit());
    ensure!(well_formed, "expected a whole number of at least 1");
    let count = value
        .parse::<u32>()
        .context("expected a whole number of at least 1 that fits in 32 bits")?;
    ensure!(count > 0, "expected a whole number of at least 1");
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHOICES: [(&str, u8); 3] = [("fifo", 0), ("fifo_relaxed", 1), ("mailbox", 2)];

    /// Strings near the accepted spellings: other cases, spaces, prefixes, suffixes.
    fn near_misses(names: &[&str]) -> Vec<String> {
        let mut values = vec![String::new(), " ".to_owned(), "default".to_owned()];
        for name in names {
            values.push(name.to_uppercase());
            values.push(format!(" {name}"));
            values.push(format!("{name} "));
            values.push(format!("{name}x"));
            values.push(name[1..].to_owned());
            values.push(name[..name.len() - 1].to_owned());
        }
        values.retain(|value| !names.contains(&value.as_str()));
        values
    }

    #[test]
    fn every_choice_is_found_by_its_name_and_nothing_else_is_accepted() {
        for (name, value) in CHOICES {
            assert_eq!(one_of(name, &CHOICES).unwrap(), value, "{name}");
        }
        let names = CHOICES.map(|(name, _)| name);
        for value in near_misses(&names) {
            let error = one_of(&value, &CHOICES).unwrap_err().to_string();
            for name in names {
                assert!(
                    error.contains(&format!("`{name}`")),
                    "{value:?}: {error:?} does not list {name}"
                );
            }
        }
    }

    #[test]
    fn every_positive_count_round_trips_and_nothing_else_is_accepted() {
        let mut state = 0x2545_f491_4f6c_dd1d_u64;
        let counts = [1, 2, 9, 10, 99, 1000, u32::MAX]
            .into_iter()
            .chain(std::iter::repeat_with(move || {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                (state as u32).max(1)
            }))
            .take(500);
        for count in counts {
            assert_eq!(positive_count(&count.to_string()).unwrap(), count);
        }
        for value in [
            "",
            "0",
            "00",
            "-1",
            "+1",
            " 1",
            "1 ",
            "1.0",
            "1e3",
            "0x10",
            "abc",
            "١",
            "4294967296",
        ] {
            assert!(positive_count(value).is_err(), "{value:?} was accepted");
        }
    }
}
