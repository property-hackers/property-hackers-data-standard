from __future__ import annotations

from collections.abc import Mapping
from dataclasses import dataclass
import json
import math
import re
from typing import Any


class GenerationError(ValueError):
    pass


@dataclass(frozen=True)
class Context:
    source_name: str
    definitions: Mapping[str, Mapping[str, Any]]

    def fail(self, pointer: tuple[str, ...], message: str) -> GenerationError:
        encoded = "/".join(
            part.replace("~", "~0").replace("/", "~1") for part in pointer
        )
        return GenerationError(f"{self.source_name}:#/{encoded}: {message}")


def _typescript_string(value: str) -> str:
    return json.dumps(value, ensure_ascii=False)


def _identifier(
    value: str,
    context: Context,
    pointer: tuple[str, ...],
) -> str:
    if not re.fullmatch(r"[A-Za-z_$][A-Za-z0-9_$]*", value):
        raise context.fail(pointer, f"invalid TypeScript export name: {value!r}")
    return value


ANNOTATION_KEYWORDS = frozenset(
    {
        "$id",
        "$schema",
        "$defs",
        "title",
        "description",
        "examples",
        "deprecated",
        "default",
        "readOnly",
        "metamodel_version",
        "version",
    }
)
VALIDATION_KEYWORDS = frozenset(
    {
        "$ref",
        "type",
        "properties",
        "required",
        "additionalProperties",
        "pattern",
        "format",
        "minLength",
        "maxLength",
        "minimum",
        "maximum",
        "exclusiveMinimum",
        "exclusiveMaximum",
        "multipleOf",
        "enum",
        "const",
        "items",
        "prefixItems",
        "minItems",
        "maxItems",
        "uniqueItems",
        "minProperties",
        "maxProperties",
        "anyOf",
        "allOf",
        "oneOf",
        "not",
        "if",
        "then",
        "else",
    }
)
_METADATA_KEYS = frozenset(
    ("default", "deprecated", "description", "examples", "readOnly", "title")
)
_ROOT_METADATA_KEYS = _METADATA_KEYS | frozenset(
    ("$id", "$schema", "metamodel_version", "version")
)
_NUMERIC_BOUNDS = (
    ("minimum", "min"),
    ("exclusiveMinimum", "gt"),
    ("maximum", "max"),
    ("exclusiveMaximum", "lt"),
)
FORMAT_RENDERERS = {
    "date": "z.iso.date()",
    "date-time": "rfc3339DateTimeSchema",
    "uri": "rfc3986UriSchema",
}

