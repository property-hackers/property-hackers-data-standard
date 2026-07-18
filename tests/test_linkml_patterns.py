from __future__ import annotations

import json
from pathlib import Path
import re
import subprocess
import tempfile
import unittest

from linkml.generators.jsonschemagen import JsonSchemaGenerator
from linkml.generators.pydanticgen import PydanticGenerator

from tools.gen_typescript import EnumAwareTypescriptGenerator


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "tests/fixtures/linkml/physical_state_patterns.yaml"
OPTIONAL_SCHEMA = ROOT / "tests/fixtures/linkml/optional_rating_patterns.yaml"


def _pydantic_class_body(source: str, name: str) -> str:
    match = re.search(rf"^class {re.escape(name)}\([^\n]*\):\n", source, re.MULTILINE)
    if match is None:
        raise AssertionError(f"no exact Pydantic class for {name!r}")
    next_class = re.search(r"^class \w+\([^\n]*\):\n", source[match.end() :], re.MULTILINE)
    end = match.end() + next_class.start() if next_class else len(source)
    return source[match.end() : end]


def _typescript_interface(source: str, name: str) -> tuple[tuple[str, ...], str]:
    match = re.search(
        rf"^export interface {re.escape(name)}(?: extends (?P<bases>[^{{]+))? \{{\n",
        source,
        re.MULTILINE,
    )
    if match is None:
        raise AssertionError(f"no exact TypeScript interface for {name!r}")
    end = source.index("\n}", match.end())
    bases = tuple(
        base.strip() for base in (match.group("bases") or "").split(",") if base.strip()
    )
    return bases, source[match.end() : end]


def _typescript_effective_body(source: str, name: str) -> str:
    bases, body = _typescript_interface(source, name)
    return "\n".join([body, *(_typescript_effective_body(source, base) for base in bases)])


def _rust_struct_body(source: str, name: str) -> str:
    match = re.search(rf"^pub struct {re.escape(name)} \{{\n", source, re.MULTILINE)
    if match is None:
        raise AssertionError(f"no exact Rust struct for {name!r}")
    end = source.index("\n}", match.end())
    return source[match.end() : end]


class PhysicalStatePatternTests(unittest.TestCase):
    def test_mixins_flatten_and_state_identity_survives_every_generator(self):
        json_schema = json.loads(JsonSchemaGenerator(str(SCHEMA)).serialize())
        for class_name in ("Structure", "StructureState"):
            properties = json_schema["$defs"][class_name]["properties"]
            self.assertIn("year_built", properties)
            self.assertIn("condition", properties)
            self.assertIn("id", properties)
        self.assertIn("property", json_schema["$defs"]["Structure"]["properties"])
        self.assertIn("subject", json_schema["$defs"]["StructureState"]["properties"])

        pydantic_source = PydanticGenerator(str(SCHEMA)).serialize()
        typescript_source = EnumAwareTypescriptGenerator(str(SCHEMA)).serialize()
        pydantic_structure = _pydantic_class_body(pydantic_source, "Structure")
        self.assertIn("year_built: Optional[int]", pydantic_structure)
        self.assertIn("condition: Optional[str]", pydantic_structure)
        self.assertIn("id: str", pydantic_structure)
        self.assertIn("property: str", pydantic_structure)
        pydantic_state = _pydantic_class_body(pydantic_source, "StructureState")
        self.assertIn("year_built: Optional[int]", pydantic_state)
        self.assertIn("condition: Optional[str]", pydantic_state)
        self.assertIn("id: str", pydantic_state)
        self.assertIn("subject: str", pydantic_state)

        typescript_structure = _typescript_effective_body(typescript_source, "Structure")
        self.assertIn("year_built?: number,", typescript_structure)
        self.assertIn("condition?: string,", typescript_structure)
        self.assertIn("id: string,", typescript_structure)
        self.assertIn("property: PropertyId,", typescript_structure)
        typescript_state = _typescript_effective_body(typescript_source, "StructureState")
        self.assertIn("year_built?: number,", typescript_state)
        self.assertIn("condition?: string,", typescript_state)
        self.assertIn("id: string,", typescript_state)
        self.assertIn("subject: StructureId,", typescript_state)

        with tempfile.TemporaryDirectory() as tmp:
            subprocess.run(
                [
                    str(ROOT / ".venv/bin/gen-rust"),
                    str(SCHEMA),
                    "--output",
                    tmp,
                    "--force",
                    "--serde",
                ],
                check=True,
                capture_output=True,
                text=True,
            )
            rust_source = (Path(tmp) / "src/lib.rs").read_text(encoding="utf-8")
        rust_structure = _rust_struct_body(rust_source, "Structure")
        self.assertIn("pub year_built: Option<isize>,", rust_structure)
        self.assertIn("pub condition: Option<String>,", rust_structure)
        self.assertIn("pub id: String", rust_structure)
        self.assertIn("pub property: String,", rust_structure)
        rust_state = _rust_struct_body(rust_source, "StructureState")
        self.assertIn("pub year_built: Option<isize>,", rust_state)
        self.assertIn("pub condition: Option<String>,", rust_state)
        self.assertIn("pub id: String", rust_state)
        self.assertIn("pub subject: String,", rust_state)

    def test_optional_import_constrains_without_entering_base_closure(self):
        base = json.loads(JsonSchemaGenerator(str(SCHEMA)).serialize())
        optional = json.loads(JsonSchemaGenerator(str(OPTIONAL_SCHEMA)).serialize())
        self.assertNotIn("OptionalStructureState", base["$defs"])
        self.assertNotIn("OptionalConditionCode", base["$defs"])
        condition = optional["$defs"]["OptionalStructureState"]["properties"]["condition"]
        self.assertEqual("#/$defs/OptionalConditionCode", condition["$ref"])

        pydantic_source = PydanticGenerator(str(OPTIONAL_SCHEMA)).serialize()
        typescript_source = EnumAwareTypescriptGenerator(str(OPTIONAL_SCHEMA)).serialize()
        pydantic_optional_state = _pydantic_class_body(
            pydantic_source, "OptionalStructureState"
        )
        self.assertIn("condition: OptionalConditionCode", pydantic_optional_state)
        self.assertIn("condition: OptionalConditionCode", typescript_source)

        with tempfile.TemporaryDirectory() as tmp:
            subprocess.run(
                [
                    str(ROOT / ".venv/bin/gen-rust"),
                    str(OPTIONAL_SCHEMA),
                    "--output",
                    tmp,
                    "--force",
                    "--serde",
                ],
                check=True,
                capture_output=True,
                text=True,
            )
            rust_source = (Path(tmp) / "src/lib.rs").read_text(encoding="utf-8")
        self.assertIn("pub struct OptionalStructureState", rust_source)
        self.assertIn("pub condition: OptionalConditionCode", rust_source)


if __name__ == "__main__":
    unittest.main()
