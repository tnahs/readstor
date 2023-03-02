//! Defines a parser to convert an [epubcfi][epubcfi] into a sortable string
//! for sorting annotations into their order of appearance inside their
//! respective books.
//!
//! [epubcfi]: https://w3c.github.io/epub-specs/epub33/epubcfi/

use std::borrow::ToOwned;

use once_cell::sync::Lazy;
use regex::Regex;

/// Capture a 'Step Reference' e.g. `/6` `/4`
///
/// <https://w3c.github.io/epub-specs/epub33/epubcfi/#sec-path-child-ref>
static RE_STEP_REFERENCE: Lazy<Regex> = Lazy::new(|| Regex::new(r"/[0-9]+").unwrap());

/// Captures an 'XML ID Assertion / Text Location Assertion' e.g. `[chap01]`
///
/// The specific difference between these two doesn't matter for our purposes.
/// We just need to strip out anything that resembles an 'Assertion'.
///
/// <https://w3c.github.io/epub-specs/epub33/epubcfi/#sec-path-xmlid>
/// <https://w3c.github.io/epub-specs/epub33/epubcfi/#sec-path-text-location>
static RE_ASSERTIONS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)
        # Captures opening square bracket e.g. `[`
        \[

        # Captures anything but square brackets e.g. `chap01`
        [^\[\]]*

        # Captures closing square bracket e.g. `]`
        \]
    ",
    )
    .unwrap()
});

/// Captures a 'Character Offset' e.g. `:2` `:100`
///
/// <https://w3c.github.io/epub-specs/epub33/epubcfi/#sec-path-terminating-char>
static RE_CHARACTER_OFFSET: Lazy<Regex> = Lazy::new(|| Regex::new(r":[0-9]+$").unwrap());

/// Captures a 'Spacial Offset' e.g. `~23.5` `~42.43`
///
/// <https://w3c.github.io/epub-specs/epub33/epubcfi/#sec-path-terminating-spatial>
static RE_TEMPORAL_OFFSET: Lazy<Regex> = Lazy::new(|| Regex::new(r"~[0-9]+\.[0-9]+").unwrap());

/// Captures a 'Temporal Offset' e.g. `@100:100` `@5.75:97.6`
///
/// <https://w3c.github.io/epub-specs/epub33/epubcfi/#sec-path-terminating-temporal>
static RE_SPACIAL_OFFSET: Lazy<Regex> = Lazy::new(|| Regex::new(r"@[0-9.]+:[0-9.]+").unwrap());

/// Returns a simplified location string from a `epubcfi`.
///
/// This is a super simple EPUB CFI parser with a focus on extracting location
/// information for sorting [`Annotation`][annotation]s.
///
/// Examples:
///
/// ```plaintext
/// input:  epubcfi(/6/4[chap01ref]!/4[body01]/10[para05]/1:3[xx,y])
/// output: 6.4.4.10.1:3
/// ```
/// <https://w3c.github.io/epub-specs/epub33/epubcfi/#example-8>
///
/// ```plaintext
/// input:  epubcfi(/6/4[chap01ref]!/4[body01]/10[para05],/2/1:1,/3:4)
/// output: 6.4.4.10.2.1:1
/// ```
/// <https://w3c.github.io/epub-specs/epub33/epubcfi/#example-23>
///
/// See <https://w3c.github.io/epub-specs/epub33/epubcfi/> for more
/// information.
///
/// [annotation]: super::annotation::Annotation
#[must_use]
pub fn parse(raw: &str) -> String {
    // Check that the incoming string is an `epubcfi`.
    if !raw.starts_with("epubcfi(") && !raw.ends_with(')') {
        return String::new();
    }

    // Starting with:
    //
    //    A: epubcfi(/6/4[chap01ref]!/4[body01]/10[para05],/2/1:1,/3:4)
    //    B: epubcfi(/6/4[chap01ref]!/4[body01]/10[para05]/1:3[xx,y])
    //    C: epubcfi(/2/4!/6[bar]/44!/3~1.11@1:1)

    // Strip start and end: i.e. `epubcfi(` & `)`
    //
    // -> A: /6/4[chap01ref]!/4[body01]/10[para05],/2/1:1,/3:4
    // -> B: /6/4[chap01ref]!/4[body01]/10[para05]/1:3[xx,y]
    // -> C: /2/4!/6[bar]/44!/3~1.11@1:1
    let mut location = raw[8..raw.len() - 1].to_owned();

    // Dropping the following elements means they are not taken into
    // consideration during sorting comparisons between `Annotation`s.

    // Remove any type of 'Assertion'.
    //
    // -> A: /6/4!/4/10,/2/1:1,/3:4
    // -> B: /6/4!/4/10/1:3
    // -> C: /2/4!/6/44!/3~1.11@1:1
    location = RE_ASSERTIONS.replace_all(&location, "").into_owned();

    // Remove 'Temporal Offsets' (~)..
    //
    // -> A: ...
    // -> B: ...
    // -> C: /2/4!/6/44!/3@1:1
    location = RE_TEMPORAL_OFFSET.replace_all(&location, "").into_owned();

    // Remove 'Spacial Offsets' (@).
    //
    // -> A: ...
    // -> B: ...
    // -> C: /2/4!/6/44!/3
    location = RE_SPACIAL_OFFSET.replace_all(&location, "").into_owned();

    // "EPUB CFIs allow the expression of simple ranges extending from a start
    // location to an end location."
    //
    // <https://w3c.github.io/epub-specs/epub33/epubcfi/#sec-ranges>
    //
    // For example:
    //
    //     epubcfi([parent-path],[range-start],[range-end])
    //
    // We only care about the [parent-path] and [range-start] which gives us
    // the absolute path to where an `Annotation` begins.
    let mut parts: Vec<&str> = location.split(',').collect();
    parts = match parts[..] {
        [parent_path, range_start, _] => {
            vec![parent_path, range_start]
        }
        _ => parts,
    };

    // -> A: /6/4!/4/10,/2/1:1
    // -> B: /6/4!/4/10/1:3
    // -> C: /2/4!/6/44!/3
    location = parts.join("");

    // -> A: /6/4/4/10/2/1
    // -> B: /6/4/4/10/1
    // -> C: /2/4/6/44/3
    let mut steps = RE_STEP_REFERENCE
        .find_iter(&location)
        .map(|m| m.as_str())
        .map(ToOwned::to_owned)
        .collect::<String>();

    // -> A: 6/4/4/10/2/1
    // -> B: 6/4/4/10/1
    // -> C: 2/4/6/44/3
    steps.remove(0);

    // -> A: 6.4.4.10.2.1
    // -> B: 6.4.4.10.1
    // -> C: 2.4.6.44.3
    steps = steps.replace('/', ".");

    // Save the character offset found at the end of [range-start].
    //
    // -> A: :1
    // -> B: :3
    // -> C: N/A
    let character_offset = RE_CHARACTER_OFFSET
        .find(&location)
        .map(|m| m.as_str())
        .map_or_else(String::new, ToOwned::to_owned);

    // -> A: 6.4.4.10.2.1:1
    // -> B: 6.4.4.10.1:3
    // -> C: 2.4.6.44.3
    location = format!("{steps}{character_offset}");

    location
}

