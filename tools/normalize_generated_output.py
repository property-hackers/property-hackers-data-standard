"""Normalize generated text to one terminal newline."""

from __future__ import annotations

import sys


def normalize_output(source: str) -> str:
    if not source:
        return source
    return source.rstrip("\r\n") + "\n"


def main() -> None:
    sys.stdout.write(normalize_output(sys.stdin.read()))


if __name__ == "__main__":
    main()