_FORMAT_HELPERS = r'''const rfc3986Path = /^(?:[A-Za-z0-9._~!$&'()*+,;=:@/-]|%[0-9A-Fa-f]{2})*$/;
const rfc3986QueryOrFragment = /^(?:[A-Za-z0-9._~!$&'()*+,;=:@/?-]|%[0-9A-Fa-f]{2})*$/;
const rfc3986UserInfo = /^(?:[A-Za-z0-9._~!$&'()*+,;=:-]|%[0-9A-Fa-f]{2})*$/;
const rfc3986RegisteredName = /^(?:[A-Za-z0-9._~!$&'()*+,;=-]|%[0-9A-Fa-f]{2})*$/;
const rfc3986IpvFuture = /^[Vv][0-9A-Fa-f]+\.[A-Za-z0-9._~!$&'()*+,;=:-]+$/;

const isRfc3986Authority = (value: string): boolean => {
  const at = value.lastIndexOf("@");
  if (at >= 0 && !rfc3986UserInfo.test(value.slice(0, at))) return false;
  const hostAndPort = value.slice(at + 1);
  if (hostAndPort.startsWith("[")) {
    const close = hostAndPort.indexOf("]");
    if (close < 0 || !/^(?::[0-9]*)?$/.test(hostAndPort.slice(close + 1))) return false;
    const literal = hostAndPort.slice(1, close);
    if (rfc3986IpvFuture.test(literal)) return true;
    try {
      new URL(`http://[${literal}]/`);
      return true;
    } catch {
      return false;
    }
  }
  const colon = hostAndPort.lastIndexOf(":");
  const hasPort = colon >= 0;
  const host = hasPort ? hostAndPort.slice(0, colon) : hostAndPort;
  const port = hasPort ? hostAndPort.slice(colon + 1) : "";
  return !host.includes(":") && rfc3986RegisteredName.test(host) && (!hasPort || /^[0-9]*$/.test(port));
};

const isRfc3986Uri = (value: string): boolean => {
  const match = /^([A-Za-z][A-Za-z0-9+.-]*):(?:\/\/([^/?#]*))?([^?#]*)(?:\?([^#]*))?(?:#(.*))?$/.exec(value);
  if (match === null) return false;
  const [, , authority, path, query, fragment] = match;
  return (authority === undefined || isRfc3986Authority(authority))
    && rfc3986Path.test(path)
    && (query === undefined || rfc3986QueryOrFragment.test(query))
    && (fragment === undefined || rfc3986QueryOrFragment.test(fragment));
};

const rfc3986UriSchema = z.string().refine(isRfc3986Uri, { message: "Invalid RFC 3986 URI" });

const publishedLeapSecondDates = new Set([
  "1972-06-30", "1972-12-31", "1973-12-31", "1974-12-31", "1975-12-31",
  "1976-12-31", "1977-12-31", "1978-12-31", "1979-12-31", "1981-06-30",
  "1982-06-30", "1983-06-30", "1985-06-30", "1987-12-31", "1989-12-31",
  "1990-12-31", "1992-06-30", "1993-06-30", "1994-06-30", "1995-12-31",
  "1997-06-30", "1998-12-31", "2005-12-31", "2008-12-31", "2012-06-30",
  "2015-06-30", "2016-12-31",
]);

const isRfc3339DateTime = (value: string): boolean => {
  const match = /^(\d{4})-(\d{2})-(\d{2})[Tt](\d{2}):(\d{2}):(\d{2})(?:\.\d+)?(?:[Zz]|([+-])(\d{2}):(\d{2}))$/.exec(value);
  if (match === null) return false;
  const [, yearText, monthText, dayText, hourText, minuteText, secondText, sign, offsetHourText = "00", offsetMinuteText = "00"] = match;
  const [year, month, day, hour, minute, second, offsetHour, offsetMinute] = [yearText, monthText, dayText, hourText, minuteText, secondText, offsetHourText, offsetMinuteText].map(Number);
  const leapYear = year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
  const daysInMonth = [31, leapYear ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][month - 1];
  if (daysInMonth === undefined || day < 1 || day > daysInMonth || hour > 23 || minute > 59 || second > 60 || offsetHour > 23 || offsetMinute > 59) return false;
  if (second < 60) return true;
  const instant = new Date(0);
  instant.setUTCFullYear(year, month - 1, day);
  instant.setUTCHours(hour, minute, 59, 0);
  const signedOffset = sign === "+" ? offsetHour * 60 + offsetMinute : sign === "-" ? -(offsetHour * 60 + offsetMinute) : 0;
  instant.setUTCMinutes(instant.getUTCMinutes() - signedOffset);
  const utcDate = `${String(instant.getUTCFullYear()).padStart(4, "0")}-${String(instant.getUTCMonth() + 1).padStart(2, "0")}-${String(instant.getUTCDate()).padStart(2, "0")}`;
  return instant.getUTCHours() === 23 && instant.getUTCMinutes() === 59 && publishedLeapSecondDates.has(utcDate);
};

const rfc3339DateTimeSchema = z.string().refine(isRfc3339DateTime, { message: "Invalid RFC 3339 date-time" });'''
_TYPE_KEYWORDS = {
    "null": frozenset(),
    "boolean": frozenset(),
    "string": frozenset(("format", "pattern", "minLength", "maxLength")),
    "integer": frozenset(
        (
            "minimum",
            "exclusiveMinimum",
            "maximum",
            "exclusiveMaximum",
            "multipleOf",
        )
    ),
    "number": frozenset(
        (
            "minimum",
            "exclusiveMinimum",
            "maximum",
            "exclusiveMaximum",
            "multipleOf",
        )
    ),
    "object": frozenset(
        (
            "properties",
            "required",
            "additionalProperties",
            "minProperties",
            "maxProperties",
        )
    ),
    "array": frozenset(
        ("items", "prefixItems", "minItems", "maxItems", "uniqueItems")
    ),
}
_KNOWN_TYPE_KEYWORDS = frozenset().union(*_TYPE_KEYWORDS.values())


def _json_value(value: object) -> str:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))


def _metadata(
    schema: Mapping[str, object], *, document_root: bool = False
) -> str:
    metadata_keys = _ROOT_METADATA_KEYS if document_root else _METADATA_KEYS
    entries = [
        f"{key}: {_json_value(schema[key])}"
        for key in sorted(metadata_keys.intersection(schema))
    ]
    if not entries:
        return ""
    return f".meta({{{', '.join(entries)}}})"


def _number(
    schema: Mapping[str, object],
    key: str,
    context: Context,
    pointer: tuple[str, ...],
) -> str:
    value = schema[key]
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise context.fail(
            pointer + (key,), f"schema keyword '{key}' must be a number"
        )
    if isinstance(value, float) and not math.isfinite(value):
        raise context.fail(
            pointer + (key,), f"schema keyword '{key}' must be a finite number"
        )
    return _json_value(value)


def _nonnegative_integer(
    schema: Mapping[str, object],
    key: str,
    context: Context,
    pointer: tuple[str, ...],
) -> int:
    value = schema[key]
    if isinstance(value, bool) or not isinstance(value, int) or value < 0:
        raise context.fail(
            pointer + (key,),
            f"schema keyword '{key}' must be a non-negative integer",
        )
    return value


def _unsupported_keywords(
    schema: Mapping[str, object],
    allowed: set[str],
    context: Context,
    pointer: tuple[str, ...],
) -> None:
    unsupported = sorted(set(schema).difference(allowed | set(ANNOTATION_KEYWORDS)))
    if unsupported:
        key = unsupported[0]
        raise context.fail(pointer + (key,), f"unsupported schema keyword {key!r}")


