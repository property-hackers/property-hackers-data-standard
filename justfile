set shell := ["bash", "-cu"]

# create .venv and install the LinkML toolchain
venv:
    python3 -m venv .venv
    .venv/bin/pip install -q -r tools/requirements.txt

# regenerate all artifacts from the LinkML source of truth
gen: gen-core gen-standards

gen-core:
    .venv/bin/gen-json-schema schema/capture.yaml | .venv/bin/python tools/normalize_generated_output.py > schema/generated/phds.schema.json
    .venv/bin/gen-jsonld-context schema/capture.yaml --no-metadata -o schema/generated/phds.context.jsonld > /dev/null
    .venv/bin/python tools/fix_jsonld_context.py schema/capture.yaml schema/generated/phds.context.jsonld
    .venv/bin/gen-pydantic schema/capture.yaml --template-dir tools/pydantic_templates > schema/generated/phds_pydantic.py
    .venv/bin/python tools/gen_typescript.py schema/capture.yaml > schema/generated/phds.ts
    mkdir -p schema/generated/phds-rust
    .venv/bin/python tools/gen_rust.py schema/capture.yaml --output schema/generated/phds-rust --force --serde
    .venv/bin/python tools/gen_wire_format_test.py --linkml-schema schema/capture.yaml

gen-standards:
    mkdir -p schema/generated/standards
    .venv/bin/gen-json-schema schema/standards/uad_3_6.yaml --top-class Uad36PropertyProfile | .venv/bin/python tools/normalize_generated_output.py > schema/generated/standards/uad_3_6.schema.json
    .venv/bin/gen-pydantic schema/standards/uad_3_6.yaml > schema/generated/standards/uad_3_6_pydantic.py
    .venv/bin/python tools/gen_typescript.py schema/standards/uad_3_6.yaml > schema/generated/standards/uad_3_6.ts
    rust_schema=$(mktemp); trap 'rm -f "$rust_schema"' EXIT; .venv/bin/gen-linkml schema/standards/uad_3_6.yaml --no-metadata --format yaml -o "$rust_schema"; .venv/bin/python tools/gen_rust.py "$rust_schema" --output schema/generated/standards/uad_3_6-rust --force --serde --omit-poly
    .venv/bin/python tools/gen_wire_format_test.py --linkml-schema schema/standards/uad_3_6.yaml --json-schema schema/generated/standards/uad_3_6.schema.json --rust-lib schema/generated/standards/uad_3_6-rust/src/lib.rs --output schema/generated/standards/uad_3_6-rust/tests/wire_format.rs --crate phds_uad_3_6
    .venv/bin/gen-json-schema schema/standards/boma_building_class_metro.yaml --top-class BomaMetroBuildingClassPropertyProfile | .venv/bin/python tools/normalize_generated_output.py > schema/generated/standards/boma_building_class.metro.schema.json
    .venv/bin/gen-json-schema schema/standards/boma_building_class_international.yaml --top-class BomaInternationalBuildingClassPropertyProfile | .venv/bin/python tools/normalize_generated_output.py > schema/generated/standards/boma_building_class.international.schema.json
    .venv/bin/gen-pydantic schema/standards/boma_building_class.yaml > schema/generated/standards/boma_building_class_pydantic.py
    .venv/bin/python tools/gen_typescript.py schema/standards/boma_building_class.yaml > schema/generated/standards/boma_building_class.ts
    rust_schema=$(mktemp); trap 'rm -f "$rust_schema"' EXIT; .venv/bin/gen-linkml schema/standards/boma_building_class.yaml --no-metadata --format yaml -o "$rust_schema"; .venv/bin/python tools/gen_rust.py "$rust_schema" --output schema/generated/standards/boma_building_class-rust --force --serde --omit-poly
    .venv/bin/python tools/gen_wire_format_test.py --linkml-schema schema/standards/boma_building_class.yaml --json-schema schema/generated/standards/boma_building_class.metro.schema.json --rust-lib schema/generated/standards/boma_building_class-rust/src/lib.rs --output schema/generated/standards/boma_building_class-rust/tests/wire_format.rs --crate phds_boma_building_class

