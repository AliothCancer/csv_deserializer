#![allow(unused, non_snake_case)]
use std::str::FromStr;

use csv_deserializer::create_enum;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Id {
    Int(i64),
    Null,
}

impl std::str::FromStr for Id {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(Id::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MSSubClass {
    Int(i64),
    Null,
}

impl std::str::FromStr for MSSubClass {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(MSSubClass::Int(i))
    }
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

impl std::str::FromStr for LotFrontage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(LotFrontage::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LotArea {
    Int(i64),
    Null,
}

impl std::str::FromStr for LotArea {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(LotArea::Int(i))
    }
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

impl std::str::FromStr for OverallQual {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(OverallQual::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OverallCond {
    Int(i64),
    Null,
}

impl std::str::FromStr for OverallCond {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(OverallCond::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum YearBuilt {
    Int(i64),
    Null,
}

impl std::str::FromStr for YearBuilt {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(YearBuilt::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum YearRemodAdd {
    Int(i64),
    Null,
}

impl std::str::FromStr for YearRemodAdd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(YearRemodAdd::Int(i))
    }
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

impl std::str::FromStr for MasVnrArea {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(MasVnrArea::Int(i))
    }
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

impl std::str::FromStr for BsmtFinSFOne {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(BsmtFinSFOne::Int(i))
    }
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

impl std::str::FromStr for BsmtFinSFTwo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(BsmtFinSFTwo::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BsmtUnfSF {
    Int(i64),
    Null,
}

impl std::str::FromStr for BsmtUnfSF {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(BsmtUnfSF::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TotalBsmtSF {
    Int(i64),
    Null,
}

impl std::str::FromStr for TotalBsmtSF {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(TotalBsmtSF::Int(i))
    }
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

impl std::str::FromStr for OnestFlrSF {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(OnestFlrSF::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TwondFlrSF {
    Int(i64),
    Null,
}

impl std::str::FromStr for TwondFlrSF {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(TwondFlrSF::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LowQualFinSF {
    Int(i64),
    Null,
}

impl std::str::FromStr for LowQualFinSF {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(LowQualFinSF::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GrLivArea {
    Int(i64),
    Null,
}

impl std::str::FromStr for GrLivArea {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(GrLivArea::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BsmtFullBath {
    Int(i64),
    Null,
}

impl std::str::FromStr for BsmtFullBath {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(BsmtFullBath::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BsmtHalfBath {
    Int(i64),
    Null,
}

impl std::str::FromStr for BsmtHalfBath {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(BsmtHalfBath::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FullBath {
    Int(i64),
    Null,
}

impl std::str::FromStr for FullBath {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(FullBath::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum HalfBath {
    Int(i64),
    Null,
}

impl std::str::FromStr for HalfBath {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(HalfBath::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BedroomAbvGr {
    Int(i64),
    Null,
}

impl std::str::FromStr for BedroomAbvGr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(BedroomAbvGr::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum KitchenAbvGr {
    Int(i64),
    Null,
}

impl std::str::FromStr for KitchenAbvGr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(KitchenAbvGr::Int(i))
    }
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

impl std::str::FromStr for TotRmsAbvGrd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(TotRmsAbvGrd::Int(i))
    }
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

impl std::str::FromStr for Fireplaces {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(Fireplaces::Int(i))
    }
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

impl std::str::FromStr for GarageYrBlt {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(GarageYrBlt::Int(i))
    }
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

impl std::str::FromStr for GarageCars {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(GarageCars::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GarageArea {
    Int(i64),
    Null,
}

impl std::str::FromStr for GarageArea {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(GarageArea::Int(i))
    }
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

impl std::str::FromStr for WoodDeckSF {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(WoodDeckSF::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OpenPorchSF {
    Int(i64),
    Null,
}

impl std::str::FromStr for OpenPorchSF {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(OpenPorchSF::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EnclosedPorch {
    Int(i64),
    Null,
}

impl std::str::FromStr for EnclosedPorch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(EnclosedPorch::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ThreeSsnPorch {
    Int(i64),
    Null,
}

impl std::str::FromStr for ThreeSsnPorch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(ThreeSsnPorch::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ScreenPorch {
    Int(i64),
    Null,
}

impl std::str::FromStr for ScreenPorch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(ScreenPorch::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PoolArea {
    Int(i64),
    Null,
}

impl std::str::FromStr for PoolArea {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(PoolArea::Int(i))
    }
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

impl std::str::FromStr for MiscVal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(MiscVal::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MoSold {
    Int(i64),
    Null,
}

impl std::str::FromStr for MoSold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(MoSold::Int(i))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum YrSold {
    Int(i64),
    Null,
}

impl std::str::FromStr for YrSold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(YrSold::Int(i))
    }
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

impl std::str::FromStr for SalePrice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.parse::<i64>().unwrap();
        Ok(SalePrice::Int(i))
    }
}

enum CsvColumn {
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

impl std::str::FromStr for CsvColumn {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Id" => Ok(CsvColumn::Id),
            "MSSubClass" => Ok(CsvColumn::MSSubClass),
            "MSZoning" => Ok(CsvColumn::MSZoning),
            "LotFrontage" => Ok(CsvColumn::LotFrontage),
            "LotArea" => Ok(CsvColumn::LotArea),
            "Street" => Ok(CsvColumn::Street),
            "Alley" => Ok(CsvColumn::Alley),
            "LotShape" => Ok(CsvColumn::LotShape),
            "LandContour" => Ok(CsvColumn::LandContour),
            "Utilities" => Ok(CsvColumn::Utilities),
            "LotConfig" => Ok(CsvColumn::LotConfig),
            "LandSlope" => Ok(CsvColumn::LandSlope),
            "Neighborhood" => Ok(CsvColumn::Neighborhood),
            "Condition1" => Ok(CsvColumn::ConditionOne),
            "Condition2" => Ok(CsvColumn::ConditionTwo),
            "BldgType" => Ok(CsvColumn::BldgType),
            "HouseStyle" => Ok(CsvColumn::HouseStyle),
            "OverallQual" => Ok(CsvColumn::OverallQual),
            "OverallCond" => Ok(CsvColumn::OverallCond),
            "YearBuilt" => Ok(CsvColumn::YearBuilt),
            "YearRemodAdd" => Ok(CsvColumn::YearRemodAdd),
            "RoofStyle" => Ok(CsvColumn::RoofStyle),
            "RoofMatl" => Ok(CsvColumn::RoofMatl),
            "Exterior1st" => Ok(CsvColumn::ExteriorOnest),
            "Exterior2nd" => Ok(CsvColumn::ExteriorTwond),
            "MasVnrType" => Ok(CsvColumn::MasVnrType),
            "MasVnrArea" => Ok(CsvColumn::MasVnrArea),
            "ExterQual" => Ok(CsvColumn::ExterQual),
            "ExterCond" => Ok(CsvColumn::ExterCond),
            "Foundation" => Ok(CsvColumn::Foundation),
            "BsmtQual" => Ok(CsvColumn::BsmtQual),
            "BsmtCond" => Ok(CsvColumn::BsmtCond),
            "BsmtExposure" => Ok(CsvColumn::BsmtExposure),
            "BsmtFinType1" => Ok(CsvColumn::BsmtFinTypeOne),
            "BsmtFinSF1" => Ok(CsvColumn::BsmtFinSFOne),
            "BsmtFinType2" => Ok(CsvColumn::BsmtFinTypeTwo),
            "BsmtFinSF2" => Ok(CsvColumn::BsmtFinSFTwo),
            "BsmtUnfSF" => Ok(CsvColumn::BsmtUnfSF),
            "TotalBsmtSF" => Ok(CsvColumn::TotalBsmtSF),
            "Heating" => Ok(CsvColumn::Heating),
            "HeatingQC" => Ok(CsvColumn::HeatingQC),
            "CentralAir" => Ok(CsvColumn::CentralAir),
            "Electrical" => Ok(CsvColumn::Electrical),
            "1stFlrSF" => Ok(CsvColumn::OnestFlrSF),
            "2ndFlrSF" => Ok(CsvColumn::TwondFlrSF),
            "LowQualFinSF" => Ok(CsvColumn::LowQualFinSF),
            "GrLivArea" => Ok(CsvColumn::GrLivArea),
            "BsmtFullBath" => Ok(CsvColumn::BsmtFullBath),
            "BsmtHalfBath" => Ok(CsvColumn::BsmtHalfBath),
            "FullBath" => Ok(CsvColumn::FullBath),
            "HalfBath" => Ok(CsvColumn::HalfBath),
            "BedroomAbvGr" => Ok(CsvColumn::BedroomAbvGr),
            "KitchenAbvGr" => Ok(CsvColumn::KitchenAbvGr),
            "KitchenQual" => Ok(CsvColumn::KitchenQual),
            "TotRmsAbvGrd" => Ok(CsvColumn::TotRmsAbvGrd),
            "Functional" => Ok(CsvColumn::Functional),
            "Fireplaces" => Ok(CsvColumn::Fireplaces),
            "FireplaceQu" => Ok(CsvColumn::FireplaceQu),
            "GarageType" => Ok(CsvColumn::GarageType),
            "GarageYrBlt" => Ok(CsvColumn::GarageYrBlt),
            "GarageFinish" => Ok(CsvColumn::GarageFinish),
            "GarageCars" => Ok(CsvColumn::GarageCars),
            "GarageArea" => Ok(CsvColumn::GarageArea),
            "GarageQual" => Ok(CsvColumn::GarageQual),
            "GarageCond" => Ok(CsvColumn::GarageCond),
            "PavedDrive" => Ok(CsvColumn::PavedDrive),
            "WoodDeckSF" => Ok(CsvColumn::WoodDeckSF),
            "OpenPorchSF" => Ok(CsvColumn::OpenPorchSF),
            "EnclosedPorch" => Ok(CsvColumn::EnclosedPorch),
            "3SsnPorch" => Ok(CsvColumn::ThreeSsnPorch),
            "ScreenPorch" => Ok(CsvColumn::ScreenPorch),
            "PoolArea" => Ok(CsvColumn::PoolArea),
            "PoolQC" => Ok(CsvColumn::PoolQC),
            "Fence" => Ok(CsvColumn::Fence),
            "MiscFeature" => Ok(CsvColumn::MiscFeature),
            "MiscVal" => Ok(CsvColumn::MiscVal),
            "MoSold" => Ok(CsvColumn::MoSold),
            "YrSold" => Ok(CsvColumn::YrSold),
            "SaleType" => Ok(CsvColumn::SaleType),
            "SaleCondition" => Ok(CsvColumn::SaleCondition),
            "SalePrice" => Ok(CsvColumn::SalePrice),
            _ => Err(format!("Unknown string: '{}'", s)),
        }
    }
}
pub struct CsvDataFrame {
    columns: Vec<CsvColumn>,
    Id: Vec<Id>,
    MSSubClass: Vec<MSSubClass>,
    MSZoning: Vec<MSZoning>,
    LotFrontage: Vec<LotFrontage>,
    LotArea: Vec<LotArea>,
    Street: Vec<Street>,
    Alley: Vec<Alley>,
    LotShape: Vec<LotShape>,
    LandContour: Vec<LandContour>,
    Utilities: Vec<Utilities>,
    LotConfig: Vec<LotConfig>,
    LandSlope: Vec<LandSlope>,
    Neighborhood: Vec<Neighborhood>,
    ConditionOne: Vec<ConditionOne>,
    ConditionTwo: Vec<ConditionTwo>,
    BldgType: Vec<BldgType>,
    HouseStyle: Vec<HouseStyle>,
    OverallQual: Vec<OverallQual>,
    OverallCond: Vec<OverallCond>,
    YearBuilt: Vec<YearBuilt>,
    YearRemodAdd: Vec<YearRemodAdd>,
    RoofStyle: Vec<RoofStyle>,
    RoofMatl: Vec<RoofMatl>,
    ExteriorOnest: Vec<ExteriorOnest>,
    ExteriorTwond: Vec<ExteriorTwond>,
    MasVnrType: Vec<MasVnrType>,
    MasVnrArea: Vec<MasVnrArea>,
    ExterQual: Vec<ExterQual>,
    ExterCond: Vec<ExterCond>,
    Foundation: Vec<Foundation>,
    BsmtQual: Vec<BsmtQual>,
    BsmtCond: Vec<BsmtCond>,
    BsmtExposure: Vec<BsmtExposure>,
    BsmtFinTypeOne: Vec<BsmtFinTypeOne>,
    BsmtFinSFOne: Vec<BsmtFinSFOne>,
    BsmtFinTypeTwo: Vec<BsmtFinTypeTwo>,
    BsmtFinSFTwo: Vec<BsmtFinSFTwo>,
    BsmtUnfSF: Vec<BsmtUnfSF>,
    TotalBsmtSF: Vec<TotalBsmtSF>,
    Heating: Vec<Heating>,
    HeatingQC: Vec<HeatingQC>,
    CentralAir: Vec<CentralAir>,
    Electrical: Vec<Electrical>,
    OnestFlrSF: Vec<OnestFlrSF>,
    TwondFlrSF: Vec<TwondFlrSF>,
    LowQualFinSF: Vec<LowQualFinSF>,
    GrLivArea: Vec<GrLivArea>,
    BsmtFullBath: Vec<BsmtFullBath>,
    BsmtHalfBath: Vec<BsmtHalfBath>,
    FullBath: Vec<FullBath>,
    HalfBath: Vec<HalfBath>,
    BedroomAbvGr: Vec<BedroomAbvGr>,
    KitchenAbvGr: Vec<KitchenAbvGr>,
    KitchenQual: Vec<KitchenQual>,
    TotRmsAbvGrd: Vec<TotRmsAbvGrd>,
    Functional: Vec<Functional>,
    Fireplaces: Vec<Fireplaces>,
    FireplaceQu: Vec<FireplaceQu>,
    GarageType: Vec<GarageType>,
    GarageYrBlt: Vec<GarageYrBlt>,
    GarageFinish: Vec<GarageFinish>,
    GarageCars: Vec<GarageCars>,
    GarageArea: Vec<GarageArea>,
    GarageQual: Vec<GarageQual>,
    GarageCond: Vec<GarageCond>,
    PavedDrive: Vec<PavedDrive>,
    WoodDeckSF: Vec<WoodDeckSF>,
    OpenPorchSF: Vec<OpenPorchSF>,
    EnclosedPorch: Vec<EnclosedPorch>,
    ThreeSsnPorch: Vec<ThreeSsnPorch>,
    ScreenPorch: Vec<ScreenPorch>,
    PoolArea: Vec<PoolArea>,
    PoolQC: Vec<PoolQC>,
    Fence: Vec<Fence>,
    MiscFeature: Vec<MiscFeature>,
    MiscVal: Vec<MiscVal>,
    MoSold: Vec<MoSold>,
    YrSold: Vec<YrSold>,
    SaleType: Vec<SaleType>,
    SaleCondition: Vec<SaleCondition>,
    SalePrice: Vec<SalePrice>,
}

impl CsvDataFrame {
    fn new(dataset: &mut csv_deserializer::CsvDataset) -> Self {
        let Id = dataset
            .values
            .pop()
            .unwrap()
            .into_iter()
            .map(|id| match id {
                csv_deserializer::CsvAny::Int(i) => Id::Int(i),
                csv_deserializer::CsvAny::Null => Id::Null,
                _ => panic!(),
            })
            .collect::<Vec<Id>>();

        let columns: Vec<CsvColumn> = dataset
            .names
            .iter()
            .enumerate()
            .filter_map(|(col_index, name)| CsvColumn::from_str(&name.raw).ok())
            .collect();

        todo!()
    }
}
