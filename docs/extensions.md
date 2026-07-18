# Extension conventions

`extras` preserves producer data that core PHDS cannot model. Keys SHOULD use
`namespace.field_name`; the namespace must be controlled or unambiguously
identified by the producer. Reverse-domain namespaces are recommended for
public interchange, for example `com.example.energy_score`.

Consumers MUST treat unknown extension keys as optional data and preserve them
during a lossless round trip. An extension key MUST NOT override or change the
meaning of a canonical PHDS field. PHDS does not define a machine-readable
extension registry in v0.2.
