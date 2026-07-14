set shell := ["bash", "-cu"]

# create .venv and install the LinkML toolchain
venv:
    python3 -m venv .venv
    .venv/bin/pip install -q -r tools/requirements.txt

# regenerate all artifacts from the LinkML source of truth
gen:
    .venv/bin/gen-json-schema schema/profiles.yaml > schema/generated/phds.schema.json
    .venv/bin/gen-pydantic    schema/profiles.yaml > schema/generated/phds_pydantic.py
    .venv/bin/gen-typescript  schema/profiles.yaml > schema/generated/phds.ts.tmp
    printf '// ISO 8601 date string (shim: gen-typescript emits the LinkML type name verbatim)
type date = string;

' | cat - schema/generated/phds.ts.tmp > schema/generated/phds.ts
    rm schema/generated/phds.ts.tmp
    mkdir -p schema/generated/phds-rust
    .venv/bin/gen-rust schema/profiles.yaml --output schema/generated/phds-rust --force
    .venv/bin/python tools/postprocess_generated.py

# examples must validate; counter_examples must fail
validate:
    for f in examples/*.json; do \
      .venv/bin/linkml-validate -s schema/profiles.yaml -C PropertyProfile "$f" && echo "PASS $f"; \
    done
    for f in counter_examples/*.json; do \
      if .venv/bin/linkml-validate -s schema/profiles.yaml -C PropertyProfile "$f" > /dev/null 2>&1; \
      then echo "FAIL (should not validate): $f"; exit 1; \
      else echo "PASS (correctly rejected) $f"; fi; \
    done

# Generated client contracts must agree with the LinkML/JSON Schema behavior.
test-generated:
    .venv/bin/python -m unittest discover -s tests -p 'test_*.py' -v

check: gen validate test-generated
