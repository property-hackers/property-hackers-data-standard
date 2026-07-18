from __future__ import annotations

import importlib.util
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[1]


class GeneratedOutputNormalizerTests(unittest.TestCase):
    def test_output_ends_with_exactly_one_newline(self):
        path = ROOT / "tools/normalize_generated_output.py"
        spec = importlib.util.spec_from_file_location("normalize_generated_output", path)
        self.assertIsNotNone(spec)
        self.assertIsNotNone(spec.loader)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)

        for source in ("value", "value\n", "value\n\n", "value\r\n\r\n"):
            with self.subTest(source=repr(source)):
                self.assertEqual("value\n", module.normalize_output(source))


if __name__ == "__main__":
    unittest.main()
