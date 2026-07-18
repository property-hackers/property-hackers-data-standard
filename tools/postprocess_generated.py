"""Apply deterministic compatibility fixes to LinkML-generated clients."""

from __future__ import annotations

from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
GENERATED = ROOT / "schema" / "generated"


def _replace_once(text: str, old: str, new: str, *, label: str) -> str:
    # Exactly one unpatched marker -> patch it; exactly one already-patched
    # marker -> idempotent no-op. Anything else (missing, duplicated) means
    # generator drift: fail closed. Note `new` may be a prefix of `old`, so
    # when old is present the new-count includes that overlap.
    old_count = text.count(old)
    new_count = text.count(new)
    if old_count == 1:
        return text.replace(old, new, 1)
    if old_count == 0 and new_count == 1:
        return text
    raise RuntimeError(
        f"unexpected {label} marker state (unpatched={old_count}, patched={new_count})"
    )


def _patch_typescript() -> None:
    path = GENERATED / "phds.ts"
    text = path.read_text(encoding="utf-8")
    text = _replace_once(
        text,
        "export enum AssessorStatus {\n    \n",
        "export enum AssessorStatus {\n",
        label="TypeScript AssessorStatus enum",
    )
    text = _replace_once(
        text,
        "export interface AssessorObservation {\n    status: string,",
        "export interface AssessorObservation {\n    status: AssessorStatus,",
        label="TypeScript AssessorObservation status",
    )
    path.write_text(text, encoding="utf-8")


def main() -> None:
    _patch_typescript()


if __name__ == "__main__":
    main()