def _validate_keywords(
    schema: Mapping[str, object],
    context: Context,
    pointer: tuple[str, ...],
) -> None:
    unsupported = sorted(
        set(schema).difference(ANNOTATION_KEYWORDS | VALIDATION_KEYWORDS)
    )
    if unsupported:
        key = unsupported[0]
        raise context.fail(pointer + (key,), f"unsupported schema keyword {key!r}")

    definitions = schema.get("$defs")
    if definitions is not None:
        if not isinstance(definitions, Mapping):
            raise context.fail(
                pointer + ("$defs",),
                "schema keyword '$defs' must be an object",
            )
        for name, definition in definitions.items():
            definition_pointer = pointer + ("$defs", str(name))
            if isinstance(definition, bool):
                continue
            if not isinstance(definition, Mapping):
                raise context.fail(
                    definition_pointer,
                    "definition schema must be an object or boolean",
                )
            _validate_keywords(definition, context, definition_pointer)

    properties = schema.get("properties")
    if isinstance(properties, Mapping):
        for name, nested in properties.items():
            nested_pointer = pointer + ("properties", str(name))
            if not isinstance(nested, Mapping):
                raise context.fail(
                    nested_pointer, "property schema must be an object"
                )
            _validate_keywords(nested, context, nested_pointer)

    for key in ("items", "additionalProperties", "not", "if", "then", "else"):
        if key not in schema:
            continue
        nested = schema[key]
        if isinstance(nested, bool):
            continue
        if not isinstance(nested, Mapping):
            raise context.fail(
                pointer + (key,), "schema must be an object or boolean"
            )
        _validate_keywords(nested, context, pointer + (key,))

    prefix_items = schema.get("prefixItems")
    if isinstance(prefix_items, list):
        for index, nested in enumerate(prefix_items):
            nested_pointer = pointer + ("prefixItems", str(index))
            if not isinstance(nested, Mapping):
                raise context.fail(
                    nested_pointer, "array item schema must be an object"
                )
            _validate_keywords(nested, context, nested_pointer)

    for key in ("anyOf", "allOf", "oneOf"):
        branches = schema.get(key)
        if not isinstance(branches, list):
            continue
        for index, nested in enumerate(branches):
            if isinstance(nested, bool):
                continue
            nested_pointer = pointer + (key, str(index))
            if not isinstance(nested, Mapping):
                raise context.fail(
                    nested_pointer, "schema must be an object or boolean"
                )
            _validate_keywords(nested, context, nested_pointer)


def _render_scalar(
    schema: Mapping[str, object], context: Context, pointer: tuple[str, ...]
) -> str:
    schema_type = schema.get("type")
    allowed = {"type"}

    if schema_type == "null":
        expression = "z.null()"
    elif schema_type == "boolean":
        expression = "z.boolean()"
    elif schema_type == "string":
        allowed.update(("format", "pattern", "minLength", "maxLength"))
        if "format" in schema:
            string_format = schema["format"]
            if (
                not isinstance(string_format, str)
                or string_format not in FORMAT_RENDERERS
            ):
                raise context.fail(
                    pointer + ("format",),
                    f"unsupported string format {string_format!r}",
                )
            expression = FORMAT_RENDERERS[string_format]
        else:
            expression = "z.string()"
        if "pattern" in schema:
            pattern = schema["pattern"]
            if not isinstance(pattern, str):
                raise context.fail(
                    pointer + ("pattern",),
                    "schema keyword 'pattern' must be a string",
                )
            expression += f".regex(new RegExp({_typescript_string(pattern)}))"
        for key, method in (("minLength", "min"), ("maxLength", "max")):
            if key in schema:
                length = _nonnegative_integer(schema, key, context, pointer)
                expression += f".{method}({length})"
    elif schema_type in ("integer", "number"):
        allowed.update(key for key, _ in _NUMERIC_BOUNDS)
        allowed.add("multipleOf")
        expression = "z.number()"
        if schema_type == "integer":
            expression += ".int()"
        for key, method in _NUMERIC_BOUNDS:
            if key in schema:
                expression += f".{method}({_number(schema, key, context, pointer)})"
        if "multipleOf" in schema:
            multiple = schema["multipleOf"]
            rendered = _number(schema, "multipleOf", context, pointer)
            if multiple <= 0:
                raise context.fail(
                    pointer + ("multipleOf",),
                    "schema keyword 'multipleOf' must be greater than zero",
                )
            expression += f".multipleOf({rendered})"
    else:
        raise context.fail(
            pointer + ("type",), f"unsupported schema type {schema_type!r}"
        )

    _unsupported_keywords(schema, allowed, context, pointer)
    return expression


