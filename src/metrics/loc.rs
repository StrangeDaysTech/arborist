use crate::languages::LanguageProfile;
use tree_sitter::Node;
use std::collections::HashSet;

/// Count source lines of code within a range, excluding blank and comment-only lines.
pub fn compute_sloc(
    node: &Node,
    source: &[u8],
    profile: &dyn LanguageProfile,
) -> u64 {
    let start_line = node.start_position().row;
    let end_line = node.end_position().row;
    compute_sloc_for_range(node, source, profile, start_line, end_line)
}

/// Count SLOC for the entire file.
pub fn compute_file_sloc(
    root: &Node,
    source: &[u8],
    profile: &dyn LanguageProfile,
) -> u64 {
    let total_lines = source.split(|&b| b == b'\n').count();
    if total_lines == 0 {
        return 0;
    }
    compute_sloc_for_range(root, source, profile, 0, total_lines.saturating_sub(1))
}

fn compute_sloc_for_range(
    root: &Node,
    source: &[u8],
    profile: &dyn LanguageProfile,
    start_line: usize,
    end_line: usize,
) -> u64 {
    let lines: Vec<&[u8]> = source.split(|&b| b == b'\n').collect();

    // Collect all lines that are entirely covered by comment nodes
    let mut comment_lines: HashSet<usize> = HashSet::new();
    collect_comment_lines(root, source, profile, &mut comment_lines);

    let mut sloc = 0u64;
    for (line_idx, line) in lines
        .iter()
        .enumerate()
        .take(end_line.min(lines.len().saturating_sub(1)) + 1)
        .skip(start_line)
    {
        // Skip blank lines
        if line.iter().all(|&b| b.is_ascii_whitespace()) {
            continue;
        }
        // Skip comment-only lines
        if comment_lines.contains(&line_idx) {
            continue;
        }
        sloc += 1;
    }

    sloc
}

fn collect_comment_lines(
    node: &Node,
    source: &[u8],
    profile: &dyn LanguageProfile,
    comment_lines: &mut HashSet<usize>,
) {
    if profile.comment_nodes().contains(&node.kind()) {
        let lines: Vec<&[u8]> = source.split(|&b| b == b'\n').collect();
        for line_idx in node.start_position().row..=node.end_position().row {
            if let Some(line) = lines.get(line_idx) {
                // Only mark as comment-only if the non-comment portion is whitespace.
                // For a line comment, check if everything before the comment start is whitespace.
                let comment_start_col = if line_idx == node.start_position().row {
                    node.start_position().column
                } else {
                    0
                };
                let before_comment = &line[..comment_start_col.min(line.len())];
                let comment_end_col = if line_idx == node.end_position().row {
                    node.end_position().column
                } else {
                    line.len()
                };
                let after_comment = &line[comment_end_col.min(line.len())..];

                if before_comment.iter().all(|&b| b.is_ascii_whitespace())
                    && after_comment.iter().all(|&b| b.is_ascii_whitespace())
                {
                    comment_lines.insert(line_idx);
                }
            }
        }
        return;
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_comment_lines(&child, source, profile, comment_lines);
    }
}
