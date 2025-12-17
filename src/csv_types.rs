#![allow(unused, non_snake_case)]
use csv_deserializer::create_enum;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Id {
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
pub enum MSSubClass {
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
pub enum LotFrontage {
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
pub enum LotArea {
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
pub enum OverallQual {
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
pub enum OverallCond {
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
pub enum YearBuilt {
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
pub enum YearRemodAdd {
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
pub enum MasVnrArea {
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
pub enum BsmtFinSFOne {
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
pub enum BsmtFinSFTwo {
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
pub enum BsmtUnfSF {
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
pub enum TotalBsmtSF {
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
"Ex" => Ex,
"Fa" => Fa,
"Gd" => Gd,
"Po" => Po,
"TA" => TA,
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
pub enum OnestFlrSF {
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
pub enum TwondFlrSF {
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
pub enum LowQualFinSF {
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
pub enum GrLivArea {
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
pub enum BsmtFullBath {
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
pub enum BsmtHalfBath {
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
pub enum FullBath {
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
pub enum HalfBath {
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
pub enum BedroomAbvGr {
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
pub enum KitchenAbvGr {
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
pub enum TotRmsAbvGrd {
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
pub enum Fireplaces {
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
pub enum GarageYrBlt {
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
pub enum GarageCars {
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
pub enum GarageArea {
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
pub enum WoodDeckSF {
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
pub enum OpenPorchSF {
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
pub enum EnclosedPorch {
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
pub enum ThreeSsnPorch {
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
pub enum ScreenPorch {
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
pub enum PoolArea {
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
pub enum MiscVal {
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
pub enum MoSold {
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
pub enum YrSold {
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
pub enum SalePrice {
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

pub enum CsvColumn {
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
    pub columns: Vec<CsvColumn>,
    pub id: Vec<Id>,
    pub mssubclass: Vec<MSSubClass>,
    pub mszoning: Vec<MSZoning>,
    pub lotfrontage: Vec<LotFrontage>,
    pub lotarea: Vec<LotArea>,
    pub street: Vec<Street>,
    pub alley: Vec<Alley>,
    pub lotshape: Vec<LotShape>,
    pub landcontour: Vec<LandContour>,
    pub utilities: Vec<Utilities>,
    pub lotconfig: Vec<LotConfig>,
    pub landslope: Vec<LandSlope>,
    pub neighborhood: Vec<Neighborhood>,
    pub conditionone: Vec<ConditionOne>,
    pub conditiontwo: Vec<ConditionTwo>,
    pub bldgtype: Vec<BldgType>,
    pub housestyle: Vec<HouseStyle>,
    pub overallqual: Vec<OverallQual>,
    pub overallcond: Vec<OverallCond>,
    pub yearbuilt: Vec<YearBuilt>,
    pub yearremodadd: Vec<YearRemodAdd>,
    pub roofstyle: Vec<RoofStyle>,
    pub roofmatl: Vec<RoofMatl>,
    pub exterioronest: Vec<ExteriorOnest>,
    pub exteriortwond: Vec<ExteriorTwond>,
    pub masvnrtype: Vec<MasVnrType>,
    pub masvnrarea: Vec<MasVnrArea>,
    pub exterqual: Vec<ExterQual>,
    pub extercond: Vec<ExterCond>,
    pub foundation: Vec<Foundation>,
    pub bsmtqual: Vec<BsmtQual>,
    pub bsmtcond: Vec<BsmtCond>,
    pub bsmtexposure: Vec<BsmtExposure>,
    pub bsmtfintypeone: Vec<BsmtFinTypeOne>,
    pub bsmtfinsfone: Vec<BsmtFinSFOne>,
    pub bsmtfintypetwo: Vec<BsmtFinTypeTwo>,
    pub bsmtfinsftwo: Vec<BsmtFinSFTwo>,
    pub bsmtunfsf: Vec<BsmtUnfSF>,
    pub totalbsmtsf: Vec<TotalBsmtSF>,
    pub heating: Vec<Heating>,
    pub heatingqc: Vec<HeatingQC>,
    pub centralair: Vec<CentralAir>,
    pub electrical: Vec<Electrical>,
    pub onestflrsf: Vec<OnestFlrSF>,
    pub twondflrsf: Vec<TwondFlrSF>,
    pub lowqualfinsf: Vec<LowQualFinSF>,
    pub grlivarea: Vec<GrLivArea>,
    pub bsmtfullbath: Vec<BsmtFullBath>,
    pub bsmthalfbath: Vec<BsmtHalfBath>,
    pub fullbath: Vec<FullBath>,
    pub halfbath: Vec<HalfBath>,
    pub bedroomabvgr: Vec<BedroomAbvGr>,
    pub kitchenabvgr: Vec<KitchenAbvGr>,
    pub kitchenqual: Vec<KitchenQual>,
    pub totrmsabvgrd: Vec<TotRmsAbvGrd>,
    pub functional: Vec<Functional>,
    pub fireplaces: Vec<Fireplaces>,
    pub fireplacequ: Vec<FireplaceQu>,
    pub garagetype: Vec<GarageType>,
    pub garageyrblt: Vec<GarageYrBlt>,
    pub garagefinish: Vec<GarageFinish>,
    pub garagecars: Vec<GarageCars>,
    pub garagearea: Vec<GarageArea>,
    pub garagequal: Vec<GarageQual>,
    pub garagecond: Vec<GarageCond>,
    pub paveddrive: Vec<PavedDrive>,
    pub wooddecksf: Vec<WoodDeckSF>,
    pub openporchsf: Vec<OpenPorchSF>,
    pub enclosedporch: Vec<EnclosedPorch>,
    pub threessnporch: Vec<ThreeSsnPorch>,
    pub screenporch: Vec<ScreenPorch>,
    pub poolarea: Vec<PoolArea>,
    pub poolqc: Vec<PoolQC>,
    pub fence: Vec<Fence>,
    pub miscfeature: Vec<MiscFeature>,
    pub miscval: Vec<MiscVal>,
    pub mosold: Vec<MoSold>,
    pub yrsold: Vec<YrSold>,
    pub saletype: Vec<SaleType>,
    pub salecondition: Vec<SaleCondition>,
    pub saleprice: Vec<SalePrice>,
}
impl CsvDataFrame {
    pub fn new(dataset: csv_deserializer::CsvDataset) -> Self {
        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "Id")
            .unwrap();
        let id = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => Id::Int(*i),
                csv_deserializer::CsvAny::Null => Id::Null,

                _ => panic!(),
            })
            .collect::<Vec<Id>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "MSSubClass")
            .unwrap();
        let mssubclass = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => MSSubClass::Int(*i),
                csv_deserializer::CsvAny::Null => MSSubClass::Null,

                _ => panic!(),
            })
            .collect::<Vec<MSSubClass>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "MSZoning")
            .unwrap();
        let mszoning = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => MSZoning::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => MSZoning::Null,

                _ => panic!(),
            })
            .collect::<Vec<MSZoning>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "LotFrontage")
            .unwrap();
        let lotfrontage = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => LotFrontage::Int(*i),
                csv_deserializer::CsvAny::Null => LotFrontage::Null,

                _ => panic!(),
            })
            .collect::<Vec<LotFrontage>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "LotArea")
            .unwrap();
        let lotarea = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => LotArea::Int(*i),
                csv_deserializer::CsvAny::Null => LotArea::Null,

                _ => panic!(),
            })
            .collect::<Vec<LotArea>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "Street")
            .unwrap();
        let street = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => Street::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => Street::Null,

                _ => panic!(),
            })
            .collect::<Vec<Street>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "Alley")
            .unwrap();
        let alley = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => Alley::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => Alley::Null,

                _ => panic!(),
            })
            .collect::<Vec<Alley>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "LotShape")
            .unwrap();
        let lotshape = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => LotShape::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => LotShape::Null,

                _ => panic!(),
            })
            .collect::<Vec<LotShape>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "LandContour")
            .unwrap();
        let landcontour = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => LandContour::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => LandContour::Null,

                _ => panic!(),
            })
            .collect::<Vec<LandContour>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "Utilities")
            .unwrap();
        let utilities = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => Utilities::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => Utilities::Null,

                _ => panic!(),
            })
            .collect::<Vec<Utilities>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "LotConfig")
            .unwrap();
        let lotconfig = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => LotConfig::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => LotConfig::Null,

                _ => panic!(),
            })
            .collect::<Vec<LotConfig>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "LandSlope")
            .unwrap();
        let landslope = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => LandSlope::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => LandSlope::Null,

                _ => panic!(),
            })
            .collect::<Vec<LandSlope>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "Neighborhood")
            .unwrap();
        let neighborhood = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => Neighborhood::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => Neighborhood::Null,

                _ => panic!(),
            })
            .collect::<Vec<Neighborhood>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "ConditionOne")
            .unwrap();
        let conditionone = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => ConditionOne::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => ConditionOne::Null,

                _ => panic!(),
            })
            .collect::<Vec<ConditionOne>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "ConditionTwo")
            .unwrap();
        let conditiontwo = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => ConditionTwo::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => ConditionTwo::Null,

                _ => panic!(),
            })
            .collect::<Vec<ConditionTwo>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "BldgType")
            .unwrap();
        let bldgtype = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => BldgType::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => BldgType::Null,

                _ => panic!(),
            })
            .collect::<Vec<BldgType>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "HouseStyle")
            .unwrap();
        let housestyle = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => HouseStyle::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => HouseStyle::Null,

                _ => panic!(),
            })
            .collect::<Vec<HouseStyle>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "OverallQual")
            .unwrap();
        let overallqual = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => OverallQual::Int(*i),
                csv_deserializer::CsvAny::Null => OverallQual::Null,

                _ => panic!(),
            })
            .collect::<Vec<OverallQual>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "OverallCond")
            .unwrap();
        let overallcond = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => OverallCond::Int(*i),
                csv_deserializer::CsvAny::Null => OverallCond::Null,

                _ => panic!(),
            })
            .collect::<Vec<OverallCond>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "YearBuilt")
            .unwrap();
        let yearbuilt = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => YearBuilt::Int(*i),
                csv_deserializer::CsvAny::Null => YearBuilt::Null,

                _ => panic!(),
            })
            .collect::<Vec<YearBuilt>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "YearRemodAdd")
            .unwrap();
        let yearremodadd = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => YearRemodAdd::Int(*i),
                csv_deserializer::CsvAny::Null => YearRemodAdd::Null,

                _ => panic!(),
            })
            .collect::<Vec<YearRemodAdd>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "RoofStyle")
            .unwrap();
        let roofstyle = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => RoofStyle::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => RoofStyle::Null,

                _ => panic!(),
            })
            .collect::<Vec<RoofStyle>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "RoofMatl")
            .unwrap();
        let roofmatl = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => RoofMatl::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => RoofMatl::Null,

                _ => panic!(),
            })
            .collect::<Vec<RoofMatl>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "ExteriorOnest")
            .unwrap();
        let exterioronest = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => ExteriorOnest::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => ExteriorOnest::Null,

                _ => panic!(),
            })
            .collect::<Vec<ExteriorOnest>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "ExteriorTwond")
            .unwrap();
        let exteriortwond = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => ExteriorTwond::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => ExteriorTwond::Null,

                _ => panic!(),
            })
            .collect::<Vec<ExteriorTwond>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "MasVnrType")
            .unwrap();
        let masvnrtype = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => MasVnrType::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => MasVnrType::Null,

                _ => panic!(),
            })
            .collect::<Vec<MasVnrType>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "MasVnrArea")
            .unwrap();
        let masvnrarea = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => MasVnrArea::Int(*i),
                csv_deserializer::CsvAny::Null => MasVnrArea::Null,

                _ => panic!(),
            })
            .collect::<Vec<MasVnrArea>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "ExterQual")
            .unwrap();
        let exterqual = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => ExterQual::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => ExterQual::Null,

                _ => panic!(),
            })
            .collect::<Vec<ExterQual>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "ExterCond")
            .unwrap();
        let extercond = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => ExterCond::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => ExterCond::Null,

                _ => panic!(),
            })
            .collect::<Vec<ExterCond>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "Foundation")
            .unwrap();
        let foundation = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => Foundation::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => Foundation::Null,

                _ => panic!(),
            })
            .collect::<Vec<Foundation>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "BsmtQual")
            .unwrap();
        let bsmtqual = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => BsmtQual::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => BsmtQual::Null,

                _ => panic!(),
            })
            .collect::<Vec<BsmtQual>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "BsmtCond")
            .unwrap();
        let bsmtcond = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => BsmtCond::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => BsmtCond::Null,

                _ => panic!(),
            })
            .collect::<Vec<BsmtCond>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "BsmtExposure")
            .unwrap();
        let bsmtexposure = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => BsmtExposure::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => BsmtExposure::Null,

                _ => panic!(),
            })
            .collect::<Vec<BsmtExposure>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "BsmtFinTypeOne")
            .unwrap();
        let bsmtfintypeone = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => BsmtFinTypeOne::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => BsmtFinTypeOne::Null,

                _ => panic!(),
            })
            .collect::<Vec<BsmtFinTypeOne>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "BsmtFinSFOne")
            .unwrap();
        let bsmtfinsfone = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => BsmtFinSFOne::Int(*i),
                csv_deserializer::CsvAny::Null => BsmtFinSFOne::Null,

                _ => panic!(),
            })
            .collect::<Vec<BsmtFinSFOne>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "BsmtFinTypeTwo")
            .unwrap();
        let bsmtfintypetwo = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => BsmtFinTypeTwo::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => BsmtFinTypeTwo::Null,

                _ => panic!(),
            })
            .collect::<Vec<BsmtFinTypeTwo>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "BsmtFinSFTwo")
            .unwrap();
        let bsmtfinsftwo = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => BsmtFinSFTwo::Int(*i),
                csv_deserializer::CsvAny::Null => BsmtFinSFTwo::Null,

                _ => panic!(),
            })
            .collect::<Vec<BsmtFinSFTwo>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "BsmtUnfSF")
            .unwrap();
        let bsmtunfsf = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => BsmtUnfSF::Int(*i),
                csv_deserializer::CsvAny::Null => BsmtUnfSF::Null,

                _ => panic!(),
            })
            .collect::<Vec<BsmtUnfSF>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "TotalBsmtSF")
            .unwrap();
        let totalbsmtsf = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => TotalBsmtSF::Int(*i),
                csv_deserializer::CsvAny::Null => TotalBsmtSF::Null,

                _ => panic!(),
            })
            .collect::<Vec<TotalBsmtSF>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "Heating")
            .unwrap();
        let heating = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => Heating::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => Heating::Null,

                _ => panic!(),
            })
            .collect::<Vec<Heating>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "HeatingQC")
            .unwrap();
        let heatingqc = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => HeatingQC::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => HeatingQC::Null,

                _ => panic!(),
            })
            .collect::<Vec<HeatingQC>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "CentralAir")
            .unwrap();
        let centralair = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => CentralAir::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => CentralAir::Null,

                _ => panic!(),
            })
            .collect::<Vec<CentralAir>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "Electrical")
            .unwrap();
        let electrical = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => Electrical::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => Electrical::Null,

                _ => panic!(),
            })
            .collect::<Vec<Electrical>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "OnestFlrSF")
            .unwrap();
        let onestflrsf = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => OnestFlrSF::Int(*i),
                csv_deserializer::CsvAny::Null => OnestFlrSF::Null,

                _ => panic!(),
            })
            .collect::<Vec<OnestFlrSF>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "TwondFlrSF")
            .unwrap();
        let twondflrsf = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => TwondFlrSF::Int(*i),
                csv_deserializer::CsvAny::Null => TwondFlrSF::Null,

                _ => panic!(),
            })
            .collect::<Vec<TwondFlrSF>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "LowQualFinSF")
            .unwrap();
        let lowqualfinsf = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => LowQualFinSF::Int(*i),
                csv_deserializer::CsvAny::Null => LowQualFinSF::Null,

                _ => panic!(),
            })
            .collect::<Vec<LowQualFinSF>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "GrLivArea")
            .unwrap();
        let grlivarea = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => GrLivArea::Int(*i),
                csv_deserializer::CsvAny::Null => GrLivArea::Null,

                _ => panic!(),
            })
            .collect::<Vec<GrLivArea>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "BsmtFullBath")
            .unwrap();
        let bsmtfullbath = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => BsmtFullBath::Int(*i),
                csv_deserializer::CsvAny::Null => BsmtFullBath::Null,

                _ => panic!(),
            })
            .collect::<Vec<BsmtFullBath>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "BsmtHalfBath")
            .unwrap();
        let bsmthalfbath = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => BsmtHalfBath::Int(*i),
                csv_deserializer::CsvAny::Null => BsmtHalfBath::Null,

                _ => panic!(),
            })
            .collect::<Vec<BsmtHalfBath>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "FullBath")
            .unwrap();
        let fullbath = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => FullBath::Int(*i),
                csv_deserializer::CsvAny::Null => FullBath::Null,

                _ => panic!(),
            })
            .collect::<Vec<FullBath>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "HalfBath")
            .unwrap();
        let halfbath = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => HalfBath::Int(*i),
                csv_deserializer::CsvAny::Null => HalfBath::Null,

                _ => panic!(),
            })
            .collect::<Vec<HalfBath>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "BedroomAbvGr")
            .unwrap();
        let bedroomabvgr = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => BedroomAbvGr::Int(*i),
                csv_deserializer::CsvAny::Null => BedroomAbvGr::Null,

                _ => panic!(),
            })
            .collect::<Vec<BedroomAbvGr>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "KitchenAbvGr")
            .unwrap();
        let kitchenabvgr = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => KitchenAbvGr::Int(*i),
                csv_deserializer::CsvAny::Null => KitchenAbvGr::Null,

                _ => panic!(),
            })
            .collect::<Vec<KitchenAbvGr>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "KitchenQual")
            .unwrap();
        let kitchenqual = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => KitchenQual::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => KitchenQual::Null,

                _ => panic!(),
            })
            .collect::<Vec<KitchenQual>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "TotRmsAbvGrd")
            .unwrap();
        let totrmsabvgrd = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => TotRmsAbvGrd::Int(*i),
                csv_deserializer::CsvAny::Null => TotRmsAbvGrd::Null,

                _ => panic!(),
            })
            .collect::<Vec<TotRmsAbvGrd>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "Functional")
            .unwrap();
        let functional = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => Functional::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => Functional::Null,

                _ => panic!(),
            })
            .collect::<Vec<Functional>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "Fireplaces")
            .unwrap();
        let fireplaces = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => Fireplaces::Int(*i),
                csv_deserializer::CsvAny::Null => Fireplaces::Null,

                _ => panic!(),
            })
            .collect::<Vec<Fireplaces>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "FireplaceQu")
            .unwrap();
        let fireplacequ = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => FireplaceQu::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => FireplaceQu::Null,

                _ => panic!(),
            })
            .collect::<Vec<FireplaceQu>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "GarageType")
            .unwrap();
        let garagetype = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => GarageType::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => GarageType::Null,

                _ => panic!(),
            })
            .collect::<Vec<GarageType>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "GarageYrBlt")
            .unwrap();
        let garageyrblt = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => GarageYrBlt::Int(*i),
                csv_deserializer::CsvAny::Null => GarageYrBlt::Null,

                _ => panic!(),
            })
            .collect::<Vec<GarageYrBlt>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "GarageFinish")
            .unwrap();
        let garagefinish = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => GarageFinish::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => GarageFinish::Null,

                _ => panic!(),
            })
            .collect::<Vec<GarageFinish>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "GarageCars")
            .unwrap();
        let garagecars = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => GarageCars::Int(*i),
                csv_deserializer::CsvAny::Null => GarageCars::Null,

                _ => panic!(),
            })
            .collect::<Vec<GarageCars>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "GarageArea")
            .unwrap();
        let garagearea = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => GarageArea::Int(*i),
                csv_deserializer::CsvAny::Null => GarageArea::Null,

                _ => panic!(),
            })
            .collect::<Vec<GarageArea>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "GarageQual")
            .unwrap();
        let garagequal = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => GarageQual::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => GarageQual::Null,

                _ => panic!(),
            })
            .collect::<Vec<GarageQual>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "GarageCond")
            .unwrap();
        let garagecond = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => GarageCond::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => GarageCond::Null,

                _ => panic!(),
            })
            .collect::<Vec<GarageCond>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "PavedDrive")
            .unwrap();
        let paveddrive = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => PavedDrive::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => PavedDrive::Null,

                _ => panic!(),
            })
            .collect::<Vec<PavedDrive>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "WoodDeckSF")
            .unwrap();
        let wooddecksf = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => WoodDeckSF::Int(*i),
                csv_deserializer::CsvAny::Null => WoodDeckSF::Null,

                _ => panic!(),
            })
            .collect::<Vec<WoodDeckSF>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "OpenPorchSF")
            .unwrap();
        let openporchsf = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => OpenPorchSF::Int(*i),
                csv_deserializer::CsvAny::Null => OpenPorchSF::Null,

                _ => panic!(),
            })
            .collect::<Vec<OpenPorchSF>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "EnclosedPorch")
            .unwrap();
        let enclosedporch = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => EnclosedPorch::Int(*i),
                csv_deserializer::CsvAny::Null => EnclosedPorch::Null,

                _ => panic!(),
            })
            .collect::<Vec<EnclosedPorch>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "ThreeSsnPorch")
            .unwrap();
        let threessnporch = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => ThreeSsnPorch::Int(*i),
                csv_deserializer::CsvAny::Null => ThreeSsnPorch::Null,

                _ => panic!(),
            })
            .collect::<Vec<ThreeSsnPorch>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "ScreenPorch")
            .unwrap();
        let screenporch = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => ScreenPorch::Int(*i),
                csv_deserializer::CsvAny::Null => ScreenPorch::Null,

                _ => panic!(),
            })
            .collect::<Vec<ScreenPorch>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "PoolArea")
            .unwrap();
        let poolarea = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => PoolArea::Int(*i),
                csv_deserializer::CsvAny::Null => PoolArea::Null,

                _ => panic!(),
            })
            .collect::<Vec<PoolArea>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "PoolQC")
            .unwrap();
        let poolqc = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => PoolQC::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => PoolQC::Null,

                _ => panic!(),
            })
            .collect::<Vec<PoolQC>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "Fence")
            .unwrap();
        let fence = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => Fence::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => Fence::Null,

                _ => panic!(),
            })
            .collect::<Vec<Fence>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "MiscFeature")
            .unwrap();
        let miscfeature = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => MiscFeature::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => MiscFeature::Null,

                _ => panic!(),
            })
            .collect::<Vec<MiscFeature>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "MiscVal")
            .unwrap();
        let miscval = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => MiscVal::Int(*i),
                csv_deserializer::CsvAny::Null => MiscVal::Null,

                _ => panic!(),
            })
            .collect::<Vec<MiscVal>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "MoSold")
            .unwrap();
        let mosold = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => MoSold::Int(*i),
                csv_deserializer::CsvAny::Null => MoSold::Null,

                _ => panic!(),
            })
            .collect::<Vec<MoSold>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "YrSold")
            .unwrap();
        let yrsold = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => YrSold::Int(*i),
                csv_deserializer::CsvAny::Null => YrSold::Null,

                _ => panic!(),
            })
            .collect::<Vec<YrSold>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "SaleType")
            .unwrap();
        let saletype = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => SaleType::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => SaleType::Null,

                _ => panic!(),
            })
            .collect::<Vec<SaleType>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "SaleCondition")
            .unwrap();
        let salecondition = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Str(s) => SaleCondition::from_str(s).unwrap(),
                csv_deserializer::CsvAny::Null => SaleCondition::Null,

                _ => panic!(),
            })
            .collect::<Vec<SaleCondition>>();

        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "SalePrice")
            .unwrap();
        let saleprice = dataset.values[index]
            .iter()
            .map(|val| match val {
                csv_deserializer::CsvAny::Int(i) => SalePrice::Int(*i),
                csv_deserializer::CsvAny::Null => SalePrice::Null,

                _ => panic!(),
            })
            .collect::<Vec<SalePrice>>();

        let columns: Vec<CsvColumn> = dataset
            .names
            .iter()
            .enumerate()
            .filter_map(|(col_index, name)| CsvColumn::from_str(&name.raw).ok())
            .collect();

        CsvDataFrame {
            columns,
            id,
            mssubclass,
            mszoning,
            lotfrontage,
            lotarea,
            street,
            alley,
            lotshape,
            landcontour,
            utilities,
            lotconfig,
            landslope,
            neighborhood,
            conditionone,
            conditiontwo,
            bldgtype,
            housestyle,
            overallqual,
            overallcond,
            yearbuilt,
            yearremodadd,
            roofstyle,
            roofmatl,
            exterioronest,
            exteriortwond,
            masvnrtype,
            masvnrarea,
            exterqual,
            extercond,
            foundation,
            bsmtqual,
            bsmtcond,
            bsmtexposure,
            bsmtfintypeone,
            bsmtfinsfone,
            bsmtfintypetwo,
            bsmtfinsftwo,
            bsmtunfsf,
            totalbsmtsf,
            heating,
            heatingqc,
            centralair,
            electrical,
            onestflrsf,
            twondflrsf,
            lowqualfinsf,
            grlivarea,
            bsmtfullbath,
            bsmthalfbath,
            fullbath,
            halfbath,
            bedroomabvgr,
            kitchenabvgr,
            kitchenqual,
            totrmsabvgrd,
            functional,
            fireplaces,
            fireplacequ,
            garagetype,
            garageyrblt,
            garagefinish,
            garagecars,
            garagearea,
            garagequal,
            garagecond,
            paveddrive,
            wooddecksf,
            openporchsf,
            enclosedporch,
            threessnporch,
            screenporch,
            poolarea,
            poolqc,
            fence,
            miscfeature,
            miscval,
            mosold,
            yrsold,
            saletype,
            salecondition,
            saleprice,
        }
    }
}