def _render_literal(
    value: object, context: Context, pointer: tuple[str, ...]
) -> str:
    if value is None or isinstance(value, (str, bool, int)):
        return f"z.literal({_json_value(value)})"
    if isinstance(value, float):
        if not math.isfinite(value):
            raise context.fail(pointer, "literal numbers must be finite")
        return f"z.literal({_json_value(value)})"
    if isinstance(value, list):
        items = [
            _render_literal(item, context, pointer + (str(index),))
            for index, item in enumerate(value)
        ]
        return f"z.tuple([{', '.join(items)}])"
    if isinstance(value, Mapping):
        properties = []
        for key, item in value.items():
            if not isinstance(key, str):
                raise context.fail(pointer, "literal object keys must be strings")
            rendered = _render_literal(item, context, pointer + (key,))
            properties.append(f"{_typescript_string(key)}: {rendered}")
        return f"z.strictObject({{{', '.join(properties)}}})"
    raise context.fail(pointer, f"unsupported literal value {value!r}")


def _render_enum(
    values: object, context: Context, pointer: tuple[str, ...]
) -> str:
    if not isinstance(values, list) or not values:
        raise context.fail(pointer, "schema keyword 'enum' must be a non-empty array")
    if all(isinstance(value, str) for value in values):
        rendered_values = [_json_value(value) for value in values]
        return f"z.enum([{', '.join(rendered_values)}])"
    literals = [
        _render_literal(value, context, pointer + (str(index),))
        for index, value in enumerate(values)
    ]
    if len(literals) == 1:
        return literals[0]
    return f"z.union([{', '.join(literals)}])"


def _inferred_json_type(value: object) -> str:
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "boolean"
    if isinstance(value, str):
        return "string"
    if isinstance(value, int):
        return "integer"
    if isinstance(value, float):
        return "number"
    if isinstance(value, list):
        return "array"
    if isinstance(value, Mapping):
        return "object"
    raise TypeError(f"unsupported JSON value {value!r}")


def _intersect_sibling_constraints(
    expression: str,
    schema: Mapping[str, object],
    omitted_keyword: str,
    values: list[object],
    context: Context,
    pointer: tuple[str, ...],
    recursive_names: frozenset[str],
) -> str:
    siblings = {
        key: value
        for key, value in schema.items()
        if key != omitted_keyword and key not in ANNOTATION_KEYWORDS
    }
    if not siblings:
        return expression
    if "type" not in siblings:
        inferred_types = list(dict.fromkeys(_inferred_json_type(value) for value in values))
        siblings["type"] = (
            inferred_types[0] if len(inferred_types) == 1 else inferred_types
        )
    constraints = _render_schema(
        siblings,
        context,
        pointer,
        recursive_names=recursive_names,
    )
    return f"z.intersection({expression}, {constraints})"


def _render_reference(
    reference: object,
    context: Context,
    pointer: tuple[str, ...],
    recursive_names: frozenset[str],
) -> str:
    del recursive_names
    name = _reference_name(reference, context, pointer)
    return f"{name}Schema"


def _reference_name(
    reference: object,
    context: Context,
    pointer: tuple[str, ...],
) -> str:
    if not isinstance(reference, str) or not reference.startswith("#/$defs/"):
        raise context.fail(pointer, f"external reference is not supported: {reference!r}")
    token = reference.removeprefix("#/$defs/")
    if not token or "/" in token or re.search(r"~(?![01])", token):
        raise context.fail(pointer, f"invalid local definition reference: {reference!r}")
    name = token.replace("~1", "/").replace("~0", "~")
    if name not in context.definitions:
        raise context.fail(pointer, f"dangling local reference: {reference!r}")
    return name


def _referenced_schema_type(
    name: str,
    context: Context,
    pointer: tuple[str, ...],
    seen: frozenset[str] = frozenset(),
) -> object:
    if name in seen:
        raise context.fail(
            pointer,
            f"cannot determine a schema type for reference target {name!r}",
        )
    target = context.definitions[name]
    target_type = target.get("type")
    if isinstance(target_type, (str, list)):
        return target_type
    if "properties" in target:
        return "object"
    if "$ref" in target:
        referenced_name = _reference_name(
            target["$ref"], context, ("$defs", name, "$ref")
        )
        return _referenced_schema_type(
            referenced_name,
            context,
            pointer,
            seen | frozenset((name,)),
        )
    raise context.fail(
        pointer,
        f"cannot determine a schema type for reference target {name!r}",
    )


def _intersect_reference_siblings(
    expression: str,
    schema: Mapping[str, object],
    referenced_name: str,
    context: Context,
    pointer: tuple[str, ...],
    recursive_names: frozenset[str],
) -> str:
    siblings = {
        key: value
        for key, value in schema.items()
        if key != "$ref" and key not in ANNOTATION_KEYWORDS
    }
    if not siblings:
        return expression
    if "type" not in siblings:
        siblings["type"] = _referenced_schema_type(
            referenced_name,
            context,
            pointer + ("$ref",),
        )
    constraints = _render_schema(
        siblings,
        context,
        pointer,
        recursive_names=recursive_names,
    )
    return f"z.intersection({expression}, {constraints})"


