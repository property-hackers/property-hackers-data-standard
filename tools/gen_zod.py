#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys

from zod_generator import GenerationError, generate_zod


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", type=Path)
    parser.add_argument("--root-name", required=True)
    args = parser.parse_args(argv)
    try:
        schema = json.loads(args.input.read_text(encoding="utf-8"))
        sys.stdout.write(
            generate_zod(
                schema,
                root_name=args.root_name,
                source_name=args.input.as_posix(),
            )
        )
    except (OSError, json.JSONDecodeError, GenerationError) as error:
        print(error, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
