#!/usr/bin/env python3
"""Regenerate artifacts in isolation and compare them to the checked-out tree."""

from __future__ import annotations

import argparse
from collections.abc import Sequence
import hashlib
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import tempfile
import time


IGNORED_DIRECTORIES = frozenset({"target", "__pycache__"})
IGNORED_FILES = frozenset({".DS_Store", "Cargo.lock"})


class GeneratedDriftError(RuntimeError):
    """The checked-out generated tree differs from clean regeneration."""


def _copy_ignore(directory: str, names: list[str], *, schema_root: Path) -> set[str]:
    ignored = {
        name
        for name in names
        if name in IGNORED_DIRECTORIES
        or name in IGNORED_FILES
    }
    if Path(directory).resolve() == schema_root.resolve() and "generated" in names:
        ignored.add("generated")
    return ignored


def _copy_generation_inputs(project_root: Path, temporary_root: Path) -> None:
    """Copy only inputs required by the project's `just gen` recipes."""
    shutil.copy2(project_root / "justfile", temporary_root / "justfile")
    shutil.copytree(
        project_root / "schema",
        temporary_root / "schema",
        ignore=lambda directory, names: _copy_ignore(
            directory,
            names,
            schema_root=project_root / "schema",
        ),
    )
    shutil.copytree(
        project_root / "tools",
        temporary_root / "tools",
        ignore=lambda directory, names: _copy_ignore(
            directory,
            names,
            schema_root=project_root / "schema",
        ),
    )
    (temporary_root / "schema/generated").mkdir(parents=True)

    venv = project_root / ".venv"
    if venv.exists():
        (temporary_root / ".venv").symlink_to(venv, target_is_directory=True)


def _manifest(root: Path) -> dict[str, str]:
    """Return hashes for authoritative generated files only."""
    manifest: dict[str, str] = {}
    for path in sorted(root.rglob("*")):
        relative = path.relative_to(root)
        if any(part in IGNORED_DIRECTORIES for part in relative.parts):
            continue
        if path.name in IGNORED_FILES or path.suffix == ".pyc":
            continue
        if path.is_file():
            manifest[relative.as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
    return manifest


def _compare_generated(regenerated: Path, checked_out: Path) -> None:
    expected = _manifest(regenerated)
    actual = _manifest(checked_out)
    missing = sorted(expected.keys() - actual.keys())
    unexpected = sorted(actual.keys() - expected.keys())
    changed = sorted(
        path for path in expected.keys() & actual.keys() if expected[path] != actual[path]
    )
    if not (missing or unexpected or changed):
        return

    details = [*(f"missing: {path}" for path in missing)]
    details.extend(f"unexpected: {path}" for path in unexpected)
    details.extend(f"changed: {path}" for path in changed)
    raise GeneratedDriftError(
        "generated artifacts differ from clean regeneration:\n  "
        + "\n  ".join(details)
    )


def check_generated(
    project_root: Path,
    *,
    generator: Sequence[str] = ("just", "gen"),
    temp_parent: Path | None = None,
) -> None:
    """Generate from empty in a temporary project and compare read-only."""
    project_root = project_root.resolve()
    if temp_parent is not None:
        temp_parent.mkdir(parents=True, exist_ok=True)
    temporary_root = Path(
        tempfile.mkdtemp(prefix="phds-check-generated-", dir=temp_parent)
    )

    handled_signals = (signal.SIGTERM, signal.SIGHUP, signal.SIGINT)
    previous_handlers = {
        signum: signal.getsignal(signum) for signum in handled_signals
    }

    process: subprocess.Popen | None = None
    interrupted = False

    def signal_generator_group(signum: signal.Signals) -> None:
        if process is None:
            return
        try:
            os.killpg(process.pid, signum)
        except (ProcessLookupError, PermissionError):
            pass

    def generator_group_exists() -> bool:
        if process is None:
            return False
        try:
            os.killpg(process.pid, 0)
        except ProcessLookupError:
            return False
        except PermissionError:
            return True
        return True

    def terminate_generator_group() -> None:
        if process is None:
            return
        if interrupted:
            signal_generator_group(signal.SIGKILL)
            if process.poll() is None:
                process.wait()
            return
        signal_generator_group(signal.SIGTERM)
        deadline = time.monotonic() + 2
        while generator_group_exists() and time.monotonic() < deadline:
            process.poll()
            time.sleep(0.02)
        if generator_group_exists():
            signal_generator_group(signal.SIGKILL)
        if process.poll() is None:
            process.wait()

    def cleanup_signal(signum, _frame):
        nonlocal interrupted
        interrupted = True
        signal_generator_group(signal.SIGTERM)
        raise SystemExit(128 + signum)

    try:
        for signum in handled_signals:
            signal.signal(signum, cleanup_signal)
        _copy_generation_inputs(project_root, temporary_root)
        process = subprocess.Popen(
            tuple(generator),
            cwd=temporary_root,
            start_new_session=True,
        )
        returncode = process.wait()
        if returncode:
            raise subprocess.CalledProcessError(returncode, process.args)
        _compare_generated(
            temporary_root / "schema/generated",
            project_root / "schema/generated",
        )
    finally:
        terminate_generator_group()
        for signum, handler in previous_handlers.items():
            signal.signal(signum, handler)
        shutil.rmtree(temporary_root, ignore_errors=True)


def main(argv: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--project-root",
        type=Path,
        default=Path(__file__).resolve().parents[1],
    )
    args = parser.parse_args(argv)
    try:
        check_generated(args.project_root)
    except GeneratedDriftError as error:
        print(error, file=sys.stderr)
        return 1
    except subprocess.CalledProcessError as error:
        return error.returncode
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