def _schema_references(
    schema: Mapping[str, object],
    context: Context,
    pointer: tuple[str, ...],
) -> frozenset[str]:
    references: set[str] = set()
    if "$ref" in schema:
        references.add(
            _reference_name(schema["$ref"], context, pointer + ("$ref",))
        )
    properties = schema.get("properties")
    if isinstance(properties, Mapping):
        for name, property_schema in properties.items():
            if isinstance(property_schema, Mapping):
                references.update(
                    _schema_references(
                        property_schema,
                        context,
                        pointer + ("properties", str(name)),
                    )
                )
    for key in ("items", "additionalProperties", "not", "if", "then", "else"):
        nested_schema = schema.get(key)
        if isinstance(nested_schema, Mapping):
            references.update(
                _schema_references(
                    nested_schema,
                    context,
                    pointer + (key,),
                )
            )
    prefix_items = schema.get("prefixItems")
    if isinstance(prefix_items, list):
        for index, nested_schema in enumerate(prefix_items):
            if isinstance(nested_schema, Mapping):
                references.update(
                    _schema_references(
                        nested_schema,
                        context,
                        pointer + ("prefixItems", str(index)),
                    )
                )
    for key in ("anyOf", "allOf", "oneOf"):
        branches = schema.get(key)
        if isinstance(branches, list):
            for index, nested_schema in enumerate(branches):
                if isinstance(nested_schema, Mapping):
                    references.update(
                        _schema_references(
                            nested_schema,
                            context,
                            pointer + (key, str(index)),
                        )
                    )
    return frozenset(references)


def _definition_dependencies(
    definitions: Mapping[str, Mapping[str, Any]],
    *,
    context: Context | None = None,
) -> dict[str, frozenset[str]]:
    active_context = context or Context("<memory>", definitions)
    return {
        name: _schema_references(
            definitions[name], active_context, ("$defs", name)
        )
        for name in sorted(definitions)
    }


def _ordered_definition_components(
    dependencies: Mapping[str, frozenset[str]],
) -> list[tuple[str, ...]]:
    components: list[tuple[str, ...]] = []
    indices: dict[str, int] = {}
    lowlinks: dict[str, int] = {}
    stack: list[str] = []
    stacked: set[str] = set()

    def visit(name: str) -> None:
        index = len(indices)
        indices[name] = index
        lowlinks[name] = index
        stack.append(name)
        stacked.add(name)

        for dependency in sorted(dependencies[name]):
            if dependency not in indices:
                visit(dependency)
                lowlinks[name] = min(lowlinks[name], lowlinks[dependency])
            elif dependency in stacked:
                lowlinks[name] = min(lowlinks[name], indices[dependency])

        if lowlinks[name] == indices[name]:
            component = []
            while True:
                member = stack.pop()
                stacked.remove(member)
                component.append(member)
                if member == name:
                    break
            components.append(tuple(sorted(component)))

    for name in sorted(dependencies):
        if name not in indices:
            visit(name)
    return components


def _is_recursive_component(
    component: tuple[str, ...],
    dependencies: Mapping[str, frozenset[str]],
) -> bool:
    return len(component) > 1 or component[0] in dependencies[component[0]]


def _component_uses_object_property_getters(
    component: tuple[str, ...],
    definitions: Mapping[str, Mapping[str, Any]],
    context: Context,
) -> bool:
    recursive_names = frozenset(component)
    for name in component:
        schema = definitions[name]
        if schema.get("type") != "object" and "properties" not in schema:
            return False
        properties = schema.get("properties")
        if not isinstance(properties, Mapping):
            return False
        schema_without_properties = {
            key: value for key, value in schema.items() if key != "properties"
        }
        unsafe_dependencies = recursive_names.intersection(
            _schema_references(
                schema_without_properties,
                context,
                ("$defs", name),
            )
        )
        if unsafe_dependencies:
            return False
    return True


def _render_composition(
    schema: Mapping[str, object],
    context: Context,
    pointer: tuple[str, ...],
    recursive_names: frozenset[str],
) -> str:
    composition_keys = frozenset(
        ("anyOf", "allOf", "oneOf", "not", "if", "then", "else")
    )
    base_schema = {
        key: value
        for key, value in schema.items()
        if key not in composition_keys and key not in ANNOTATION_KEYWORDS
    }
    constraints = []
    if base_schema:
        constraints.append(
            _render_schema(
                base_schema,
                context,
                pointer,
                recursive_names=recursive_names,
            )
        )

    for key in ("anyOf", "allOf", "oneOf"):
        if key not in schema:
            continue
        branches = schema[key]
        if not isinstance(branches, list) or not branches:
            raise context.fail(
                pointer + (key,),
                f"schema keyword '{key}' must be a non-empty array",
            )
        rendered = [
            _render_boolean_or_mapping_schema(
                branch,
                context,
                pointer + (key, str(index)),
                recursive_names,
            )
            for index, branch in enumerate(branches)
        ]
        if key == "allOf":
            composition = rendered[0]
            for branch in rendered[1:]:
                composition = f"z.intersection({composition}, {branch})"
        else:
            composition = (
                rendered[0]
                if len(rendered) == 1
                else f"z.union([{', '.join(rendered)}])"
            )
            if key == "oneOf":
                schemas = f"[{', '.join(rendered)}]"
                composition += (
                    ".superRefine((value, ctx) => { const matchCount = "
                    f"{schemas}.filter((schema) => "
                    "schema.safeParse(value).success).length; "
                    "if (matchCount !== 1) { ctx.addIssue({ code: \"custom\", "
                    "message: \"Value must match exactly one oneOf branch\" }); } })"
                )
        constraints.append(composition)

    if "not" in schema:
        excluded = _render_boolean_or_mapping_schema(
            schema["not"],
            context,
            pointer + ("not",),
            recursive_names,
        )
        constraints.append(
            "z.unknown().refine((value) => !"
            f"{excluded}.safeParse(value).success, "
            '{ message: "Value must not match the excluded schema" })'
        )

    expression = constraints[0] if constraints else "z.unknown()"
    for constraint in constraints[1:]:
        expression = f"z.intersection({expression}, {constraint})"

    if "if" in schema:
        predicate = _render_boolean_or_mapping_schema(
            schema["if"],
            context,
            pointer + ("if",),
            recursive_names,
        )
        then_schema = _render_boolean_or_mapping_schema(
            schema.get("then", True),
            context,
            pointer + ("then",),
            recursive_names,
        )
        else_schema = _render_boolean_or_mapping_schema(
            schema.get("else", True),
            context,
            pointer + ("else",),
            recursive_names,
        )
        expression += (
            ".superRefine((value, ctx) => { const selected = "
            f"{predicate}.safeParse(value).success ? {then_schema} : {else_schema}; "
            "const result = selected.safeParse(value); if (!result.success) { "
            "for (const issue of result.error.issues) { ctx.addIssue(issue); } } })"
        )
    return expression


