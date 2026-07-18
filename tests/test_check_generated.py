"""Behavioral tests for the isolated generated-artifact drift checker."""

from __future__ import annotations

import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time
import unittest

from tools.check_generated import GeneratedDriftError, check_generated


class GeneratedDriftCheckerTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.base = Path(self.temporary.name)
        self.project = self.base / "project"
        self.temp_parent = self.base / "checker-temporaries"
        (self.project / "schema/generated").mkdir(parents=True)
        (self.project / "schema/core.yaml").write_text("version: 0.2.0\n")
        (self.project / "tools").mkdir()
        (self.project / "justfile").write_text("gen:\n    true\n")

    def tearDown(self):
        self.temporary.cleanup()

    def _generator(self, body: str, name: str = "generator.py") -> tuple[str, ...]:
        path = self.base / name
        path.write_text(
            "from pathlib import Path\n"
            "generated = Path.cwd() / 'schema/generated'\n"
            "generated.mkdir(parents=True, exist_ok=True)\n"
            + body
        )
        return (sys.executable, str(path))

    def _actual(self, relative: str, content: str) -> Path:
        path = self.project / "schema/generated" / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content)
        return path

    def test_altered_generated_content_fails_without_mutating_original(self):
        actual = self._actual("contract.txt", "checked-out\n")
        generator = self._generator(
            "(generated / 'contract.txt').write_text('regenerated\\n')\n"
        )

        with self.assertRaisesRegex(GeneratedDriftError, "changed: contract.txt"):
            check_generated(
                self.project, generator=generator, temp_parent=self.temp_parent
            )

        self.assertEqual("checked-out\n", actual.read_text())

    def test_extra_stale_generated_artifact_fails(self):
        self._actual("contract.txt", "canonical\n")
        self._actual("stale.txt", "obsolete\n")
        generator = self._generator(
            "(generated / 'contract.txt').write_text('canonical\\n')\n"
        )

        with self.assertRaisesRegex(GeneratedDriftError, "unexpected: stale.txt"):
            check_generated(
                self.project, generator=generator, temp_parent=self.temp_parent
            )

    def test_generator_failure_is_propagated_and_original_tree_is_preserved(self):
        actual = self._actual("contract.txt", "untouched\n")
        generator = self._generator("raise SystemExit(23)\n")

        with self.assertRaises(subprocess.CalledProcessError) as raised:
            check_generated(
                self.project, generator=generator, temp_parent=self.temp_parent
            )

        self.assertEqual(23, raised.exception.returncode)
        self.assertEqual("untouched\n", actual.read_text())

    def test_matching_clean_generation_preserves_original_and_ignores_caches(self):
        actual = self._actual("nested/contract.txt", "canonical\n")
        self._actual(".DS_Store", "finder metadata")
        self._actual("__pycache__/contract.pyc", "bytecode")
        self._actual("crate/target/cache.bin", "rust cache")
        self._actual("crate/Cargo.lock", "cargo cache")
        generator = self._generator(
            "nested = generated / 'nested'\n"
            "nested.mkdir()\n"
            "(nested / 'contract.txt').write_text('canonical\\n')\n"
        )

        check_generated(
            self.project, generator=generator, temp_parent=self.temp_parent
        )

        self.assertEqual("canonical\n", actual.read_text())
        self.assertEqual("finder metadata", (self.project / "schema/generated/.DS_Store").read_text())

    def test_only_the_top_level_schema_generated_tree_is_excluded_from_inputs(self):
        self._actual("contract.txt", "canonical\n")
        nested_input = self.project / "schema/extensions/generated/source.yaml"
        nested_input.parent.mkdir(parents=True)
        nested_input.write_text("kept: true\n")
        generator = self._generator(
            "nested = Path.cwd() / 'schema/extensions/generated/source.yaml'\n"
            "assert nested.read_text() == 'kept: true\\n'\n"
            "(generated / 'contract.txt').write_text('canonical\\n')\n"
        )

        check_generated(
            self.project,
            generator=generator,
            temp_parent=self.temp_parent,
        )

    def test_temporary_tree_is_removed_after_success_and_failure(self):
        self._actual("contract.txt", "canonical\n")
        success = self._generator(
            "(generated / 'contract.txt').write_text('canonical\\n')\n",
            "success.py",
        )
        failure = self._generator("raise SystemExit(19)\n", "failure.py")

        check_generated(self.project, generator=success, temp_parent=self.temp_parent)
        self.assertEqual([], list(self.temp_parent.iterdir()))
        with self.assertRaises(subprocess.CalledProcessError):
            check_generated(
                self.project, generator=failure, temp_parent=self.temp_parent
            )
        self.assertEqual([], list(self.temp_parent.iterdir()))

    def test_temporary_tree_is_removed_on_term_hup_and_int(self):
        self._actual("contract.txt", "canonical\n")
        for signum in (signal.SIGTERM, signal.SIGHUP, signal.SIGINT):
            with self.subTest(signal=signal.Signals(signum).name):
                marker = self.base / f"started-{signum}"
                descendant_marker = self.base / f"descendant-{signum}"
                child_code = (
                    "import time; from pathlib import Path; time.sleep(0.4); "
                    f"Path({str(descendant_marker)!r}).write_text('orphaned')"
                )
                if signum == signal.SIGTERM:
                    generator_body = (
                        "import signal\n"
                        "import subprocess\n"
                        "import sys\n"
                        "def exit_with_descendant(*_):\n"
                        f"    subprocess.Popen((sys.executable, '-c', {child_code!r}))\n"
                        "    raise SystemExit(143)\n"
                        "signal.signal(signal.SIGTERM, exit_with_descendant)\n"
                        f"Path({str(marker)!r}).write_text('started')\n"
                        "import time\n"
                        "time.sleep(60)\n"
                    )
                else:
                    generator_body = (
                        f"Path({str(marker)!r}).write_text('started')\n"
                        "import subprocess\n"
                        "import sys\n"
                        f"subprocess.Popen((sys.executable, '-c', {child_code!r}))\n"
                        "import time\n"
                        "time.sleep(60)\n"
                    )
                generator = self._generator(generator_body, f"slow-{signum}.py")
                runner = (
                    "from pathlib import Path; "
                    "import sys; "
                    "from tools.check_generated import check_generated; "
                    f"check_generated(Path({str(self.project)!r}), "
                    f"generator=(sys.executable, {str(generator[1])!r}), "
                    f"temp_parent=Path({str(self.temp_parent)!r}))"
                )
                process = subprocess.Popen(
                    (sys.executable, "-c", runner),
                    cwd=Path(__file__).resolve().parents[1],
                )
                deadline = time.monotonic() + 10
                while not marker.exists() and time.monotonic() < deadline:
                    time.sleep(0.02)
                self.assertTrue(marker.exists(), "generator did not start")

                os.kill(process.pid, signum)
                self.assertEqual(128 + signum, process.wait(timeout=10))
                self.assertEqual([], list(self.temp_parent.iterdir()))
                time.sleep(0.6)
                self.assertFalse(
                    descendant_marker.exists(),
                    "generator descendants must be terminated before cleanup",
                )


if __name__ == "__main__":
    unittest.main()
