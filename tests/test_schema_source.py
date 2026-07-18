"""Lint the LinkML source for regex constructs that diverge across consumers.

Schema `pattern` values are compiled by Python's re (linkml-validate,
gen-pydantic output), Python jsonschema, and ECMA engines (AJV et al.).
Only the dialect-portable subset is allowed; these rules encode divergences
already fixed once (trailing-newline `$`, Unicode `\\d`) so they cannot be
reintroduced. Patterns are discovered by walking the parsed YAML (any
quoting style), and checks are token-aware: escaped literals like `\\$`
or `\\\\d` are not false positives, and the portable `[\\s\\S]` sentinel
is explicitly allowed inside character classes.
"""

from __future__ import annotations

from pathlib import Path
import unittest

import yaml

ROOT = Path(__file__).resolve().parents[1]

# escape-pair -> why it is forbidden (outside character classes for b/B)
FORBIDDEN_ESCAPES = {
    "d": r"Python \d matches Unicode digits, ECMA \d is ASCII-only — use [0-9]",
    "D": r"complement of \d diverges the same way — use [^0-9]",
    "w": r"Python \w is Unicode-aware, ECMA \w is [A-Za-z0-9_] — spell the class out",
    "W": r"complement of \w diverges the same way",
    "b": r"word boundary depends on \w and diverges with it",
    "B": r"non-boundary depends on \w and diverges with it",
    "A": r"Python-only anchor; not valid ECMA",
    "Z": r"Python-only anchor; not valid ECMA",
    "z": r"not supported by Python re or ECMA",
    "p": r"Unicode property escapes are ECMA-only (with /u); Python re lacks \p",
    "P": r"Unicode property escapes are ECMA-only (with /u); Python re lacks \P",
}

# after "(?", only these introducers are portable across both dialects
PORTABLE_GROUP_INTRODUCERS = (":", "=", "!", "<=", "<!")


def pattern_issues(pattern: str) -> list[str]:
    """Return dialect-portability problems in a regex pattern string."""
    issues = []
    in_class = False
    i = 0
    while i < len(pattern):
        c = pattern[i]
        if c == "\\":
            nxt = pattern[i + 1] if i + 1 < len(pattern) else ""
            if nxt in FORBIDDEN_ESCAPES:
                # \s / \S never appear here (not in FORBIDDEN_ESCAPES); \b
                # inside a class is a literal backspace in both dialects.
                if not (nxt in "bB" and in_class):
                    issues.append(f"\\{nxt}: {FORBIDDEN_ESCAPES[nxt]}")
            i += 2
            continue
        if c == "[" and not in_class:
            in_class = True
        elif c == "]" and in_class:
            in_class = False
        elif not in_class and pattern.startswith("(?", i):
            rest = pattern[i + 2 :]
            if not rest.startswith(PORTABLE_GROUP_INTRODUCERS):
                if rest.startswith("P"):
                    issues.append("(?P...): Python-only named-group syntax")
                elif rest.startswith("<"):
                    issues.append("(?<name>): ECMA named groups are not Python re syntax")
                else:
                    issues.append("(?flags): inline flags are not portable to ECMA")
            i += 2
            continue
        i += 1
    if pattern.endswith("$") and not pattern.endswith("\\$"):
        issues.append(
            "trailing $: Python $ also matches before a trailing newline, "
            "ECMA $ does not — end with (?![\\s\\S]) instead"
        )
    return issues


def _walk_patterns(node, source):
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "pattern" and isinstance(value, str):
                yield source, value
            else:
                yield from _walk_patterns(value, source)
    elif isinstance(node, list):
        for item in node:
            yield from _walk_patterns(item, source)


def schema_patterns():
    for path in sorted((ROOT / "schema").glob("*.yaml")):
        doc = yaml.safe_load(path.read_text(encoding="utf-8"))
        yield from _walk_patterns(doc, path.name)


class PatternCheckerTests(unittest.TestCase):
    """The checker itself must catch known divergences and allow the portable subset."""

    def test_flags_divergent_constructs(self):
        for bad in (
            r"^\d+$",
            r"^[0-9]+\Z",
            r"\A[0-9]+",
            r"^\w+(?![\s\S])",
            r"^ab\b",
            r"\B",
            r"\p{L}+",
            r"(?i)abc",
            r"(?P<year>[0-9]{4})",
            r"(?<year>[0-9]{4})",
            r"^[0-9]+$",
        ):
            with self.subTest(pattern=bad):
                self.assertTrue(pattern_issues(bad))

    def test_allows_portable_subset(self):
        for good in (
            r"^-?[0-9]+(\.[0-9]+)?(?![\s\S])",
            r"^[A-Z]{3}(?![\s\S])",
            r"^(?:a|b)+(?![\s\S])",
            r"^(?=x)x[0-9]*(?![\s\S])",
            r"^(?<=a)b(?![\s\S])",
            r"price\$",          # escaped literal $ is fine
            r"^a\\d(?![\s\S])",  # literal backslash then d, not \d
            r"^[\b]?x(?![\s\S])",  # backspace literal inside a class
        ):
            with self.subTest(pattern=good):
                self.assertEqual([], pattern_issues(good))


class SchemaPatternDialectTests(unittest.TestCase):
    def test_schema_declares_patterns(self):
        # DecimalString type, shared amount + currency slots, country x2
        self.assertGreaterEqual(len(list(schema_patterns())), 5)

    def test_schema_patterns_are_dialect_portable(self):
        for source, pattern in schema_patterns():
            with self.subTest(source=source, pattern=pattern):
                self.assertEqual([], pattern_issues(pattern))


if __name__ == "__main__":
    unittest.main()