def _render_type_union(
    schema: Mapping[str, object],
    context: Context,
    pointer: tuple[str, ...],
    recursive_names: frozenset[str],
) -> tuple[str, bool]:
    schema_types = schema["type"]
    if not schema_types or not all(isinstance(value, str) for value in schema_types):
        raise context.fail(
            pointer + ("type",), "schema keyword 'type' must be a non-empty string array"
        )
    nullable = "null" in schema_types
    non_null_types = [value for value in schema_types if value != "null"]
    unknown_keywords = set(schema).difference(
        {"type"} | set(ANNOTATION_KEYWORDS) | set(_KNOWN_TYPE_KEYWORDS)
    )
    if not non_null_types:
        if unknown_keywords:
            key = sorted(unknown_keywords)[0]
            raise context.fail(
                pointer + (key,), f"unsupported schema keyword {key!r}"
            )
        return "z.null()", False

    rendered = []
    for schema_type in non_null_types:
        applicable = _TYPE_KEYWORDS.get(schema_type)
        if applicable is None:
            applicable = frozenset()
        branch_schema = {"type": schema_type}
        for key, value in schema.items():
            if key in applicable or key in unknown_keywords:
                branch_schema[key] = value
        rendered.append(
            _render_schema(
                branch_schema,
                context,
                pointer,
                recursive_names=recursive_names,
            )
        )
    expression = (
        rendered[0]
        if len(rendered) == 1
        else f"z.union([{', '.join(rendered)}])"
    )
    return expression, nullable


def _render_object(
    schema: Mapping[str, object],
    context: Context,
    pointer: tuple[str, ...],
    recursive_names: frozenset[str],
) -> str:
    allowed = {
        "type",
        "properties",
        "required",
        "additionalProperties",
        "minProperties",
        "maxProperties",
    }
    _unsupported_keywords(schema, allowed, context, pointer)

    properties = schema.get("properties", {})
    if not isinstance(properties, Mapping):
        raise context.fail(
            pointer + ("properties",),
            "schema keyword 'properties' must be an object",
        )
    required_value = schema.get("required", [])
    if not isinstance(required_value, list) or not all(
        isinstance(name, str) for name in required_value
    ):
        raise context.fail(
            pointer + ("required",),
            "schema keyword 'required' must be a string array",
        )
    required = set(required_value)
    shape_entries = []
    for property_name, property_schema in properties.items():
        if not isinstance(property_schema, Mapping):
            raise context.fail(
                pointer + ("properties", str(property_name)),
                "property schema must be an object",
            )
        rendered = _render_schema(
            property_schema,
            context,
            pointer + ("properties", str(property_name)),
            property_required=property_name in required,
            recursive_names=recursive_names,
        )
        recursive_property = bool(
            recursive_names.intersection(
                _schema_references(
                    property_schema,
                    context,
                    pointer + ("properties", str(property_name)),
                )
            )
        )
        if recursive_property:
            property_key = str(property_name)
            getter_name = (
                property_key
                if re.fullmatch(r"[A-Za-z_$][A-Za-z0-9_$]*", property_key)
                else _typescript_string(property_key)
            )
            shape_entries.append(
                f"get {getter_name}() {{ return {rendered}; }}"
            )
        else:
            shape_entries.append(
                f"{_typescript_string(str(property_name))}: {rendered}"
            )
    shape = "{" + ", ".join(shape_entries) + "}"

    additional = schema.get("additionalProperties", True)
    if additional is False:
        expression = f"z.strictObject({shape})"
    elif additional is True:
        expression = f"z.looseObject({shape})"
    elif isinstance(additional, Mapping):
        catchall = _render_schema(
            additional,
            context,
            pointer + ("additionalProperties",),
            recursive_names=recursive_names,
        )
        expression = f"z.object({shape}).catchall({catchall})"
    else:
        raise context.fail(
            pointer + ("additionalProperties",),
            "schema keyword 'additionalProperties' must be a boolean or schema",
        )

    for required_name in required_value:
        if required_name not in properties:
            rendered_name = _typescript_string(required_name)
            message = _typescript_string(
                f"Required property {required_name} is missing"
            )
            expression += (
                ".refine((value) => Object.prototype.hasOwnProperty.call(value, "
                f"{rendered_name}), {{ message: {message} }})"
            )
    if "minProperties" in schema:
        minimum = _nonnegative_integer(schema, "minProperties", context, pointer)
        expression += f'.refine((value) => Object.keys(value).length >= {minimum}, {{ message: "Object must have at least {minimum} properties" }})'
    if "maxProperties" in schema:
        maximum = _nonnegative_integer(schema, "maxProperties", context, pointer)
        expression += f'.refine((value) => Object.keys(value).length <= {maximum}, {{ message: "Object must have at most {maximum} properties" }})'
    return expression


