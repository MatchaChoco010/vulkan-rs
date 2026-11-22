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

fn column_count(cols: &str) -> Option<usize> {
    let mut count = 0;
    for column in cols.split([',', ';']) {
        let column = column.trim();
        count += match column.split_once('*') {
            Some((multiplier, _)) => multiplier.trim().parse::<usize>().ok()?,
            None if column.is_empty() => return None,
            None => 1,
        };
    }
    (count > 0).then_some(count)
}

fn cell_style(spec: &str) -> Option<Option<char>> {
    let mut rest = spec;
    if let Some(after) = rest.strip_prefix(['<', '^', '>']) {
        rest = after;
    }
    if let Some(after) = rest
        .strip_prefix(".<")
        .or_else(|| rest.strip_prefix(".^"))
        .or_else(|| rest.strip_prefix(".>"))
    {
        rest = after;
    }
    let mut chars = rest.chars();
    match (chars.next(), chars.next()) {
        (None, _) => Some(None),
        (Some(style), None) if "adehlmsv".contains(style) => Some(Some(style)),
        _ => None,
    }
}

fn is_span(spec: &str) -> bool {
    let factor = spec.trim_start_matches(|c: char| c == '.' || c.is_ascii_digit());
    let factor_len = spec.len() - factor.len();
    spec[..factor_len].chars().any(|c| c.is_ascii_digit()) && factor.starts_with(['+', '*'])
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
            let token_start = before
                .char_indices()
                .rev()
                .find(|(_, c)| c.is_whitespace())
                .map_or(0, |(space, c)| space + c.len_utf8());
            let token = &before[token_start..];
            let (text, style) = if token.is_empty() {
                (before, None)
            } else if is_span(token) {
                return None;
            } else {
                match cell_style(token) {
                    Some(style) => (&before[..token_start], style),
                    None => (before, None),
                }
            };
            match cells.last_mut() {
                Some(cell) => cell.content.push_str(text),
                None if !text.trim().is_empty() => return None,
                None => {}
            }
            cells.push(RawCell {
                style,
                line: line_index,
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
                .filter(|line| !line.is_empty())
                .copied()
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
            || line.starts_with("image::");
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
                        columns = Some(column_count(value.trim().trim_matches('"'))?);
                        continue;
                    }
                    "options" | "opts" => value.trim().trim_matches('"').to_string(),
                    option if option.starts_with('%') => option.to_string(),
                    _ => continue,
                };
                for option in options.split(['%', ',']).map(str::trim) {
                    match option {
                        "header" => header_option = Some(true),
                        "noheader" => header_option = Some(false),
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
    let implicit_header = cells.len() > columns
        && cells[..columns].iter().all(|cell| cell.line == first_line)
        && cells[columns].line > first_line
        && content
            .get(first_line + 1)
            .is_some_and(|line| line.trim().is_empty());
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
            "                    p_0(A_s,A_d) & = A_sA_d \\\\",
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

    #[test]
    fn column_specifications_count_multiplied_columns() {
        assert_eq!(column_count("59%,28%,13%"), Some(3));
        assert_eq!(column_count("<35,<23"), Some(2));
        assert_eq!(column_count("3*,2"), Some(4));
        assert_eq!(column_count(""), None);
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
