// Inspired by https://github.com/sampsyo/flatcalc/tree/flat

use alloc::vec::Vec;
use crate::ast::{AstError, Label};

// Fact AST
pub struct Fact {
    name: FactTypeRef,
    fields: Vec<FieldTypeRef>,
}
pub struct FactRef(u32);
pub struct FactType(Label);
pub struct FactTypeRef(u32);

// Field AST
pub struct Field {
    name: FieldNameRef,
    field_type: FieldTypeRef,
}
pub struct FieldName(Label);
pub struct FieldNameRef(u32);

pub struct FieldTypeRef(u32);
pub enum FieldType {
    Bool,
    String,
    Decimal,
    Integer,
    OptionalRelation(FactRef),
    Relation(FactRef),
    MultipleRelation(FactRef),
}
