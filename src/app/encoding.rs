use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum SequenceType {
    String,
    StringVec,
    Int,
    IntVec,
    Float,
    FloatVec,
    Bool,
    BoolVec,
    Gene,
    GeneVec,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncodingType {
    RNA(RnaType),
    GRN(GrnType),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrnType {
    Promoter,
    Enhancer,
    PIWI,
    Spacers,
    Telomere,
    Centromere,
    Silencer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RnaType {
    Translation(TranslationRnaType),
    Regulatory(RegulatoryRnaType),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum TranslationRnaType {
    ERna,
    MRna,
    RRna,
    TRna,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegulatoryRnaType {
    SnRna,
    ScaRna,
    SiRna,
    TmRna,
    MiRna,
    PiRna,
    SnoRna,
    CrRna,
    TracrRna,
    LncRna,
    CircRna,
    SgRna,
}
