//! Lightweight span helpers for miette diagnostics.
//!
//! Replaces `swc_common::SourceMap` for byte-offset computation.
//! All functions operate directly on `&str` content without external dependencies.

use miette::SourceSpan;

/// Convert a 1-based line number to its byte offset in `content`.
///
/// Returns `0` for line 1. Returns `content.len()` if the line number
/// exceeds the number of lines in the file.
pub fn line_to_byte_offset(content: &str, line: usize) -> usize {
    if line <= 1 {
        return 0;
    }
    let target_newline = line - 2; // 0-indexed: we want the (line-2)'th newline
    let mut newline_count = 0;
    for (i, ch) in content.char_indices() {
        if ch == '\n' {
            if newline_count == target_newline {
                return i + 1;
            }
            newline_count += 1;
        }
    }
    content.len()
}

/// Create a `SourceSpan` that covers the entire given 1-based line.
///
/// The span runs from the start of the line to the character before the
/// newline (or the end of content if there is no trailing newline).
pub fn span_for_line(content: &str, line: usize) -> SourceSpan {
    let start = line_to_byte_offset(content, line);
    let line_content = content[start..].split('\n').next().unwrap_or("");
    (start, line_content.len()).into()
}

/// Create a `SourceSpan` that highlights a specific substring within the
/// given 1-based line.
///
/// If `needle` is not found on the line, the full line is highlighted
/// excluding any trailing statement terminator (`;`).
pub fn span_for_import(content: &str, line: usize, needle: &str) -> SourceSpan {
    let line_start = line_to_byte_offset(content, line);
    let line_content = content[line_start..].split('\n').next().unwrap_or("");
    if let Some(offset) = line_content.find(needle) {
        (line_start + offset, needle.len()).into()
    } else {
        let len = line_content.trim_end_matches(';').len();
        (line_start, len).into()
    }
}

/// Count the number of lines between two byte offsets in `content`.
///
/// Used for measuring function/method body length when only byte offsets
/// are available (replaces `cm.lookup_char_pos(span.lo/hi).line`).
pub fn lines_between_offsets(content: &str, start: usize, end: usize) -> usize {
    let start = start.min(content.len());
    let end = end.min(content.len());
    if end <= start {
        return 0;
    }
    content[start..end].chars().filter(|&c| c == '\n').count()
}