#[cfg(test)]
mod test_epubcfi {

    use super::*;

    // https://stackoverflow.com/a/34666891/16968574
    macro_rules! test_parse_epubcfi {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (raw, expected) = $value;
                    let parsed = parse(raw);
                    assert_eq!(parsed, expected);
                }
            )*
        }
    }

    // https://stackoverflow.com/a/34666891/16968574
    macro_rules! test_compare_epubcfi {
        ($($name:ident: ($lhs:tt $cmp:tt $rhs:tt),)*) => {
            $(
                #[test]
                fn $name() {
                    let lhs_parsed = parse($lhs);
                    let rhs_parsed = parse($rhs);
                    assert!(lhs_parsed $cmp rhs_parsed);
                }
            )*
        }
    }

    // <https://github.com/fread-ink/epub-cfi-resolver/blob/master/tests/simple.js>
    test_parse_epubcfi! {
        parse_epubcfi_00: (
            "epubcfi(/1/2)",
            "1.2",
        ),
        parse_epubcfi_01: (
            "epubcfi(/1/0)",
            "1.0",
        ),
        parse_epubcfi_02: (
            "epubcfi(/1/2:3[pre,post])",
            "1.2:3",
        ),
        parse_epubcfi_03: (
            "epubcfi(/1/2:3[,post])",
            "1.2:3",
        ),
        parse_epubcfi_04: (
            "epubcfi(/1/2:3[pre,])",
            "1.2:3",
        ),
        parse_epubcfi_05: (
            "epubcfi(/1[^^^]])",
            "1",
        ),
        parse_epubcfi_06: (
            "epubcfi(/6/14[cha!/p05ref]!/4[bo!/dy01]/10/2/1[foo]:5[don't!/ panic;s=b])",
            "6.14.4.10.2.1:5",
        ),
        parse_epubcfi_07: (
            "epubcfi(/6/4[chap01ref]!/4[body01]/10[para05]/3:5)",
            "6.4.4.10.3:5",
        ),
        parse_epubcfi_08: (
            "epubcfi(/6/4[chap01ref]!/4/10/0)",
            "6.4.4.10.0",
        ),
        parse_epubcfi_09: (
            "epubcfi(/6/4[chap01ref]!/4/10/999)",
            "6.4.4.10.999",
        ),
        parse_epubcfi_10: (
            "epubcfi(/6/4[chap01ref]!/4[body01],/10[para05]/3:5,/10[para05]/3:8)",
            "6.4.4.10.3:5",
        ),
        parse_epubcfi_11: (
            "epubcfi(/6/4[chap01ref]!/4[body01]/10[para05]/3:3[34,67])",
            "6.4.4.10.3:3",
        ),
        parse_epubcfi_12: (
            "epubcfi(/6/14[cha!/p05ref]!/4[bo!/dy01]/10/2/1[foo]~42.43@100:101)",
            "6.14.4.10.2.1",
        ),
        // Test that 'Temporal' and 'Spatial' offsets are ignored on all but last subpart.
        parse_epubcfi_13: (
            "epubcfi(/2~42.43@100:101/4!/6/8:100/6:200)",
            "2.4.6.8.6:200",
        ),
        // Test that parser ignores vendor extensions.
        // <https://w3c.github.io/epub-specs/epub33/epubcfi/#sec-extensions>
        parse_epubcfi_14: (
            "epubcfi(/2/4vnd.foo/6foo.bar:20)",
            "2.4.6:20",
        ),
        parse_epubcfi_15: (
            "epubcfi(/6/4[chap01ref]!/4[body01]/10[para05],/2/1:1,/3:4)",
            "6.4.4.10.2.1:1",
        ),
        parse_epubcfi_16: (
            "epubcfi(/6/4[chap01ref]!/4[body01]/10[para05]/1:3[xx,y])",
            "6.4.4.10.1:3",
        ),
        parse_epubcfi_17: (
            "epubcfi(/6/28[chap06]!/4/24[para06]/1,:4,:44)",
            // TODO: Could this --------------------^^ cause an error? Should it be padded with a
            // `0` so it doesn't look like its attached to the wrong step? -> '6.28.4.24.1.0:4'
            "6.28.4.24.1:4",
        ),
        parse_epubcfi_18: (
            "epubcfi(/2/4[node-id]!/6/7:5[pre,post;s=b])",
            "2.4.6.7:5",
        ),
        parse_epubcfi_19: (
            "epubcfi(/2/4@4:2)",
            "2.4",
        ),
        parse_epubcfi_20: (
            "epubcfi(/2/4~3.14)",
            "2.4",
        ),
        parse_epubcfi_21: (
            "epubcfi(/2/4~3.14@4:2)",
            "2.4",
        ),
    }

    // <https://github.com/fread-ink/epub-cfi-resolver/blob/master/tests/compare.js>
    test_compare_epubcfi! {
        compare_epubcfi_00: (
            "epubcfi(/2)" < "epubcfi(/6)"
        ),
        compare_epubcfi_01: (
            "epubcfi(/2/4!/6)" < "epubcfi(/2/4!/7)"
        ),
        compare_epubcfi_02: (
            "epubcfi(/2/4!/8)" > "epubcfi(/2/4!/7)"
        ),
        compare_epubcfi_03: (
            "epubcfi(/2/4!/6[foo]/42!/12:100[lol])" < "epubcfi(/2/4!/6[bar]/44!/12:100[cat])"
        ),
        // Test that node ids and text location assertions are ignored.
        compare_epubcfi_04: (
            "epubcfi(/2/4!/6[foo]/44!/12:100[lol])" == "epubcfi(/2/4!/6[bar]/44!/12:100[cat])"
        ),
        compare_epubcfi_05: (
            "epubcfi(/2/4!/6[bar]/44!/12:100[cat])" == "epubcfi(/2/4!/6[bar]/44!/12:100[cat])"
        ),
        // Test that temporal and spatial offsets are ignored on character (text/cdata) nodes
        compare_epubcfi_06: (
            "epubcfi(/2/4!/6[bar]/44!/3~1.11@1:1)" == "epubcfi(/2/4!/6[bar]/44!/3~2.22@2:2)"
        ),
        // Compare identical ranges.
        compare_epubcfi_07: (
            "epubcfi(/2/4,/6/8,/10/12)" == "epubcfi(/2/4,/6/8,/10/12)"
        ),
        // Compare ranges with different [range-start].
        compare_epubcfi_08: (
            "epubcfi(/2/4,/6/7,/10/11)" < "epubcfi(/2/4,/6/8,/10/12)"
        ),
        // Compare ranges with different [parent-path].
        compare_epubcfi_09: (
            "epubcfi(/2/2,/6/8,/10/12)" < "epubcfi(/2/4,/6/8,/10/12)"
        ),
        // Compare a range against a non-range.
        compare_epubcfi_10: (
            "epubcfi(/2/4,/6/8,/10/13)" > "epubcfi(/2/4/6/7)"
        ),
        // Compare a range against a non-range
        compare_epubcfi_11: (
            "epubcfi(/2/4,/6/8,/10/13)" == "epubcfi(/2/4/6/8)"
        ),
        compare_epubcfi_12: (
            "epubcfi(/2/4!/6[bar]/44!/12:100[hah])" < "epubcfi(/2/4!/6[bar]/44!/12:200[cat])"
        ),
    }
}