_UNIQUE_ITEMS_REFINEMENT = (
    '.refine((value) => { const canonicalize = (item: unknown): string => '
    'item !== null && typeof item === "object" ? '
    'Array.isArray(item) ? `[${item.map(canonicalize).join(",")}]` : '
    '`{${Object.keys(item as Record<string, unknown>).sort().map((key) => '
    '`${JSON.stringify(key)}:${canonicalize((item as Record<string, unknown>)[key])}`).join(",")}}` : '
    'JSON.stringify(item) ?? "undefined"; '
    'return new Set(value.map(canonicalize)).size === value.length; }, '
    '{ message: "Array items must be unique" })'
)


def _render_array(
    schema: Mapping[str, object],
    context: Context,
    pointer: tuple[str, ...],
    recursive_names: frozenset[str],
) -> str:
    allowed = {"type", "items", "prefixItems", "minItems", "maxItems", "uniqueItems"}
    _unsupported_keywords(schema, allowed, context, pointer)

    prefix_items = schema.get("prefixItems")
    if prefix_items is not None:
        if not isinstance(prefix_items, list):
            raise context.fail(
                pointer + ("prefixItems",),
                "schema keyword 'prefixItems' must be an array",
            )
        prefixes = []
        for index, item_schema in enumerate(prefix_items):
            if not isinstance(item_schema, Mapping):
                raise context.fail(
                    pointer + ("prefixItems", str(index)),
                    "array item schema must be an object",
                )
            prefixes.append(
                _render_schema(
                    item_schema,
                    context,
                    pointer + ("prefixItems", str(index)),
                    recursive_names=recursive_names,
                )
            )
        rest = schema.get("items", True)
        if rest is False:
            expression = f"z.tuple([{', '.join(prefixes)}])"
        else:
            rest_expression = _render_boolean_or_mapping_schema(
                rest, context, pointer + ("items",), recursive_names
            )
            expression = f"z.tuple([{', '.join(prefixes)}], {rest_expression})"
    else:
        item_schema = schema.get("items", True)
        item_expression = _render_boolean_or_mapping_schema(
            item_schema, context, pointer + ("items",), recursive_names
        )
        expression = f"z.array({item_expression})"

    for key, method in (("minItems", "min"), ("maxItems", "max")):
        if key in schema:
            count = _nonnegative_integer(schema, key, context, pointer)
            if prefix_items is None:
                expression += f".{method}({count})"
            else:
                operator = ">=" if key == "minItems" else "<="
                expression += f'.refine((value) => value.length {operator} {count}, {{ message: "Array length must be {operator} {count}" }})'
    if "uniqueItems" in schema:
        unique = schema["uniqueItems"]
        if not isinstance(unique, bool):
            raise context.fail(
                pointer + ("uniqueItems",),
                "schema keyword 'uniqueItems' must be a boolean",
            )
        if unique:
            expression += _UNIQUE_ITEMS_REFINEMENT
    return expression


def _render_boolean_or_mapping_schema(
    schema: object,
    context: Context,
    pointer: tuple[str, ...],
    recursive_names: frozenset[str],
) -> str:
    if schema is True:
        return "z.unknown()"
    if schema is False:
        return "z.never()"
    if not isinstance(schema, Mapping):
        raise context.fail(pointer, "schema must be an object or boolean")
    return _render_schema(schema, context, pointer, recursive_names=recursive_names)


