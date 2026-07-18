from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys
from typing import Any


BOMA_MARKET_SYSTEMS = {
    "urn:phds:standard:boma:office:metro",
    "urn:phds:standard:boma:office:international",
}


def boma_placement_issues(value: Any, path: str = "") -> list[str]:
    """Reject BOMA market classifications in physical rating fields."""
    issues: list[str] = []
    if isinstance(value, dict):
        for key, child in value.items():
            child_path = f"{path}.{key}" if path else key
            if key in {"condition_ratings", "quality_ratings"} and isinstance(
                child, list
            ):
                for index, rating in enumerate(child):
                    if not isinstance(rating, dict):
                        continue
                    if rating.get("system") not in BOMA_MARKET_SYSTEMS:
                        continue
                    issues.append(
                        f"{child_path}[{index}]: BOMA market classification is not a "
                        "physical condition or construction quality rating"
                    )
            if key != "extras":
                issues.extend(boma_placement_issues(child, child_path))
    elif isinstance(value, list):
        for index, child in enumerate(value):
            issues.extend(boma_placement_issues(child, f"{path}[{index}]"))
    return issues


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Validate semantic rules for optional PHDS standards profiles."
    )
    parser.add_argument("document", type=Path)
    args = parser.parse_args()

    try:
        document = json.loads(args.document.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        print(f"{args.document}: {error}", file=sys.stderr)
        return 2

    issues = boma_placement_issues(document)
    for issue in issues:
        print(issue)
    return 1 if issues else 0


if __name__ == "__main__":
    raise SystemExit(main())