# Examples must pass both validation layers. Structural and semantic negative
# fixtures must fail only their respective layer.
validate:
    for f in examples/PropertyProfile-*.json; do \
      .venv/bin/linkml-validate -s schema/capture.yaml -C PropertyProfile "$f" || exit 1; \
      .venv/bin/python tools/profile_validation.py "$f" || exit 1; \
      echo "PASS $f"; \
    done
    capture_examples=(examples/capture/MlsObservation-*.json); \
    if [ ! -e "${capture_examples[0]}" ]; \
    then echo "FAIL no capture examples found"; exit 1; fi; \
    for f in "${capture_examples[@]}"; do \
      .venv/bin/linkml-validate -s schema/capture.yaml -C MlsObservation "$f" || exit 1; \
      .venv/bin/python tools/profile_validation.py --document-type capture-envelope "$f" || exit 1; \
      echo "PASS $f"; \
    done
    schema_counterexamples=(counter_examples/schema/*.json); \
    if [ ! -e "${schema_counterexamples[0]}" ]; \
    then echo "FAIL no structural counterexamples found"; exit 1; fi; \
    for f in "${schema_counterexamples[@]}"; do \
      if .venv/bin/linkml-validate -s schema/capture.yaml -C PropertyProfile "$f" >/dev/null 2>&1; \
      then echo "FAIL structural counterexample validated: $f"; exit 1; \
      else echo "PASS structurally rejected $f"; fi; \
    done
    semantic_counterexamples=(counter_examples/semantic/*.json); \
    if [ ! -e "${semantic_counterexamples[0]}" ]; \
    then echo "FAIL no semantic counterexamples found"; exit 1; fi; \
    for f in "${semantic_counterexamples[@]}"; do \
      if ! .venv/bin/linkml-validate -s schema/capture.yaml -C PropertyProfile "$f" >/dev/null; \
      then echo "FAIL semantic counterexample is structurally invalid: $f"; exit 1; fi; \
      if .venv/bin/python tools/profile_validation.py "$f" >/dev/null 2>&1; \
      then echo "FAIL semantic counterexample validated: $f"; exit 1; \
      else echo "PASS semantically rejected $f"; fi; \
    done
    .venv/bin/linkml-validate -s schema/standards/uad_3_6.yaml -C Uad36PropertyProfile examples/standards/uad-property-profile.json
    .venv/bin/linkml-validate -s schema/standards/boma_building_class.yaml -C BomaMetroPropertyProfile examples/standards/boma-metro-property-profile.json
    .venv/bin/linkml-validate -s schema/standards/boma_building_class.yaml -C BomaInternationalPropertyProfile examples/standards/boma-international-property-profile.json
    .venv/bin/python tools/profile_validation.py examples/standards/uad-property-profile.json
    .venv/bin/python tools/standards_validation.py examples/standards/uad-property-profile.json
    .venv/bin/python tools/profile_validation.py examples/standards/boma-metro-property-profile.json
    .venv/bin/python tools/standards_validation.py examples/standards/boma-metro-property-profile.json
    .venv/bin/python tools/profile_validation.py examples/standards/boma-international-property-profile.json
    .venv/bin/python tools/standards_validation.py examples/standards/boma-international-property-profile.json
    .venv/bin/linkml-validate -s schema/standards/uad_3_6.yaml -C Uad36PropertyProfile counter_examples/standards/uad-core-semantic-invalid.json >/dev/null
    if .venv/bin/python tools/profile_validation.py counter_examples/standards/uad-core-semantic-invalid.json >/dev/null 2>&1; then echo "FAIL UAD core-semantic counterexample validated"; exit 1; else echo "PASS UAD core-semantic counterexample rejected"; fi
    uad_counterexamples=(uad-missing-scope uad-invalid-condition-code uad-quality-rating-in-condition); for name in "${uad_counterexamples[@]}"; do if .venv/bin/linkml-validate -s schema/standards/uad_3_6.yaml -C Uad36PropertyProfile "counter_examples/standards/$name.json" >/dev/null 2>&1; then echo "FAIL UAD counterexample validated: $name"; exit 1; else echo "PASS UAD counterexample rejected: $name"; fi; done
    boma_metro_counterexamples=(boma-metro-invalid-code boma-system-code-mismatch); for name in "${boma_metro_counterexamples[@]}"; do if .venv/bin/linkml-validate -s schema/standards/boma_building_class.yaml -C BomaMetroPropertyProfile "counter_examples/standards/$name.json" >/dev/null 2>&1; then echo "FAIL BOMA metro counterexample validated: $name"; exit 1; else echo "PASS BOMA metro counterexample rejected: $name"; fi; done
    if .venv/bin/linkml-validate -s schema/standards/boma_building_class.yaml -C BomaInternationalPropertyProfile counter_examples/standards/boma-international-invalid-code.json >/dev/null 2>&1; then echo "FAIL BOMA international invalid code validated"; exit 1; else echo "PASS BOMA international invalid code rejected"; fi
    .venv/bin/linkml-validate -s schema/standards/boma_building_class.yaml -C BomaMetroPropertyProfile counter_examples/standards/boma-market-rating-in-condition.json >/dev/null
    if .venv/bin/python tools/standards_validation.py counter_examples/standards/boma-market-rating-in-condition.json >/dev/null 2>&1; then echo "FAIL BOMA condition placement validated"; exit 1; else echo "PASS BOMA condition placement rejected"; fi

# Generated client contracts must agree with the LinkML/JSON Schema behavior.
test-generated:
    .venv/bin/python -m unittest discover -s tests -p 'test_*.py' -v

# Rust enums must round-trip the canonical snake_case wire values.
test-rust:
    cargo test --manifest-path schema/generated/phds-rust/Cargo.toml --features serde
    cargo test --manifest-path schema/generated/standards/uad_3_6-rust/Cargo.toml --features serde
    cargo test --manifest-path schema/generated/standards/boma_building_class-rust/Cargo.toml --features serde

# Regeneration must be deterministic and must not mutate the checked-out tree.
# The checker builds from empty in an isolated temporary project.
check-generated:
    .venv/bin/python tools/check_generated.py

check: check-generated validate test-generated test-rust
