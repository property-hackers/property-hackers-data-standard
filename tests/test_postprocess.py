"""The postprocess patcher must be idempotent and fail closed on drift."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]


def _load_tool():
    path = ROOT / "tools" / "postprocess_generated.py"
    spec = importlib.util.spec_from_file_location("postprocess_generated", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class ReplaceOnceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tool = _load_tool()

    def replace(self, text, old="OLD", new="NEW"):
        return self.tool._replace_once(text, old, new, label="test")

    def test_patches_single_marker(self):
        self.assertEqual("a NEW b", self.replace("a OLD b"))

    def test_already_patched_is_a_no_op(self):
        self.assertEqual("a NEW b", self.replace("a NEW b"))

    def test_prefix_overlapping_markers_patch_and_reapply(self):
        # The TypeScript enum marker's replacement is a prefix of its target,
        # so "new" is present even in the unpatched state.
        old, new = "enum {\n    \n", "enum {\n"
        patched = self.tool._replace_once(f"x {old} y", old, new, label="test")
        self.assertEqual(f"x {new} y", patched)
        self.assertEqual(patched, self.tool._replace_once(patched, old, new, label="test"))

    def test_missing_marker_raises(self):
        with self.assertRaises(RuntimeError):
            self.replace("neither marker present")

    def test_duplicate_old_markers_raise(self):
        with self.assertRaises(RuntimeError):
            self.replace("OLD and OLD again")

    def test_duplicate_new_markers_raise(self):
        # Two copies of the patched form means the file is not in a state this
        # script understands; it must fail closed, not silently continue.
        with self.assertRaises(RuntimeError):
            self.replace("NEW and NEW again")


if __name__ == "__main__":
    unittest.main()
