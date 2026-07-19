import json
import re
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


class ZodGeneratedSourceTests(unittest.TestCase):
    def test_generated_exports_match_json_schema_definitions(self) -> None:
        cases = (
            (
                "schema/generated/phds.schema.json",
                "schema/generated/phds.zod.ts",
                "Phds",
            ),
            (
                "schema/generated/standards/uad_3_6.schema.json",
                "schema/generated/standards/uad_3_6.zod.ts",
                "Uad36",
            ),
            (
                "schema/generated/standards/boma_building_class.metro.schema.json",
                "schema/generated/standards/boma_building_class.metro.zod.ts",
                "BomaMetro",
            ),
            (
                "schema/generated/standards/boma_building_class.international.schema.json",
                "schema/generated/standards/boma_building_class.international.zod.ts",
                "BomaInternational",
            ),
        )

        for schema_path, zod_path, root_name in cases:
            with self.subTest(zod_path=zod_path):
                schema = json.loads((ROOT / schema_path).read_text())
                source = (ROOT / zod_path).read_text()
                self.assertTrue(
                    source.startswith(
                        f"// Generated from {schema_path} by tools/gen_zod.py. "
                        "DO NOT EDIT.\n"
                    )
                )
                schemas = set(
                    re.findall(r"^export const (\w+)Schema =", source, re.MULTILINE)
                )
                types = set(
                    re.findall(r"^export type (\w+) = z\.infer<", source, re.MULTILINE)
                )
                expected = set(schema["$defs"]) | {root_name}

                self.assertEqual(expected, schemas)
                self.assertEqual(expected, types)
                self.assertNotIn("z.any()", source)
                self.assertTrue(source.endswith("\n"))
                self.assertFalse(source.endswith("\n\n"))


if __name__ == "__main__":
    unittest.main()
