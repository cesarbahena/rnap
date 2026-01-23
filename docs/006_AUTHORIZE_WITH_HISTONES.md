# 006 Authorize With Histones

## Capability

Add Histone-based authorization and contextual evaluation to the working vertical path.

## Contract

Use the model in [DOMAIN_MODEL.md](DOMAIN_MODEL.md).

Slice 006 implements:

- `TfClass`
- `Histone`
- `HistoneMark`
- `HistoneValueType`
- `HistoneValue`
- `HistoneTarget`
- `ChromatinState`

## Behavior

- Permissions do not exist independently from Histones.
- TfClass authorization behavior is defined through HistoneMarks.
- TfClass definitions are flat and non-inheritable.
- Composition occurs through multiple TfClasses on a Tf.
- Histones may exist at Insulator or Genome scope.
- Genome Histones extend Insulator Histones.
- `Histone.key` is unique within an Insulator.
- Only one active HistoneMark may exist per `(target, histone)` pair.
- Multi-valued attributes use vector value types.
- More specific marks override broader marks unless the broader state is `ConstitutiveHeterochromatin`.
- Auditable records degrade rather than hard-delete.

## Resolution

```text
TfClass histones
+ Tf histones
+ Genome/Gene/Allele/Sequence context histones
```

Resolution defaults to deny.

## Implementation Contract

Pending implementation.

## Approved Tests

Pending.
