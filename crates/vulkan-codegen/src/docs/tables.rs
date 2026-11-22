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
    let cols = cols.replace(' ', "");
    if !cols.is_empty() && cols.bytes().all(|b| b.is_ascii_digit()) {
        let count = cols.parse::<usize>().ok()?;
        return (count > 0 && count.to_string() == cols).then_some(count);
    }
    let mut count = 0;
    for column in cols.split([',', ';']) {
        let column = column.trim();
        if column.chars().any(|c| c.is_ascii_alphabetic()) {
            return None;
        }
        count += match column.split_once('*') {
            Some((multiplier, _)) => multiplier.trim().parse::<usize>().ok()?,
            None if column.is_empty() => return None,
            None => 1,
        };
    }
    (count > 0).then_some(count)
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
    let digits = line.chars().take_while(char::is_ascii_digit).count();
    digits > 0 && line[digits..].starts_with('.')
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
                        columns = Some(column_count(value.trim().trim_matches('"'))?);
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

    #[test]
    fn column_specifications_count_multiplied_columns() {
        assert_eq!(column_count("59%,28%,13%"), Some(3));
        assert_eq!(column_count("<35,<23"), Some(2));
        assert_eq!(column_count("3*,2"), Some(4));
        assert_eq!(column_count(""), None);
        assert_eq!(
            column_count("1,2a"),
            None,
            "a column style changes how its cells are read"
        );
        assert_eq!(column_count("3*a"), None);
        assert_eq!(column_count("<1,^.^2l"), None);
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
        assert_eq!(column_count("0"), None);
        assert_eq!(column_count("03"), None);
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
        let implicit_header = header && rows.len() > 1 && choose(2) == 0;
        let first_row_on_one_line = matches!(cols, Cols::Absent) || implicit_header;
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
            }
            if one_line {
                text.push(line);
            }
            if index == 0 && implicit_header {
                text.push(String::new());
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
