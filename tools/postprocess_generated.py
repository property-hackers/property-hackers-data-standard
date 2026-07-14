"""Apply deterministic compatibility fixes to LinkML-generated clients."""

from __future__ import annotations

from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
GENERATED = ROOT / "schema" / "generated"


def _replace_once(text: str, old: str, new: str, *, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise RuntimeError(f"expected one {label} marker, found {count}")
    return text.replace(old, new, 1)


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


def _patch_rust() -> None:
    path = GENERATED / "phds-rust" / "src" / "lib.rs"
    text = path.read_text(encoding="utf-8")
    text = _replace_once(
        text,
        "#[cfg_attr(feature = \"serde\", derive(Serialize, Deserialize))]\n"
        "pub enum AssessorStatus {",
        "#[cfg_attr(feature = \"serde\", derive(Serialize, Deserialize))]\n"
        "#[cfg_attr(feature = \"serde\", serde(rename_all = \"snake_case\"))]\n"
        "pub enum AssessorStatus {",
        label="Rust AssessorStatus enum",
    )
    text += """

#[cfg(all(test, feature = "serde"))]
mod assessor_status_serde_tests {
    use super::AssessorStatus;

    #[test]
    fn canonical_wire_values_round_trip() {
        let cases = [
            (AssessorStatus::Success, "success"),
            (AssessorStatus::NotFound, "not_found"),
            (AssessorStatus::Timeout, "timeout"),
            (AssessorStatus::ApiError, "api_error"),
            (AssessorStatus::ParseError, "parse_error"),
            (AssessorStatus::InvalidAddress, "invalid_address"),
            (AssessorStatus::Ambiguous, "ambiguous"),
        ];

        for (value, wire) in cases {
            let encoded = serde_yml::to_string(&value).expect("serialize assessor status");
            assert_eq!(encoded.trim(), wire);
            let decoded: AssessorStatus =
                serde_yml::from_str(wire).expect("deserialize assessor status");
            assert_eq!(decoded, value);
        }
    }
}
"""
    path.write_text(text, encoding="utf-8")


def main() -> None:
    _patch_typescript()
    _patch_rust()


if __name__ == "__main__":
    main()
