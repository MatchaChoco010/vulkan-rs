#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Cell {
    Lines(Vec<String>),
    Unsupported,
}

impl Cell {
    pub(crate) fn single_line(&self) -> Option<&str> {
        match self {
            Cell::Lines(lines) if lines.len() == 1 => Some(&lines[0]),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Table {
    pub(crate) title: Option<String>,
    pub(crate) header: Option<Vec<Cell>>,
    pub(crate) rows: Vec<Vec<Cell>>,
}

struct RawCell {
    style: Option<char>,
    line: usize,
    starts_its_line: bool,
    content: String,
}

fn attribute_entries(list: &str) -> Vec<String> {
    let mut entries = Vec::new();
    let mut entry = String::new();
    let mut quoted = false;
    for c in list.chars() {
        match c {
            '"' => quoted = !quoted,
            ',' if !quoted => entries.push(std::mem::take(&mut entry).trim().to_string()),
            c => entry.push(c),
        }
    }
    entries.push(entry.trim().to_string());
    entries
}

struct ColumnSpec {
    columns: usize,
    styled: bool,
}

fn column_spec(record: &str) -> Option<ColumnSpec> {
    let (columns, rest) = match record.split_once('*') {
        Some((multiplier, rest)) if multiplier.chars().all(|c| c.is_ascii_digit()) => {
            (multiplier.parse().ok()?, rest)
        }
        Some(_) => return None,
        None => (1, record),
    };
    let alignment = ['<', '^', '>'];
    let rest = rest.strip_prefix(alignment).unwrap_or(rest);
    let rest = match rest.strip_prefix('.') {
        Some(vertical) => vertical.strip_prefix(alignment)?,
        None => rest,
    };
    let after_digits = rest.trim_start_matches(|c: char| c.is_ascii_digit());
    let rest = match rest.strip_prefix('~') {
        Some(after_the_width) => after_the_width,
        None if after_digits.len() < rest.len() => {
            after_digits.strip_prefix('%').unwrap_or(after_digits)
        }
        None => rest,
    };
    let mut style = rest.chars();
    match (style.next(), style.next()) {
        (None, _) => Some(ColumnSpec {
            columns,
            styled: false,
        }),
        (Some(style), None) if style.is_ascii_lowercase() => Some(ColumnSpec {
            columns,
            styled: true,
        }),
        _ => None,
    }
}

fn column_count(cols: &str) -> Option<usize> {
    let cols = cols.replace(' ', "");
    if let Ok(count) = cols.parse::<usize>()
        && count.to_string() == cols
    {
        return Some(count);
    }
    if cols.is_empty() {
        return Some(0);
    }
    let separator = if cols.contains(',') { ',' } else { ';' };
    let mut count = 0usize;
    for record in cols.split(separator) {
        match column_spec(record) {
            Some(ColumnSpec { styled: true, .. }) => return None,
            Some(ColumnSpec { columns, .. }) => count = count.checked_add(columns)?,
            None => {}
        }
    }
    Some(count)
}

#[derive(Debug, PartialEq, Eq)]
struct CellSpec {
    spans: bool,
    style: Option<char>,
}

fn cell_spec(token: &str) -> Option<CellSpec> {
    let factor_len = token
        .find(|c: char| !(c == '.' || c.is_ascii_digit()))
        .unwrap_or(token.len());
    let factor = &token[..factor_len];
    let spans = token[factor_len..].starts_with(['+', '*'])
        && factor.matches('.').count() <= 1
        && factor
            .split_once('.')
            .map_or(!factor.is_empty(), |(columns, rows)| {
                !columns.is_empty() || !rows.is_empty()
            });
    let mut rest = if spans {
        &token[factor_len + 1..]
    } else {
        token
    };
    let alignment = ['<', '^', '>'];
    let horizontal = rest.starts_with(alignment);
    if horizontal {
        rest = &rest[1..];
    }
    if let Some(after) = rest.strip_prefix('.') {
        let vertical = after.starts_with(alignment);
        if !horizontal && !vertical {
            return None;
        }
        rest = if vertical { &after[1..] } else { after };
    }
    let mut chars = rest.chars();
    let style = match (chars.next(), chars.next()) {
        (None, _) => None,
        (Some(style), None) if style.is_ascii_lowercase() => Some(style),
        _ => return None,
    };
    (!token.is_empty()).then_some(CellSpec { spans, style })
}

fn raw_cells(content: &[&str]) -> Option<Vec<RawCell>> {
    let mut cells = Vec::<RawCell>::new();
    for (line_index, line) in content.iter().enumerate() {
        let mut segment_start = 0;
        let mut escaped = false;
        for (index, c) in line.char_indices() {
            if escaped {
                escaped = false;
                continue;
            }
            if c == '\\' {
                escaped = true;
                continue;
            }
            if c != '|' {
                continue;
            }
            let before = &line[segment_start..index];
            let after_space = before
                .char_indices()
                .rev()
                .find(|(_, c)| *c == ' ' || *c == '\t')
                .map(|(space, c)| space + c.len_utf8());
            let token_start = match after_space {
                Some(start) => Some(start),
                None if segment_start == 0 => Some(0),
                None => None,
            };
            let spec = token_start.and_then(|start| Some((start, cell_spec(&before[start..])?)));
            let (text, style) = match spec {
                Some((_, CellSpec { spans: true, .. })) => return None,
                Some((start, CellSpec { style, .. })) => (&before[..start], style),
                None => (before, None),
            };
            match cells.last_mut() {
                Some(cell) => cell.content.push_str(text),
                None if !text.trim().is_empty() => return None,
                None => {}
            }
            cells.push(RawCell {
                style,
                line: line_index,
                starts_its_line: segment_start == 0 && text.trim().is_empty(),
                content: String::new(),
            });
            segment_start = index + 1;
        }
        match cells.last_mut() {
            Some(cell) => {
                cell.content.push_str(&line[segment_start..]);
                cell.content.push('\n');
            }
            None if !line.trim().is_empty() => return None,
            None => {}
        }
    }
    Some(cells)
}

fn is_block_attribute(line: &str) -> bool {
    line.starts_with('[') && line.ends_with(']') && !line.starts_with("[[")
}

fn is_ordered_list_item(line: &str) -> bool {
    line.trim_start_matches(|c: char| c.is_ascii_digit())
        .starts_with('.')
}

fn asciidoc_lines(content: &str) -> Option<Vec<String>> {
    let lines = content.lines().map(str::trim).collect::<Vec<_>>();
    let mut out = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        if line == "[latexmath]" && lines.get(index + 1) == Some(&"++++") {
            let close = (index + 2..lines.len()).find(|close| lines[*close] == "++++")?;
            let body = lines[index + 2..close]
                .iter()
                .flat_map(|line| line.split_whitespace())
                .collect::<Vec<_>>()
                .join(" ");
            out.push(format!("latexmath:[{body}]"));
            index = close + 1;
            continue;
        }
        let starts_a_block = is_block_attribute(line)
            || ["----", "====", "....", "++++", "****", "--"].contains(&line)
            || line.starts_with(['*', '-', '.'])
            || line.starts_with("include::")
            || line.starts_with("image::")
            || line.starts_with('!')
            || is_ordered_list_item(line);
        if starts_a_block {
            return None;
        }
        out.push(line.to_string());
        index += 1;
    }
    Some(out)
}

fn cell(raw: &RawCell) -> Cell {
    let content = raw.content.replace("\\|", "|");
    let lines = match raw.style {
        None | Some('d') => Some(
            content
                .lines()
                .map(|line| line.trim().to_string())
                .collect(),
        ),
        Some('a') => asciidoc_lines(&content),
        Some(_) => None,
    };
    let Some(lines) = lines else {
        return Cell::Unsupported;
    };
    let first = lines.iter().position(|line| !line.is_empty());
    let last = lines.iter().rposition(|line| !line.is_empty());
    let (Some(first), Some(last)) = (first, last) else {
        return Cell::Lines(Vec::new());
    };
    let mut out = Vec::<String>::new();
    let mut continued = false;
    for line in &lines[first..=last] {
        if line.is_empty() || line == "+" {
            return Cell::Unsupported;
        }
        let (text, breaks) = match line.strip_suffix(" +") {
            Some(text) => (text.trim_end(), true),
            None => (line.as_str(), false),
        };
        match out.last_mut() {
            Some(previous) if continued => {
                previous.push('\n');
                previous.push_str(text);
            }
            _ => out.push(text.to_string()),
        }
        continued = !breaks;
    }
    Cell::Lines(out)
}

pub(crate) fn read(text: &str) -> Option<Table> {
    let lines = text.lines().collect::<Vec<_>>();
    let open = lines.iter().position(|line| {
        let line = line.trim_end();
        line.len() >= 4 && line.starts_with('|') && line[1..].chars().all(|c| c == '=')
    })?;
    let delimiter = lines[open].trim_end();
    let close = open
        + 1
        + lines[open + 1..]
            .iter()
            .position(|line| line.trim_end() == delimiter)?;

    let mut title = None;
    let mut columns = None;
    let mut header_option = None;
    for line in &lines[..open] {
        let line = line.trim();
        if line.starts_with("[[") {
            continue;
        } else if let Some(text) = line.strip_prefix('.') {
            title = Some(text.trim().to_string());
        } else if let Some(list) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            for entry in attribute_entries(list) {
                let (key, value) = entry.split_once('=').unwrap_or((&entry, ""));
                let options = match key.trim() {
                    "cols" => {
                        columns = Some(column_count(value.trim().trim_matches('"'))?)
                            .filter(|count| *count > 0);
                        continue;
                    }
                    "format" if value.trim().trim_matches('"') != "psv" => return None,
                    "separator" => return None,
                    "options" | "opts" => value.trim().trim_matches('"').to_string(),
                    option if option.starts_with('%') => option.to_string(),
                    _ => continue,
                };
                for option in options.split(['%', ',']).map(str::trim) {
                    match option {
                        "header" => header_option = Some(true),
                        "noheader" => header_option = Some(false),
                        "footer" => return None,
                        _ => {}
                    }
                }
            }
        } else {
            return None;
        }
    }

    let content = lines[open + 1..close]
        .iter()
        .copied()
        .filter(|line| !line.starts_with("//") || line.starts_with("///"))
        .collect::<Vec<_>>();
    if content.iter().any(|line| line.starts_with("////")) {
        return None;
    }
    let cells = raw_cells(&content)?;
    let first_line = cells.first()?.line;
    let columns = match columns {
        Some(columns) => columns,
        None => cells.iter().filter(|cell| cell.line == first_line).count(),
    };
    if cells.len() % columns != 0 {
        return None;
    }
    let is_blank = |line: &&str| line.trim().is_empty();
    let after_the_gap = content.iter().skip(1).position(|line| !is_blank(line));
    let implicit_header = first_line == 0
        && content.get(1).is_some_and(is_blank)
        && after_the_gap.is_none_or(|skipped| {
            cells
                .iter()
                .any(|cell| cell.line == skipped + 1 && cell.starts_its_line)
        });
    let mut rows = cells
        .chunks(columns)
        .map(|row| row.iter().map(cell).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let header = header_option
        .unwrap_or(implicit_header)
        .then(|| rows.remove(0));
    Some(Table {
        title,
        header,
        rows,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::proptest_config;
    use proptest::prelude::*;

    fn lines(lines: &[&str]) -> Cell {
        Cell::Lines(lines.iter().map(|line| line.to_string()).collect())
    }

    fn table(text: &[&str]) -> Table {
        read(&text.join("\n")).unwrap_or_else(|| panic!("a table: {text:?}"))
    }

    #[test]
    fn a_table_with_a_header_option_and_column_widths_has_a_title_header_and_rows() {
        let table = table(&[
            "[[framebuffer-blend-factors-table]]",
            ".Blend Factors",
            "[width=\"100%\",options=\"header\",align=\"center\",cols=\"59%,28%,13%\"]",
            "|====",
            "|elink:VkBlendFactor        | RGB [eq]#(S~r~,S~g~,S~b~)# | Alpha",
            "|ename:VK_BLEND_FACTOR_ZERO | [eq]#(0,0,0)#              | [eq]#0#",
            "|ename:VK_BLEND_FACTOR_ONE  | [eq]#(1,1,1)#              | [eq]#1#",
            "|====",
        ]);
        assert_eq!(table.title.as_deref(), Some("Blend Factors"));
        assert_eq!(
            table.header,
            Some(vec![
                lines(&["elink:VkBlendFactor"]),
                lines(&["RGB [eq]#(S~r~,S~g~,S~b~)#"]),
                lines(&["Alpha"]),
            ])
        );
        assert_eq!(
            table.rows,
            vec![
                vec![
                    lines(&["ename:VK_BLEND_FACTOR_ZERO"]),
                    lines(&["[eq]#(0,0,0)#"]),
                    lines(&["[eq]#0#"]),
                ],
                vec![
                    lines(&["ename:VK_BLEND_FACTOR_ONE"]),
                    lines(&["[eq]#(1,1,1)#"]),
                    lines(&["[eq]#1#"]),
                ],
            ]
        );
    }

    #[test]
    fn cells_span_lines_and_hard_line_breaks_separate_their_lines() {
        let table = table(&[
            "[width=\"100%\",cols=\"45%,30%,25%\",options=\"header\",align=\"center\"]",
            "|====",
            "|elink:VkBlendOp | RGB Components | Alpha Component",
            "",
            "|ename:VK_BLEND_OP_ADD",
            "| [eq]#R = R~s0~ {plus} R~d~# +",
            "  [eq]#G = G~s0~# +",
            "  [eq]#B = B~s0~ {plus}",
            "  B~d~#",
            "| [eq]#A = A~s0~#",
            "",
            "|ename:VK_BLEND_OP_MIN",
            "| [eq]#R = min(R~s0~,R~d~)#",
            "| [eq]#A = min(A~s0~,A~d~)#",
            "|====",
        ]);
        assert_eq!(table.title, None);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(
            table.rows[0][1],
            lines(&[
                "[eq]#R = R~s0~ {plus} R~d~#",
                "[eq]#G = G~s0~#",
                "[eq]#B = B~s0~ {plus}\nB~d~#",
            ])
        );
        assert_eq!(table.rows[0][2], lines(&["[eq]#A = A~s0~#"]));
    }

    #[test]
    fn asciidoc_cells_turn_latexmath_blocks_into_inline_math() {
        let table = table(&[
            ".Advanced Blend Overlap Modes",
            "[width=\"80%\",options=\"header\"]",
            "|====",
            "| Overlap Mode                             | Weighting Equations",
            "| ename:VK_BLEND_OVERLAP_UNCORRELATED_EXT a|",
            "[latexmath]",
            "++++",
            "                  \\begin{aligned}",
            "                    p_0(A_s,A_d)    & = A_sA_d \\\\",
            "                  \\end{aligned}",
            "++++",
            "",
            "| ename:VK_BLEND_OVERLAP_CONJOINT_EXT     a|",
            "* a list item",
            "|====",
        ]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(
            table.rows[0],
            vec![
                lines(&["ename:VK_BLEND_OVERLAP_UNCORRELATED_EXT"]),
                lines(&[
                    "latexmath:[\\begin{aligned} p_0(A_s,A_d) & = A_sA_d \\\\ \\end{aligned}]"
                ]),
            ]
        );
        assert_eq!(table.rows[1][1], Cell::Unsupported);
    }

    #[test]
    fn without_column_widths_the_first_line_counts_the_columns() {
        let table = table(&[
            "[options=\"header\"]",
            "|====",
            "| Shading Rate | Width | Height",
            "| ename:VK_SHADING_RATE_A | 0 | 0",
            "// a comment line",
            "| ename:VK_SHADING_RATE_B | 1 | 2",
            "|====",
        ]);
        assert_eq!(table.header.as_ref().map(Vec::len), Some(3));
        assert_eq!(
            table.rows,
            vec![
                vec![
                    lines(&["ename:VK_SHADING_RATE_A"]),
                    lines(&["0"]),
                    lines(&["0"]),
                ],
                vec![
                    lines(&["ename:VK_SHADING_RATE_B"]),
                    lines(&["1"]),
                    lines(&["2"]),
                ],
            ]
        );
    }

    #[test]
    fn rows_with_empty_cells_and_escaped_bars_keep_their_columns() {
        let table = table(&[
            "[cols=\"67%,33%\",options=\"header\"]",
            "|====",
            "|*Common Tokens* | *Command Data*",
            "|ename:VK_TOKEN_A | `u32[]` raw data",
            "|*Compute Tokens*|",
            "|ename:VK_TOKEN_B | a \\| b",
            "|====",
        ]);
        assert_eq!(
            table.header,
            Some(vec![
                lines(&["*Common Tokens*"]),
                lines(&["*Command Data*"])
            ])
        );
        assert_eq!(
            table.rows[1],
            vec![lines(&["*Compute Tokens*"]), lines(&[])]
        );
        assert_eq!(table.rows[2][1], lines(&["a | b"]));
    }

    #[test]
    fn a_first_line_followed_by_an_empty_line_is_an_implicit_header() {
        let implicit = table(&[
            "[cols=\"1,1\"]",
            "|====",
            "|Mode |Result",
            "",
            "|ename:VK_A |one",
            "|====",
        ]);
        assert_eq!(
            implicit.header,
            Some(vec![lines(&["Mode"]), lines(&["Result"])])
        );
        let none = table(&[
            "[width=\"70%\"]",
            "|====",
            "|ename:VK_A",
            "|ename:VK_B",
            "|====",
        ]);
        assert_eq!(none.header, None);
        assert_eq!(none.rows.len(), 2);
        let refused = table(&[
            "[cols=\"1,1\",options=\"noheader\"]",
            "|====",
            "|Mode |Result",
            "",
            "|ename:VK_A |one",
            "|====",
        ]);
        assert_eq!(refused.header, None);
        assert_eq!(refused.rows.len(), 2);
    }

    fn header_and_rows(content: &[&str]) -> (Option<Vec<Cell>>, Vec<Vec<Cell>>) {
        let mut text = vec!["[cols=\"1,1\"]", "|==="];
        text.extend(content);
        text.push("|===");
        let table = table(&text);
        (table.header, table.rows)
    }

    fn row(cells: &[&str]) -> Vec<Cell> {
        cells.iter().map(|cell| lines(&[cell])).collect()
    }

    #[test]
    fn the_implicit_header_is_the_first_row_however_the_cells_are_spread_over_the_lines() {
        assert_eq!(
            header_and_rows(&["|a |b |c |d", "", "|e |f"]),
            (
                Some(row(&["a", "b"])),
                vec![row(&["c", "d"]), row(&["e", "f"])]
            )
        );
        assert_eq!(
            header_and_rows(&["|a |b |c", "", "|d"]),
            (Some(row(&["a", "b"])), vec![row(&["c", "d"])])
        );
        assert_eq!(
            header_and_rows(&["|a", "", "|b |e |f"]),
            (Some(row(&["a", "b"])), vec![row(&["e", "f"])])
        );
        assert_eq!(
            header_and_rows(&["|a |b", ""]),
            (Some(row(&["a", "b"])), vec![])
        );
        for gap in [
            vec![""],
            vec!["", ""],
            vec!["  "],
            vec!["// a comment", ""],
            vec!["", "// a comment"],
        ] {
            let mut content = vec!["|a |b"];
            content.extend(&gap);
            content.push("|c |d");
            assert_eq!(
                header_and_rows(&content),
                (Some(row(&["a", "b"])), vec![row(&["c", "d"])]),
                "{gap:?}"
            );
        }
        for next in ["  |c |d", "\t|c |d", "d|c |d", "^.^|c |d"] {
            assert_eq!(
                header_and_rows(&["|a |b", "", next]),
                (Some(row(&["a", "b"])), vec![row(&["c", "d"])]),
                "{next:?}"
            );
        }
    }

    #[test]
    fn a_first_line_that_no_empty_line_follows_or_whose_cell_goes_on_is_no_header() {
        for content in [
            vec!["|a", "|b", "", "|c |d"],
            vec!["|a |b", "|c |d", ""],
            vec!["|a |b", "// a comment", "|c |d"],
            vec!["", "|a |b", "", "|c |d"],
            vec!["  ", "|a |b", "", "|c |d"],
        ] {
            assert_eq!(
                header_and_rows(&content),
                (None, vec![row(&["a", "b"]), row(&["c", "d"])]),
                "{content:?}"
            );
        }
        for next in ["more of b", "x |c |d", "x ||d", "x | |d"] {
            let (header, rows) = header_and_rows(&["|a |b", "", next, "|c |d"]);
            assert_eq!(header, None, "{next:?}");
            assert_eq!(rows[0][0], lines(&["a"]), "{next:?}");
            assert_eq!(rows[0][1], Cell::Unsupported, "{next:?}");
        }
    }

    #[test]
    fn column_specifications_count_multiplied_columns() {
        assert_eq!(column_count("59%,28%,13%"), Some(3));
        assert_eq!(column_count("<35,<23"), Some(2));
        assert_eq!(column_count("3*,2"), Some(4));
        assert_eq!(column_count("1, 2 ,3"), Some(3));
        assert_eq!(column_count(">.^1,.>3,^~,2*<.>20%"), Some(5));
        assert_eq!(
            column_count("1,2a"),
            None,
            "a column style changes how its cells are read"
        );
        assert_eq!(column_count("3*a"), None);
        assert_eq!(column_count("<1,^.^2l"), None);
    }

    #[test]
    fn column_specifications_are_separated_by_commas_or_else_by_semicolons() {
        assert_eq!(column_count("2;1"), Some(2));
        assert_eq!(column_count("2;1;3*"), Some(5));
        assert_eq!(column_count("1;2,3"), Some(1));
        assert_eq!(column_count("1,2;3,4"), Some(2));
        assert_eq!(column_count("1;2a,3"), Some(1));
    }

    #[test]
    fn an_empty_column_specification_is_a_column_and_one_that_is_none_is_passed_over() {
        assert_eq!(column_count("1,,2"), Some(3));
        assert_eq!(column_count("1,"), Some(2));
        assert_eq!(column_count(","), Some(2));
        assert_eq!(column_count(";"), Some(2));
        for (cols, columns) in [
            ("*", 0),
            ("1,*", 1),
            ("x*,1", 1),
            ("1,1.5,2", 2),
            ("1,%,2", 2),
            ("1,.,2", 2),
            ("1,<.,2", 2),
            ("1,2*3*,2", 2),
            ("1,2A,2", 2),
            ("1,2ab,2", 2),
            ("1,-1,2", 2),
            ("1,+2*,2", 2),
            ("1,-2*,2", 2),
            ("1,99999999999999999999999999*,2", 2),
        ] {
            assert_eq!(column_count(cols), Some(columns), "{cols:?}");
        }
    }

    #[test]
    fn columns_multiplied_by_zero_do_not_count_and_no_columns_leave_the_count_to_the_first_line() {
        assert_eq!(column_count("0*"), Some(0));
        assert_eq!(column_count("0*,1"), Some(1));
        assert_eq!(column_count("2*,0*"), Some(2));
        assert_eq!(column_count("0,0"), Some(2));
        assert_eq!(column_count(""), Some(0));
        assert_eq!(column_count("  "), Some(0));
        for cols in ["0*", "0", "", "*", "0*;0*"] {
            let unspecified = table(&[
                &format!("[cols=\"{cols}\"]"),
                "|===",
                "|a |b |c",
                "|d |e |f",
                "|===",
            ]);
            assert_eq!(
                unspecified.rows,
                vec![row(&["a", "b", "c"]), row(&["d", "e", "f"])],
                "{cols:?}"
            );
        }
        let one_column = table(&["[cols=\"0*,1\"]", "|===", "|a |b", "|==="]);
        assert_eq!(one_column.rows, vec![row(&["a"]), row(&["b"])]);
    }

    #[test]
    fn tables_that_cannot_be_understood_are_refused() {
        let spanning = [
            "[cols=\"1,1\",options=\"header\"]",
            "|====",
            "|Mode |Result",
            "2+|ename:VK_A",
            "|====",
        ];
        assert_eq!(read(&spanning.join("\n")), None);
        for specifier in ["2+", ".2+", "2.3+", "3*", "2+a"] {
            let text = format!("[cols=\"1,1\"]\n|====\n|ename:VK_A {specifier}|one\n|====");
            assert_eq!(read(&text), None, "{specifier}");
        }
        let uneven = [
            "[cols=\"1,1\",options=\"header\"]",
            "|====",
            "|Mode |Result",
            "|ename:VK_A",
            "|====",
        ];
        assert_eq!(read(&uneven.join("\n")), None);
        let text_before_the_first_cell = ["|====", "text |ename:VK_A", "|===="];
        assert_eq!(read(&text_before_the_first_cell.join("\n")), None);
        let unterminated = ["|====", "|ename:VK_A"];
        assert_eq!(read(&unterminated.join("\n")), None);
    }

    #[test]
    fn tables_with_a_footer_or_styled_columns_are_refused() {
        let footer = [
            "[options=\"header,footer\"]",
            "|====",
            "|Mode |Result",
            "|ename:VK_A |one",
            "|ename:VK_B |total",
            "|====",
        ];
        assert_eq!(read(&footer.join("\n")), None);
        let styled = [
            "[cols=\"1,1a\",options=\"header\"]",
            "|====",
            "|Mode |Result",
            "|ename:VK_A |",
            "[latexmath]",
            "++++",
            "x",
            "++++",
            "|====",
        ];
        assert_eq!(read(&styled.join("\n")), None);
    }

    #[test]
    fn asciidoc_cells_with_ordered_lists_or_nested_tables_are_unsupported() {
        let table = table(&[
            "[cols=\"1,1\",options=\"header\"]",
            "|====",
            "|Mode |Result",
            "|ename:VK_A a|",
            "1. first",
            "2. second",
            "|ename:VK_B a|",
            "!===",
            "!x !y",
            "!===",
            "|ename:VK_C a|plain text",
            "|====",
        ]);
        assert_eq!(table.rows[0][1], Cell::Unsupported);
        assert_eq!(table.rows[1][1], Cell::Unsupported);
        assert_eq!(table.rows[2][1], lines(&["plain text"]));
    }

    #[test]
    fn a_single_number_of_columns_spreads_the_columns_equally() {
        assert_eq!(column_count("3"), Some(3));
        assert_eq!(column_count(" 2 "), Some(2));
        assert_eq!(column_count("0"), Some(0));
        assert_eq!(column_count("03"), Some(1));
        assert_eq!(column_count("007"), Some(1));
        let one_column = table(&["[cols=\"03\"]", "|===", "|a|b", "|==="]);
        assert_eq!(
            one_column.rows,
            vec![vec![lines(&["a"])], vec![lines(&["b"])]]
        );
        let unspecified = table(&["[cols=\"0\"]", "|===", "|a|b", "|c|d", "|==="]);
        assert_eq!(
            unspecified.rows,
            vec![
                vec![lines(&["a"]), lines(&["b"])],
                vec![lines(&["c"]), lines(&["d"])]
            ]
        );
        let table = table(&[
            "[cols=\"2\",options=\"header\"]",
            "|===",
            "|h1|h2",
            "|a|b",
            "|===",
        ]);
        assert_eq!(table.header, Some(vec![lines(&["h1"]), lines(&["h2"])]));
        assert_eq!(table.rows, vec![vec![lines(&["a"]), lines(&["b"])]]);
    }

    #[test]
    fn a_one_character_cell_between_separators_is_content_not_a_specifier() {
        let letters = table(&["|===", "|a|b|c", "|d|e|f", "|==="]);
        assert_eq!(
            letters.rows,
            vec![
                vec![lines(&["a"]), lines(&["b"]), lines(&["c"])],
                vec![lines(&["d"]), lines(&["e"]), lines(&["f"])],
            ]
        );
        let specifier_like = table(&["[cols=\"1,1,1\"]", "|===", "|x |s|^", "|m|<.^|.", "|==="]);
        assert_eq!(
            specifier_like.rows,
            vec![
                vec![lines(&["x"]), lines(&["s"]), lines(&["^"])],
                vec![lines(&["m"]), lines(&["<.^"]), lines(&["."])],
            ]
        );
    }

    #[test]
    fn specifiers_after_a_space_or_at_the_start_of_a_line_apply_to_the_next_cell() {
        let aligned = table(&[
            "[cols=\"1,1,1\"]",
            "|===",
            "|ename:VK_A ^|centered >.^d|bottom",
            "^.|top .>|middle <.<|left",
            "|===",
        ]);
        assert_eq!(
            aligned.rows,
            vec![
                vec![
                    lines(&["ename:VK_A"]),
                    lines(&["centered"]),
                    lines(&["bottom"])
                ],
                vec![lines(&["top"]), lines(&["middle"]), lines(&["left"])],
            ]
        );
        let styled = table(&[
            "[cols=\"1,1\"]",
            "|===",
            "|ename:VK_A x|unknown style",
            "|===",
        ]);
        assert_eq!(styled.rows[0][1], Cell::Unsupported);
    }

    #[test]
    fn cell_specifiers_follow_the_asciidoctor_grammar() {
        let spec = |spans, style| Some(CellSpec { spans, style });
        assert_eq!(cell_spec("a"), spec(false, Some('a')));
        assert_eq!(cell_spec("^"), spec(false, None));
        assert_eq!(cell_spec("^."), spec(false, None));
        assert_eq!(cell_spec(".^"), spec(false, None));
        assert_eq!(cell_spec("<.>m"), spec(false, Some('m')));
        assert_eq!(cell_spec("2+"), spec(true, None));
        assert_eq!(cell_spec(".3+"), spec(true, None));
        assert_eq!(cell_spec("2.+^s"), spec(true, Some('s')));
        assert_eq!(cell_spec("3*e"), spec(true, Some('e')));
        for content in ["", ".", "+", "2", "ab", "A", "^^", "<^", ".+", "x2", "*"] {
            assert_eq!(cell_spec(content), None, "{content:?}");
        }
    }

    #[test]
    fn tables_in_other_formats_or_with_other_separators_are_refused() {
        for attributes in [
            "[format=csv]",
            "[format=\"dsv\",options=\"header\"]",
            "[separator=!]",
            "[cols=\"1,1\",separator=\";\"]",
        ] {
            let text = format!("{attributes}\n|===\n|a|b\n|===");
            assert_eq!(read(&text), None, "{attributes}");
        }
        assert!(read("[format=psv]\n|===\n|a|b\n|===").is_some());
    }

    #[derive(Clone, Debug)]
    enum Content {
        Empty,
        Char(char),
        Words(String),
        Wrapped(String, String),
        Broken(String, String),
    }

    impl Content {
        fn expected(&self) -> Cell {
            match self {
                Content::Empty => lines(&[]),
                Content::Char(c) => lines(&[&c.to_string()]),
                Content::Words(words) => lines(&[words]),
                Content::Wrapped(first, second) => lines(&[&format!("{first}\n{second}")]),
                Content::Broken(first, second) => lines(&[first, second]),
            }
        }

        fn source(&self) -> String {
            match self {
                Content::Empty => String::new(),
                Content::Char(c) => c.to_string(),
                Content::Words(words) => words.clone(),
                Content::Wrapped(first, second) => format!("{first}\n{second}"),
                Content::Broken(first, second) => format!("{first} +\n{second}"),
            }
        }

        fn on_one_line(self) -> Self {
            match self {
                Content::Wrapped(first, second) | Content::Broken(first, second) => {
                    Content::Words(format!("{first} {second}"))
                }
                other => other,
            }
        }

        fn spans_lines(&self) -> bool {
            matches!(self, Content::Wrapped(..) | Content::Broken(..))
        }
    }

    fn content() -> impl Strategy<Value = Content> {
        let word = "[a-z]{2,8}";
        prop_oneof![
            Just(Content::Empty),
            proptest::sample::select("abcdehlmsvxz019<^>.*%~=-".chars().collect::<Vec<_>>())
                .prop_map(Content::Char),
            "[a-z]{2,8}( [a-z]{2,8}){0,2}".prop_map(Content::Words),
            (word, word).prop_map(|(a, b)| Content::Wrapped(a, b)),
            (word, word).prop_map(|(a, b)| Content::Broken(a, b)),
        ]
    }

    #[derive(Clone, Copy, Debug)]
    enum Cols {
        Absent,
        Count,
        Ones,
        Widths,
        Multiplied,
        Aligned,
    }

    fn cols_attribute(cols: Cols, columns: usize) -> Option<String> {
        let each =
            |spec: &dyn Fn(usize) -> String| (0..columns).map(spec).collect::<Vec<_>>().join(",");
        match cols {
            Cols::Absent => None,
            Cols::Count => Some(columns.to_string()),
            Cols::Ones => Some(each(&|_| "1".to_string())),
            Cols::Widths => Some(each(&|i| format!("{}%", 10 + i))),
            Cols::Multiplied => Some(format!("{columns}*")),
            Cols::Aligned => Some(each(&|i| ["<1", "^2", ">.^1", ".>3"][i % 4].to_string())),
        }
    }

    #[derive(Debug)]
    struct Generated {
        text: String,
        expected: Table,
    }

    fn generated_table() -> impl Strategy<Value = Generated> {
        (1usize..=4, 1usize..=4).prop_flat_map(|(columns, rows)| {
            (
                proptest::collection::vec(proptest::collection::vec(content(), columns), rows),
                proptest::sample::select(vec![
                    Cols::Absent,
                    Cols::Count,
                    Cols::Ones,
                    Cols::Widths,
                    Cols::Multiplied,
                    Cols::Aligned,
                ]),
                any::<bool>(),
                any::<bool>(),
                proptest::collection::vec(any::<u8>(), 1..32),
            )
                .prop_map(move |(rows, cols, header, title, choices)| {
                    print_table(columns, rows, cols, header, title, &choices)
                })
        })
    }

    fn print_table(
        columns: usize,
        mut rows: Vec<Vec<Content>>,
        cols: Cols,
        header: bool,
        title: bool,
        choices: &[u8],
    ) -> Generated {
        let mut next = 0;
        let mut choose = |range: u8| {
            next += 1;
            choices[next % choices.len()] % range
        };
        let implicit_header = header && choose(2) == 0;
        let first_row_on_one_line = matches!(cols, Cols::Absent) || implicit_header;
        let cells_before_the_gap = match cols {
            Cols::Absent => columns,
            _ => 1 + choose(columns as u8) as usize,
        };
        if first_row_on_one_line {
            rows[0] = std::mem::take(&mut rows[0])
                .into_iter()
                .map(Content::on_one_line)
                .collect();
        }
        let mut text = Vec::<String>::new();
        if title {
            text.push(".Generated Table".to_string());
        }
        let mut attributes = Vec::new();
        if let Some(cols) = cols_attribute(cols, columns) {
            attributes.push(format!("cols=\"{cols}\""));
        }
        if header && !implicit_header {
            attributes.push("options=\"header\"".to_string());
        }
        if !attributes.is_empty() {
            text.push(format!("[{}]", attributes.join(",")));
        }
        let delimiter = ["|===", "|===="][choose(2) as usize];
        text.push(delimiter.to_string());
        for (index, row) in rows.iter().enumerate() {
            let one_line = (index == 0 && first_row_on_one_line)
                || (!row.iter().any(Content::spans_lines) && choose(2) == 0);
            let mut line = String::new();
            for (column, content) in row.iter().enumerate() {
                let leading = choose(2) == 0;
                let mut trailing = choose(2) == 0;
                let last = column + 1 == row.len();
                if one_line && !last && leading && matches!(content, Content::Char(_)) {
                    trailing = true;
                }
                let cell = format!(
                    "|{}{}{}",
                    if leading { " " } else { "" },
                    content.source(),
                    if trailing { " " } else { "" }
                );
                if one_line {
                    line.push_str(&cell);
                } else {
                    text.extend(cell.lines().map(str::to_string));
                }
                if index == 0 && implicit_header && column + 1 == cells_before_the_gap {
                    text.push(std::mem::take(&mut line));
                    text.push(String::new());
                }
            }
            if !line.is_empty() {
                text.push(line);
            }
        }
        text.push(delimiter.to_string());
        let mut expected_rows = rows
            .iter()
            .map(|row| row.iter().map(Content::expected).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let expected_header = header.then(|| expected_rows.remove(0));
        Generated {
            text: text.join("\n"),
            expected: Table {
                title: title.then(|| "Generated Table".to_string()),
                header: expected_header,
                rows: expected_rows,
            },
        }
    }

    proptest! {
        #![proptest_config(proptest_config(2048))]

        #[test]
        fn generated_tables_read_back_to_their_rows_and_columns(generated in generated_table()) {
            prop_assert_eq!(
                read(&generated.text),
                Some(generated.expected),
                "reading\n{}", generated.text
            );
        }
    }

    #[test]
    fn only_a_cell_with_exactly_one_line_has_a_single_line() {
        assert_eq!(lines(&["a"]).single_line(), Some("a"));
        assert_eq!(lines(&[]).single_line(), None);
        assert_eq!(lines(&["a", "b"]).single_line(), None);
        assert_eq!(Cell::Unsupported.single_line(), None);
    }

    #[test]
    fn empty_lines_before_the_first_cell_are_skipped_and_text_there_is_refused() {
        let row = vec![lines(&["a"]), lines(&["b"])];
        for blank in ["", "  ", "\t"] {
            let text = format!("|===\n{blank}\n{blank}\n|a |b\n|===");
            assert_eq!(
                read(&text).map(|table| table.rows),
                Some(vec![row.clone()]),
                "{text:?}"
            );
        }
        for stray in ["stray", " x", "a b c"] {
            let text = format!("|===\n{stray}\n|a |b\n|===");
            assert_eq!(read(&text), None, "{text:?}");
        }
    }

    #[test]
    fn a_table_is_delimited_by_a_bar_and_three_or_more_equals_signs() {
        for delimiter in ["|===", "|====", "|=======", "|===  "] {
            let text = format!("{delimiter}\n|a\n{delimiter}");
            assert_eq!(
                read(&text).map(|table| table.rows),
                Some(vec![vec![lines(&["a"])]]),
                "{delimiter:?}"
            );
        }
        for other in [
            "|==", "|=", "|", "====", "=====", "|abc", "|==x", "x|===", "!===",
        ] {
            let text = format!("{other}\n|a\n{other}");
            assert_eq!(read(&text), None, "{other:?}");
        }
    }

    #[test]
    fn the_header_option_is_read_from_its_long_and_short_forms_only() {
        let rows = |attributes: &str| {
            let table = table(&[attributes, "|===", "|a |b", "|c |d", "|==="]);
            (table.header.is_some(), table.rows.len())
        };
        for with_header in [
            "[options=\"header\"]",
            "[opts=\"header\"]",
            "[opts=header]",
            "[%header]",
            "[%header,cols=\"1,1\"]",
            "[cols=\"1,1\",%header%autowidth]",
            "[%autowidth%header]",
        ] {
            assert_eq!(rows(with_header), (true, 1), "{with_header}");
        }
        for without_header in [
            "[header]",
            "[cols=\"1,1\"]",
            "[%autowidth]",
            "[%noheader]",
            "[options=\"noheader\"]",
            "[role=\"header\"]",
            "[headers]",
        ] {
            assert_eq!(rows(without_header), (false, 2), "{without_header}");
        }
    }

    #[test]
    fn asciidoc_cells_keep_lines_of_text_and_refuse_block_attributes() {
        let cell = |content: &[&str]| {
            let mut text = vec!["[cols=\"1,1\"]", "|===", "|ename:VK_A a|"];
            text.extend(content);
            text.push("|===");
            table(&text).rows[0][1].clone()
        };
        for kept in [
            "[eq]#x# and more",
            "[[anchor]]",
            "[[anchor]]text",
            "text [x]",
            "2 planes",
            "2x.",
            "a. b",
        ] {
            assert_eq!(cell(&[kept]), lines(&[kept]), "{kept}");
        }
        for block in [
            "[source,c]",
            "[NOTE]",
            "[x]",
            "1. first",
            "12. twelfth",
            ". first",
            "include::file.adoc[]",
            "image::picture.svg[]",
            "++++",
            "....",
        ] {
            assert_eq!(cell(&[block, "text"]), Cell::Unsupported, "{block}");
        }
        assert_eq!(
            cell(&["text", "++++", "x", "++++"]),
            Cell::Unsupported,
            "a passthrough block that no `[latexmath]` line introduces"
        );
        assert_eq!(
            cell(&["[latexmath]", "++++", "x +", " y", "++++", "text"]),
            lines(&["latexmath:[x + y]\ntext"])
        );
    }

    #[test]
    fn cells_with_other_styles_or_several_paragraphs_are_unsupported() {
        let table = table(&[
            "[cols=\"1,1\",options=\"header\"]",
            "|====",
            "|Mode |Result",
            "|ename:VK_A m|code",
            "|ename:VK_B |one",
            "",
            "two",
            "|====",
        ]);
        assert_eq!(table.rows[0][1], Cell::Unsupported);
        assert_eq!(table.rows[1][1], Cell::Unsupported);
    }
}
