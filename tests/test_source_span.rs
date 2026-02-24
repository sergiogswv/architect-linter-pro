//! Tests for the source_span module.

use architect_linter_pro::source_span::{
    line_to_byte_offset, lines_between_offsets, span_for_import, span_for_line,
};

// --- line_to_byte_offset ---

#[test]
fn test_offset_line_1_is_zero() {
    assert_eq!(line_to_byte_offset("hello\nworld\n", 1), 0);
}

#[test]
fn test_offset_line_2_after_first_newline() {
    // "hello\n" = 6 bytes
    assert_eq!(line_to_byte_offset("hello\nworld\n", 2), 6);
}

#[test]
fn test_offset_line_3() {
    // "hello\nworld\n" = 12 bytes, line 3 starts at 12
    assert_eq!(line_to_byte_offset("hello\nworld\n", 3), 12);
}

#[test]
fn test_offset_single_line_file() {
    // No newlines: line 1 = 0, line 2 = past end
    let s = "no newlines here";
    assert_eq!(line_to_byte_offset(s, 1), 0);
    assert_eq!(line_to_byte_offset(s, 2), s.len());
}

#[test]
fn test_offset_typescript_import() {
    let src = "import { foo } from 'bar';\nimport { baz } from 'qux';\n";
    // Line 2 starts after "import { foo } from 'bar';\n" = 27 bytes
    assert_eq!(line_to_byte_offset(src, 2), 27);
}

// --- span_for_line ---

#[test]
fn test_span_for_line_1_covers_first_line() {
    let src = "import foo from 'bar';\nimport baz from 'qux';\n";
    let span = span_for_line(src, 1);
    assert_eq!(span.offset(), 0);
    // "import foo from 'bar';" = 22 chars
    assert_eq!(span.len(), 22);
}

#[test]
fn test_span_for_line_2_covers_second_line() {
    let src = "import foo from 'bar';\nimport baz from 'qux';\n";
    let span = span_for_line(src, 2);
    assert_eq!(span.offset(), 23); // after "import foo from 'bar';\n"
    assert_eq!(span.len(), 22);    // "import baz from 'qux';"
}

// --- span_for_import ---

#[test]
fn test_span_for_import_finds_needle() {
    let src = "import { foo } from './foo';\nimport { bar } from './bar';\n";
    let span = span_for_import(src, 1, "./foo");
    // "./foo" is in "import { foo } from './foo';" — offset within file
    assert_eq!(&src[span.offset()..span.offset() + span.len()], "./foo");
}

#[test]
fn test_span_for_import_falls_back_to_full_line_when_not_found() {
    let src = "import { foo } from './foo';\n";
    let span = span_for_import(src, 1, "NOT_THERE");
    // Should fall back to full line length
    assert_eq!(span.offset(), 0);
    assert_eq!(span.len(), 27); // "import { foo } from './foo';"
}

// --- lines_between_offsets ---

#[test]
fn test_lines_between_same_line() {
    let src = "fn foo() {\n    return 1;\n}\n";
    // Both offsets on line 1
    assert_eq!(lines_between_offsets(src, 0, 10), 0);
}

#[test]
fn test_lines_between_two_lines() {
    let src = "fn foo() {\n    return 1;\n}\n";
    // 0..24 spans "fn foo() {\n    return 1;" = 1 newline
    assert_eq!(lines_between_offsets(src, 0, 24), 1);
}

#[test]
fn test_lines_between_whole_file() {
    let src = "fn foo() {\n    return 1;\n}\n";
    // 3 newlines in the full file
    assert_eq!(lines_between_offsets(src, 0, src.len()), 3);
}

#[test]
fn test_lines_between_reversed_offsets() {
    // end < start → 0
    assert_eq!(lines_between_offsets("a\nb\nc\n", 5, 2), 0);
}