def _render_schema(
    schema: Mapping[str, object],
    context: Context,
    pointer: tuple[str, ...],
    *,
    property_required: bool = True,
    recursive_names: frozenset[str] = frozenset(),
    document_root: bool = False,
) -> str:
    _validate_keywords(schema, context, pointer)
    nullable = False
    if not schema:
        expression = "z.unknown()"
    elif any(
        key in schema
        for key in ("anyOf", "allOf", "oneOf", "not", "if", "then", "else")
    ):
        expression = _render_composition(schema, context, pointer, recursive_names)
    elif "$ref" in schema:
        referenced_name = _reference_name(
            schema["$ref"], context, pointer + ("$ref",)
        )
        expression = _render_reference(
            schema["$ref"], context, pointer + ("$ref",), recursive_names
        )
        expression = _intersect_reference_siblings(
            expression,
            schema,
            referenced_name,
            context,
            pointer,
            recursive_names,
        )
    elif "const" in schema:
        expression = _render_literal(
            schema["const"], context, pointer + ("const",)
        )
        expression = _intersect_sibling_constraints(
            expression,
            schema,
            "const",
            [schema["const"]],
            context,
            pointer,
            recursive_names,
        )
    elif "enum" in schema:
        expression = _render_enum(schema["enum"], context, pointer + ("enum",))
        enum_values = schema["enum"]
        if isinstance(enum_values, list):
            expression = _intersect_sibling_constraints(
                expression,
                schema,
                "enum",
                enum_values,
                context,
                pointer,
                recursive_names,
            )
    elif isinstance(schema.get("type"), list):
        expression, nullable = _render_type_union(
            schema, context, pointer, recursive_names
        )
    elif schema.get("type") == "object" or "properties" in schema:
        expression = _render_object(schema, context, pointer, recursive_names)
    elif (
        "type" not in schema
        and _TYPE_KEYWORDS["object"].intersection(schema)
    ):
        object_schema = {"type": "object", **schema}
        object_expression = _render_object(
            object_schema,
            context,
            pointer,
            recursive_names,
        )
        expression = (
            f"z.union([{object_expression}, z.null(), z.boolean(), z.string(), "
            "z.number(), z.array(z.unknown())])"
        )
    elif schema.get("type") == "array":
        expression = _render_array(schema, context, pointer, recursive_names)
    else:
        expression = _render_scalar(schema, context, pointer)

    if nullable:
        expression += ".nullable()"
    if not property_required:
        expression += ".optional()"
    return expression + _metadata(schema, document_root=document_root)


def _jsdoc(description: object) -> list[str]:
    if not isinstance(description, str):
        return []
    lines = ["/**"]
    escaped_lines = (line.replace("*/", "*\\/") for line in description.splitlines())
    lines.extend(f" * {line}" for line in escaped_lines)
    lines.append(" */")
    return lines


def generate_zod(
    schema: Mapping[str, object],
    *,
    root_name: str,
    source_name: str = "<memory>",
) -> str:
    definitions_value = schema.get("$defs", {})
    if not isinstance(definitions_value, Mapping):
        context = Context(source_name=source_name, definitions={})
        raise context.fail(("$defs",), "schema keyword '$defs' must be an object")
    definitions: dict[str, Mapping[str, Any]] = {}
    for definition_name, definition_schema in definitions_value.items():
        if not isinstance(definition_name, str) or not isinstance(
            definition_schema, Mapping
        ):
            context = Context(source_name=source_name, definitions={})
            raise context.fail(
                ("$defs", str(definition_name)),
                "definition schema must be an object",
            )
        definitions[definition_name] = definition_schema

    context = Context(source_name=source_name, definitions=definitions)
    name = _identifier(root_name, context, ("root_name",))
    if name in definitions:
        raise context.fail(
            ("root_name",),
            f"root export name {name!r} collides with definition {name!r}",
        )
    dependencies = _definition_dependencies(definitions, context=context)
    ordered_components = _ordered_definition_components(dependencies)
    for component in ordered_components:
        recursive = _is_recursive_component(component, dependencies)
        getter_safe = not recursive or _component_uses_object_property_getters(
            component,
            definitions,
            context,
        )
        if recursive and not getter_safe:
            raise context.fail(
                ("$defs",),
                "recursive definitions cannot be emitted without weakening "
                f"inference: {', '.join(component)}",
            )
    root_schema = {key: value for key, value in schema.items() if key != "$defs"}
    expression = _render_schema(root_schema, context, (), document_root=True)

    lines = [
        f"// Generated from {source_name} by tools/gen_zod.py. DO NOT EDIT.",
        'import * as z from "zod";',
        "",
        _FORMAT_HELPERS,
        "",
    ]
    for component in ordered_components:
        recursive_names = (
            frozenset(component)
            if _is_recursive_component(component, dependencies)
            else frozenset()
        )
        for definition_name in component:
            exported_name = _identifier(
                definition_name,
                context,
                ("$defs", definition_name),
            )
            definition_schema = definitions[definition_name]
            definition_expression = _render_schema(
                definition_schema,
                context,
                ("$defs", definition_name),
                recursive_names=recursive_names,
            )
            lines.extend(_jsdoc(definition_schema.get("description")))
            lines.append(
                f"export const {exported_name}Schema = {definition_expression};"
            )
            lines.append("")
            lines.append(
                f"export type {exported_name} = z.infer<typeof {exported_name}Schema>;"
            )
            lines.append("")
    lines.extend(_jsdoc(schema.get("description")))
    lines.append(f"export const {name}Schema = {expression};")
    lines.append("")
    lines.append(f"export type {name} = z.infer<typeof {name}Schema>;")
    return "\n".join(lines).rstrip("\n") + "\n"
