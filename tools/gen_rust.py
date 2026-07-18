"""Generate Rust with PHDS's strict-object serde contract.

LinkML 1.11.1 exposes a custom Jinja environment on ``RustGenerator`` but its
Rust CLI does not expose a template-directory option. This wrapper keeps every
upstream template except the project-owned struct override.
"""

from __future__ import annotations

import argparse
from pathlib import Path
import re

from jinja2 import ChoiceLoader, Environment, FileSystemLoader, PackageLoader, PrefixLoader
from linkml.generators.rustgen import RustGenerator
from linkml.generators.rustgen import rustgen as rustgen_module
from linkml.generators.rustgen.template import Import, RustProperty
from linkml.generators.common.template import ObjectImport


TEMPLATE_DIRECTORY = Path(__file__).with_name("rust_templates")
NONBLANK_TRIMMED_PATTERN = (
    r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A"
    r"\u2028\u2029\u202F\u205F\u3000\uFEFF]"
    r"(?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-"
    r"\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])"
)
DECIMAL_STRING_PATTERN = r"^-?[0-9]+(\.[0-9]+)?(?![\s\S])"
ISO_CURRENCY_PATTERN = r"^[A-Z]{3}(?![\s\S])"
ISO_COUNTRY_PATTERN = r"^[A-Z]{2}(?![\s\S])"
STRING_PATTERN_VALIDATORS = {
    NONBLANK_TRIMMED_PATTERN: "nonblank_trimmed_string",
    DECIMAL_STRING_PATTERN: "decimal_string",
    ISO_CURRENCY_PATTERN: "iso_currency",
    ISO_COUNTRY_PATTERN: "iso_country",
}

FIXED_OFFSET_DATETIME = "DateTime<FixedOffset>"
for linkml_datetime in ("XSDDateTime", "datetime"):
    rustgen_module.PYTHON_TO_RUST[linkml_datetime] = FIXED_OFFSET_DATETIME
rustgen_module.RUST_IMPORTS[FIXED_OFFSET_DATETIME] = Import(
    module="chrono",
    features=["serde"],
    version="0.4.41",
    objects=[ObjectImport(name="DateTime"), ObjectImport(name="FixedOffset")],
)


class PHDSRustProperty(RustProperty):
    """Rust field carrying PHDS's portable wire constraints."""

    validation_kind: str | None = None
    wire_name: str | None = None


class PHDSRustGenerator(RustGenerator):
    """Rust generator with collision-free primitive and slot aliases."""

    def generate_enum(self, enum):
        result = super().generate_enum(enum)
        seen: set[str] = set()
        for item in result.enum.items:
            if not re.fullmatch(r"(?:r#)?[A-Za-z_][A-Za-z0-9_]*", item.variant):
                parts = re.findall(r"[A-Za-z0-9]+", item.text)
                variant = "".join(part[:1].upper() + part[1:] for part in parts)
                if not variant or variant[0].isdigit():
                    variant = f"Value{variant}"
                item.variant = variant
            if item.variant in seen:
                raise ValueError(
                    f"Rust enum variant collision in {result.enum.name}: "
                    f"{item.variant}"
                )
            seen.add(item.variant)
        return result

    def gen_struct_or_subtype_enum(self, cls):
        result = super().gen_struct_or_subtype_enum(cls)
        if result is not None and cls.name == "CodeableConcept":
            result.struct_names.append("CodeableConcept")
        return result

    def gen_poly_trait(self, cls):
        result = super().gen_poly_trait(cls)
        for subtype in result.subtypes:
            if subtype.enum_name != "CodeableConceptOrSubtype":
                continue
            for attribute in subtype.attrs:
                attribute.cases.append("CodeableConcept")
        return result

    def render(self, mode=None):
        rendered = super().render(mode=mode)
        if getattr(self, "omit_poly", False) and hasattr(rendered, "extra_files"):
            rendered.file.emit_poly = False
            rendered.extra_files.pop("poly", None)
            rendered.extra_files.pop("poly_containers", None)
        primitive_aliases = {alias.name: alias for alias in rendered.file.types}
        slots = []
        for alias in rendered.file.slots:
            primitive = primitive_aliases.get(alias.name)
            if primitive is None:
                slots.append(alias)
                continue
            slot_type = primitive.type_ if alias.type_ == alias.name else alias.type_
            slot_signature = (
                slot_type,
                bool(alias.multivalued),
                bool(alias.class_range),
                alias.slot_range_as_union,
            )
            primitive_signature = (
                primitive.type_,
                bool(primitive.multivalued),
                bool(primitive.class_range),
                primitive.slot_range_as_union,
            )
            if slot_signature != primitive_signature:
                raise ValueError(
                    f"incompatible Rust alias collision: {alias.name}"
                )
        rendered.file.slots = slots
        return rendered

    def generate_attribute(self, attr, cls):
        result = super().generate_attribute(attr, cls)
        validation_kind = None
        if attr.pattern and attr.range not in self.schemaview.all_enums():
            validation_kind = STRING_PATTERN_VALIDATORS.get(attr.pattern)
            if validation_kind is None:
                raise ValueError(
                    f"unsupported Rust string pattern on {cls.name}.{attr.name}: "
                    f"{attr.pattern}"
                )
        generated_name = result.attribute.name
        wire_name = attr.name if generated_name != attr.name else None
        if validation_kind is not None or wire_name is not None:
            values = {
                name: getattr(result.attribute, name)
                for name in RustProperty.model_fields
            }
            result.attribute = PHDSRustProperty(
                **values,
                validation_kind=validation_kind,
                wire_name=wire_name,
            )
        return result


def template_environment() -> Environment:
    return Environment(
        loader=ChoiceLoader(
            [
                FileSystemLoader(TEMPLATE_DIRECTORY),
                PrefixLoader(
                    {
                        "upstream": PackageLoader(
                            "linkml.generators.rustgen", "templates"
                        )
                    }
                ),
                PackageLoader("linkml.generators.rustgen", "templates"),
            ]
        ),
        trim_blocks=True,
        lstrip_blocks=True,
    )


def generate(
    schema: Path,
    output: Path,
    *,
    force: bool = False,
    serde: bool = False,
    omit_poly: bool = False,
) -> str:
    generator = PHDSRustGenerator(
        schema,
        output=output,
        pyo3=False,
        serde=serde,
        _environment=template_environment(),
    )
    generator.omit_poly = omit_poly
    result = generator.serialize(force=force)
    if omit_poly:
        for name in ("poly.rs", "poly_containers.rs"):
            stale_path = output / "src" / name
            if stale_path.exists():
                stale_path.unlink()
    return result


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("schema", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--serde", action="store_true")
    parser.add_argument("--omit-poly", action="store_true")
    args = parser.parse_args()
    generate(
        args.schema,
        args.output,
        force=args.force,
        serde=args.serde,
        omit_poly=args.omit_poly,
    )


if __name__ == "__main__":
    main()
