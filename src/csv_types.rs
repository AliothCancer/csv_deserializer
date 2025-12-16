#![allow(unused)]
use csv_deserializer::create_enum;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Id {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MSSubClass {
    Int(i64),
    Null,
}

create_enum!(MSZoning;
"C (all)" => CSpaceOpenParenallCloseParen,
"FV" => FV,
"RH" => RH,
"RL" => RL,
"RM" => RM,
Null,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum LotFrontage {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LotArea {
    Int(i64),
    Null,
}

create_enum!(Street;
"Grvl" => Grvl,
"Pave" => Pave,
Null,
);

create_enum!(Alley;
"Grvl" => Grvl,
"Pave" => Pave,
Null,
);

create_enum!(LotShape;
"IR1" => IROne,
"IR2" => IRTwo,
"IR3" => IRThree,
"Reg" => Reg,
Null,
);

create_enum!(LandContour;
"Bnk" => Bnk,
"HLS" => HLS,
"Low" => Low,
"Lvl" => Lvl,
Null,
);

create_enum!(Utilities;
"AllPub" => AllPub,
"NoSeWa" => NoSeWa,
Null,
);

create_enum!(LotConfig;
"Corner" => Corner,
"CulDSac" => CulDSac,
"FR2" => FRTwo,
"FR3" => FRThree,
"Inside" => Inside,
Null,
);

create_enum!(LandSlope;
"Gtl" => Gtl,
"Mod" => Mod,
"Sev" => Sev,
Null,
);

create_enum!(Neighborhood;
"Blmngtn" => Blmngtn,
"Blueste" => Blueste,
"BrDale" => BrDale,
"BrkSide" => BrkSide,
"ClearCr" => ClearCr,
"CollgCr" => CollgCr,
"Crawfor" => Crawfor,
"Edwards" => Edwards,
"Gilbert" => Gilbert,
"IDOTRR" => IDOTRR,
"MeadowV" => MeadowV,
"Mitchel" => Mitchel,
"NAmes" => NAmes,
"NPkVill" => NPkVill,
"NWAmes" => NWAmes,
"NoRidge" => NoRidge,
"NridgHt" => NridgHt,
"OldTown" => OldTown,
"SWISU" => SWISU,
"Sawyer" => Sawyer,
"SawyerW" => SawyerW,
"Somerst" => Somerst,
"StoneBr" => StoneBr,
"Timber" => Timber,
"Veenker" => Veenker,
Null,
);

create_enum!(ConditionOne;
"Artery" => Artery,
"Feedr" => Feedr,
"Norm" => Norm,
"PosA" => PosA,
"PosN" => PosN,
"RRAe" => RRAe,
"RRAn" => RRAn,
"RRNe" => RRNe,
"RRNn" => RRNn,
Null,
);

create_enum!(ConditionTwo;
"Artery" => Artery,
"Feedr" => Feedr,
"Norm" => Norm,
"PosA" => PosA,
"PosN" => PosN,
"RRAe" => RRAe,
"RRAn" => RRAn,
"RRNn" => RRNn,
Null,
);

create_enum!(BldgType;
"1Fam" => OneFam,
"2fmCon" => TwofmCon,
"Duplex" => Duplex,
"Twnhs" => Twnhs,
"TwnhsE" => TwnhsE,
Null,
);

create_enum!(HouseStyle;
"1.5Fin" => OnePointFiveFin,
"1.5Unf" => OnePointFiveUnf,
"1Story" => OneStory,
"2.5Fin" => TwoPointFiveFin,
"2.5Unf" => TwoPointFiveUnf,
"2Story" => TwoStory,
"SFoyer" => SFoyer,
"SLvl" => SLvl,
Null,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum OverallQual {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OverallCond {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum YearBuilt {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum YearRemodAdd {
    Int(i64),
    Null,
}

create_enum!(RoofStyle;
"Flat" => Flat,
"Gable" => Gable,
"Gambrel" => Gambrel,
"Hip" => Hip,
"Mansard" => Mansard,
"Shed" => Shed,
Null,
);

create_enum!(RoofMatl;
"ClyTile" => ClyTile,
"CompShg" => CompShg,
"Membran" => Membran,
"Metal" => Metal,
"Roll" => Roll,
"Tar&Grv" => TarAndGrv,
"WdShake" => WdShake,
"WdShngl" => WdShngl,
Null,
);

create_enum!(ExteriorOnest;
"AsbShng" => AsbShng,
"AsphShn" => AsphShn,
"BrkComm" => BrkComm,
"BrkFace" => BrkFace,
"CBlock" => CBlock,
"CemntBd" => CemntBd,
"HdBoard" => HdBoard,
"ImStucc" => ImStucc,
"MetalSd" => MetalSd,
"Plywood" => Plywood,
"Stone" => Stone,
"Stucco" => Stucco,
"VinylSd" => VinylSd,
"Wd Sdng" => WdSpaceSdng,
"WdShing" => WdShing,
Null,
);

create_enum!(ExteriorTwond;
"AsbShng" => AsbShng,
"AsphShn" => AsphShn,
"Brk Cmn" => BrkSpaceCmn,
"BrkFace" => BrkFace,
"CBlock" => CBlock,
"CmentBd" => CmentBd,
"HdBoard" => HdBoard,
"ImStucc" => ImStucc,
"MetalSd" => MetalSd,
"Other" => Other,
"Plywood" => Plywood,
"Stone" => Stone,
"Stucco" => Stucco,
"VinylSd" => VinylSd,
"Wd Sdng" => WdSpaceSdng,
"Wd Shng" => WdSpaceShng,
Null,
);

create_enum!(MasVnrType;
"BrkCmn" => BrkCmn,
"BrkFace" => BrkFace,
"None" => None,
"Stone" => Stone,
Null,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum MasVnrArea {
    Int(i64),
    Null,
}

create_enum!(ExterQual;
"Ex" => Ex,
"Fa" => Fa,
"Gd" => Gd,
"TA" => TA,
Null,
);

create_enum!(ExterCond;
"Ex" => Ex,
"Fa" => Fa,
"Gd" => Gd,
"Po" => Po,
"TA" => TA,
Null,
);

create_enum!(Foundation;
"BrkTil" => BrkTil,
"CBlock" => CBlock,
"PConc" => PConc,
"Slab" => Slab,
"Stone" => Stone,
"Wood" => Wood,
Null,
);

create_enum!(BsmtQual;
"Ex" => Ex,
"Fa" => Fa,
"Gd" => Gd,
"TA" => TA,
Null,
);

create_enum!(BsmtCond;
"Fa" => Fa,
"Gd" => Gd,
"Po" => Po,
"TA" => TA,
Null,
);

create_enum!(BsmtExposure;
"Av" => Av,
"Gd" => Gd,
"Mn" => Mn,
"No" => No,
Null,
);

create_enum!(BsmtFinTypeOne;
"ALQ" => ALQ,
"BLQ" => BLQ,
"GLQ" => GLQ,
"LwQ" => LwQ,
"Rec" => Rec,
"Unf" => Unf,
Null,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum BsmtFinSFOne {
    Int(i64),
    Null,
}

create_enum!(BsmtFinTypeTwo;
"ALQ" => ALQ,
"BLQ" => BLQ,
"GLQ" => GLQ,
"LwQ" => LwQ,
"Rec" => Rec,
"Unf" => Unf,
Null,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum BsmtFinSFTwo {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BsmtUnfSF {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TotalBsmtSF {
    Int(i64),
    Null,
}

create_enum!(Heating;
"Floor" => Floor,
"GasA" => GasA,
"GasW" => GasW,
"Grav" => Grav,
"OthW" => OthW,
"Wall" => Wall,
Null,
);

create_enum!(HeatingQC;
"Floor" => Floor,
"GasA" => GasA,
"GasW" => GasW,
"Grav" => Grav,
"OthW" => OthW,
"Wall" => Wall,
Null,
);

create_enum!(CentralAir;
"N" => N,
"Y" => Y,
Null,
);

create_enum!(Electrical;
"FuseA" => FuseA,
"FuseF" => FuseF,
"FuseP" => FuseP,
"Mix" => Mix,
"SBrkr" => SBrkr,
Null,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum OnestFlrSF {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TwondFlrSF {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LowQualFinSF {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GrLivArea {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BsmtFullBath {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BsmtHalfBath {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FullBath {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum HalfBath {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BedroomAbvGr {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum KitchenAbvGr {
    Int(i64),
    Null,
}

create_enum!(KitchenQual;
"Ex" => Ex,
"Fa" => Fa,
"Gd" => Gd,
"TA" => TA,
Null,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum TotRmsAbvGrd {
    Int(i64),
    Null,
}

create_enum!(Functional;
"Maj1" => MajOne,
"Maj2" => MajTwo,
"Min1" => MinOne,
"Min2" => MinTwo,
"Mod" => Mod,
"Sev" => Sev,
"Typ" => Typ,
Null,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Fireplaces {
    Int(i64),
    Null,
}

create_enum!(FireplaceQu;
"Ex" => Ex,
"Fa" => Fa,
"Gd" => Gd,
"Po" => Po,
"TA" => TA,
Null,
);

create_enum!(GarageType;
"2Types" => TwoTypes,
"Attchd" => Attchd,
"Basment" => Basment,
"BuiltIn" => BuiltIn,
"CarPort" => CarPort,
"Detchd" => Detchd,
Null,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum GarageYrBlt {
    Int(i64),
    Null,
}

create_enum!(GarageFinish;
"Fin" => Fin,
"RFn" => RFn,
"Unf" => Unf,
Null,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum GarageCars {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GarageArea {
    Int(i64),
    Null,
}

create_enum!(GarageQual;
"Ex" => Ex,
"Fa" => Fa,
"Gd" => Gd,
"Po" => Po,
"TA" => TA,
Null,
);

create_enum!(GarageCond;
"Ex" => Ex,
"Fa" => Fa,
"Gd" => Gd,
"Po" => Po,
"TA" => TA,
Null,
);

create_enum!(PavedDrive;
"N" => N,
"P" => P,
"Y" => Y,
Null,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum WoodDeckSF {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OpenPorchSF {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EnclosedPorch {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ThreeSsnPorch {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ScreenPorch {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PoolArea {
    Int(i64),
    Null,
}

create_enum!(PoolQC;
"Ex" => Ex,
"Fa" => Fa,
"Gd" => Gd,
Null,
);

create_enum!(Fence;
"GdPrv" => GdPrv,
"GdWo" => GdWo,
"MnPrv" => MnPrv,
"MnWw" => MnWw,
Null,
);

create_enum!(MiscFeature;
"Gar2" => GarTwo,
"Othr" => Othr,
"Shed" => Shed,
"TenC" => TenC,
Null,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum MiscVal {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MoSold {
    Int(i64),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum YrSold {
    Int(i64),
    Null,
}

create_enum!(SaleType;
"COD" => COD,
"CWD" => CWD,
"Con" => Con,
"ConLD" => ConLD,
"ConLI" => ConLI,
"ConLw" => ConLw,
"New" => New,
"Oth" => Oth,
"WD" => WD,
Null,
);

create_enum!(SaleCondition;
"Abnorml" => Abnorml,
"AdjLand" => AdjLand,
"Alloca" => Alloca,
"Family" => Family,
"Normal" => Normal,
"Partial" => Partial,
Null,
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum SalePrice {
    Int(i64),
    Null,
}

enum CsvColumns {
    Id,
    MSSubClass,
    MSZoning,
    LotFrontage,
    LotArea,
    Street,
    Alley,
    LotShape,
    LandContour,
    Utilities,
    LotConfig,
    LandSlope,
    Neighborhood,
    ConditionOne,
    ConditionTwo,
    BldgType,
    HouseStyle,
    OverallQual,
    OverallCond,
    YearBuilt,
    YearRemodAdd,
    RoofStyle,
    RoofMatl,
    ExteriorOnest,
    ExteriorTwond,
    MasVnrType,
    MasVnrArea,
    ExterQual,
    ExterCond,
    Foundation,
    BsmtQual,
    BsmtCond,
    BsmtExposure,
    BsmtFinTypeOne,
    BsmtFinSFOne,
    BsmtFinTypeTwo,
    BsmtFinSFTwo,
    BsmtUnfSF,
    TotalBsmtSF,
    Heating,
    HeatingQC,
    CentralAir,
    Electrical,
    OnestFlrSF,
    TwondFlrSF,
    LowQualFinSF,
    GrLivArea,
    BsmtFullBath,
    BsmtHalfBath,
    FullBath,
    HalfBath,
    BedroomAbvGr,
    KitchenAbvGr,
    KitchenQual,
    TotRmsAbvGrd,
    Functional,
    Fireplaces,
    FireplaceQu,
    GarageType,
    GarageYrBlt,
    GarageFinish,
    GarageCars,
    GarageArea,
    GarageQual,
    GarageCond,
    PavedDrive,
    WoodDeckSF,
    OpenPorchSF,
    EnclosedPorch,
    ThreeSsnPorch,
    ScreenPorch,
    PoolArea,
    PoolQC,
    Fence,
    MiscFeature,
    MiscVal,
    MoSold,
    YrSold,
    SaleType,
    SaleCondition,
    SalePrice,
}

impl std::str::FromStr for CsvColumns {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Id" => Ok(CsvColumns::Id),
            "MSSubClass" => Ok(CsvColumns::MSSubClass),
            "MSZoning" => Ok(CsvColumns::MSZoning),
            "LotFrontage" => Ok(CsvColumns::LotFrontage),
            "LotArea" => Ok(CsvColumns::LotArea),
            "Street" => Ok(CsvColumns::Street),
            "Alley" => Ok(CsvColumns::Alley),
            "LotShape" => Ok(CsvColumns::LotShape),
            "LandContour" => Ok(CsvColumns::LandContour),
            "Utilities" => Ok(CsvColumns::Utilities),
            "LotConfig" => Ok(CsvColumns::LotConfig),
            "LandSlope" => Ok(CsvColumns::LandSlope),
            "Neighborhood" => Ok(CsvColumns::Neighborhood),
            "Condition1" => Ok(CsvColumns::ConditionOne),
            "Condition2" => Ok(CsvColumns::ConditionTwo),
            "BldgType" => Ok(CsvColumns::BldgType),
            "HouseStyle" => Ok(CsvColumns::HouseStyle),
            "OverallQual" => Ok(CsvColumns::OverallQual),
            "OverallCond" => Ok(CsvColumns::OverallCond),
            "YearBuilt" => Ok(CsvColumns::YearBuilt),
            "YearRemodAdd" => Ok(CsvColumns::YearRemodAdd),
            "RoofStyle" => Ok(CsvColumns::RoofStyle),
            "RoofMatl" => Ok(CsvColumns::RoofMatl),
            "Exterior1st" => Ok(CsvColumns::ExteriorOnest),
            "Exterior2nd" => Ok(CsvColumns::ExteriorTwond),
            "MasVnrType" => Ok(CsvColumns::MasVnrType),
            "MasVnrArea" => Ok(CsvColumns::MasVnrArea),
            "ExterQual" => Ok(CsvColumns::ExterQual),
            "ExterCond" => Ok(CsvColumns::ExterCond),
            "Foundation" => Ok(CsvColumns::Foundation),
            "BsmtQual" => Ok(CsvColumns::BsmtQual),
            "BsmtCond" => Ok(CsvColumns::BsmtCond),
            "BsmtExposure" => Ok(CsvColumns::BsmtExposure),
            "BsmtFinType1" => Ok(CsvColumns::BsmtFinTypeOne),
            "BsmtFinSF1" => Ok(CsvColumns::BsmtFinSFOne),
            "BsmtFinType2" => Ok(CsvColumns::BsmtFinTypeTwo),
            "BsmtFinSF2" => Ok(CsvColumns::BsmtFinSFTwo),
            "BsmtUnfSF" => Ok(CsvColumns::BsmtUnfSF),
            "TotalBsmtSF" => Ok(CsvColumns::TotalBsmtSF),
            "Heating" => Ok(CsvColumns::Heating),
            "HeatingQC" => Ok(CsvColumns::HeatingQC),
            "CentralAir" => Ok(CsvColumns::CentralAir),
            "Electrical" => Ok(CsvColumns::Electrical),
            "1stFlrSF" => Ok(CsvColumns::OnestFlrSF),
            "2ndFlrSF" => Ok(CsvColumns::TwondFlrSF),
            "LowQualFinSF" => Ok(CsvColumns::LowQualFinSF),
            "GrLivArea" => Ok(CsvColumns::GrLivArea),
            "BsmtFullBath" => Ok(CsvColumns::BsmtFullBath),
            "BsmtHalfBath" => Ok(CsvColumns::BsmtHalfBath),
            "FullBath" => Ok(CsvColumns::FullBath),
            "HalfBath" => Ok(CsvColumns::HalfBath),
            "BedroomAbvGr" => Ok(CsvColumns::BedroomAbvGr),
            "KitchenAbvGr" => Ok(CsvColumns::KitchenAbvGr),
            "KitchenQual" => Ok(CsvColumns::KitchenQual),
            "TotRmsAbvGrd" => Ok(CsvColumns::TotRmsAbvGrd),
            "Functional" => Ok(CsvColumns::Functional),
            "Fireplaces" => Ok(CsvColumns::Fireplaces),
            "FireplaceQu" => Ok(CsvColumns::FireplaceQu),
            "GarageType" => Ok(CsvColumns::GarageType),
            "GarageYrBlt" => Ok(CsvColumns::GarageYrBlt),
            "GarageFinish" => Ok(CsvColumns::GarageFinish),
            "GarageCars" => Ok(CsvColumns::GarageCars),
            "GarageArea" => Ok(CsvColumns::GarageArea),
            "GarageQual" => Ok(CsvColumns::GarageQual),
            "GarageCond" => Ok(CsvColumns::GarageCond),
            "PavedDrive" => Ok(CsvColumns::PavedDrive),
            "WoodDeckSF" => Ok(CsvColumns::WoodDeckSF),
            "OpenPorchSF" => Ok(CsvColumns::OpenPorchSF),
            "EnclosedPorch" => Ok(CsvColumns::EnclosedPorch),
            "3SsnPorch" => Ok(CsvColumns::ThreeSsnPorch),
            "ScreenPorch" => Ok(CsvColumns::ScreenPorch),
            "PoolArea" => Ok(CsvColumns::PoolArea),
            "PoolQC" => Ok(CsvColumns::PoolQC),
            "Fence" => Ok(CsvColumns::Fence),
            "MiscFeature" => Ok(CsvColumns::MiscFeature),
            "MiscVal" => Ok(CsvColumns::MiscVal),
            "MoSold" => Ok(CsvColumns::MoSold),
            "YrSold" => Ok(CsvColumns::YrSold),
            "SaleType" => Ok(CsvColumns::SaleType),
            "SaleCondition" => Ok(CsvColumns::SaleCondition),
            "SalePrice" => Ok(CsvColumns::SalePrice),
            _ => Err(format!("Unknown string: '{}'", s)),
        }
    }
}
