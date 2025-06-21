use crate::ast::{fact::FactType, Label};
use alloc::vec::Vec;
use crate::ast::fact::{FactTypeRef, FieldName};

struct Specification {
    result_name: Label,
    arguments: Vec<ArgumentRef>,
    conditions: Vec<ConditionRef>,
    projections: Vec<SpecificationRef>, // Projections can't have arguments.
}
struct SpecificationRef(u32);

struct Argument {
    name: IntrinsicValueNameRef,
    fact_type: FactTypeRef,
}
struct ArgumentRef(u32);

struct IntrinsicValueName(Label);
struct IntrinsicValueNameRef(u32);

struct Condition {
    name: IntrinsicValueNameRef,
    fact_type: FactTypeRef,
    clauses: Vec<ClauseRef>,
}
struct ConditionRef(u32);

enum Clause {
    Equal(EqualityRef),
    NotEqual(ConditionRef),
}
struct ClauseRef(u32);

struct Equality {
    left: PathRef,
    right: PathRef,
}
struct EqualityRef(u32);

struct Path {
    intrinsic: IntrinsicValueName,
    properties: Vec<FieldName>
}
struct PathRef(u32);