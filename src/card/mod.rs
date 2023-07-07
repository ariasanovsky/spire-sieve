pub mod pandoras_box;
pub mod pool;
pub mod reward;

use strum::EnumCount;
use strum_macros::EnumCount;
use strum_macros::EnumIter;
use strum_macros::FromRepr;

pub struct CardSlice<'a> {
    pub slice: &'a [Card],
}

impl<'a> CardSlice<'a> {
    pub const fn new(slice: &'a [Card]) -> Self {
        Self { slice }
    }

    pub const fn trim_before(self, card: Card) -> Self {
        let mut slice = self.slice;
        while let Some((other, rest)) = slice.split_first() {
            if card.equal_as_usize(other) {
                return Self { slice };
            }
            slice = rest;
        }
        Self { slice }
    }

    pub const fn trim_after(self, card: Card) -> Self {
        let mut slice = self.slice;
        while let Some((other, rest)) = slice.split_last() {
            if card.equal_as_usize(other) {
                return Self { slice };
            }
            slice = rest;
        }
        Self { slice }
    }

    pub const fn trim_inclusive(self, first: Card, last: Card) -> Self {
        self.trim_before(first).trim_after(last)
    }
}

const fn all_cards() -> [Card; Card::COUNT] {
    let mut cards = [Card::Invalid; Card::COUNT];
    let mut i = 0;
    while i < Card::COUNT {
        cards[i] = if let Some(card) = Card::from_repr(i) {
            card
        } else {
            continue;
        };
        i += 1;
    }
    cards
}

const fn rev_cards() -> [Card; Card::COUNT] {
    let mut cards = [Card::Invalid; Card::COUNT];
    let mut i = 0;
    while i < Card::COUNT {
        cards[i] = if let Some(card) = Card::from_repr(Card::COUNT - i - 1) {
            card
        } else {
            continue;
        };
        i += 1;
    }
    cards
}

pub const CARDS: [Card; Card::COUNT] = all_cards();
pub const REV_CARDS: [Card; Card::COUNT] = rev_cards();

#[test]
fn test_card_array_initialization() {
    use std::dbg;
    dbg!(&CARDS, CARDS.len());
    dbg!(std::mem::size_of_val(&CARDS));
    dbg!(std::mem::size_of::<Card>());
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, EnumCount)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
}

impl Card {
    const fn equal_as_usize(self, other: &Card) -> bool {
        self as usize == *other as usize
    }
}

#[derive(Debug, Eq, PartialEq, FromRepr, EnumIter, EnumCount, Clone, Copy, Default)]
pub enum Card {
    #[default]
    Invalid,
    // Ironclad
    // Common
    SwordBoomerang,
    PerfectedStrike,
    HeavyBlade,
    WildStrike,
    Headbutt,
    Havoc,
    Armaments,
    Clothesline,
    TwinStrike,
    PommelStrike,
    Thunderclap,
    Clash,
    ShrugItOff,
    TrueGrit,
    BodySlam,
    IronWave,
    Flex,
    Warcry,
    Cleave,
    Anger,
    // Uncommon
    Evolve,
    Uppercut,
    GhostlyArmor,
    FireBreathing,
    Dropkick,
    Carnage,
    Bloodletting,
    Rupture,
    SecondWind,
    SearingBlow,
    BattleTrance,
    Sentinel,
    Entrench,
    Rage,
    FeelNoPain,
    Disarm,
    SeeingRed,
    DarkEmbrace,
    Combust,
    Whirlwind,
    SeverSoul,
    Rampage,
    Shockwave,
    Metallicize,
    BurningPact,
    Pummel,
    FlameBarrier,
    BloodForBlood,
    Intimidate,
    Hemokinesis,
    RecklessCharge,
    InfernalBlade,
    DualWield,
    PowerThrough,
    Inflame,
    SpotWeakness,
    // Rare
    DoubleTap,
    DemonForm,
    Bludgeon,
    Feed,
    LimitBreak,
    Corruption,
    Barricade,
    FiendFire,
    Berserk,
    Impervious,
    Juggernaut,
    Brutality,
    Reaper,
    Exhume,
    Offering,
    Immolate,
    // Silent
    // Common
    FlyingKnee,
    DodgeAndRoll,
    SuckerPunch,
    PiercingWail,
    Prepared,
    Outmaneuver,
    Backflip,
    Slice,
    QuickSlash,
    Acrobatics,
    PoisonedStab,
    DaggerThrow,
    Deflect,
    BladeDance,
    Bane,
    DaggerSpray,
    DeadlyPoison,
    SneakyStrike,
    CloakAndDagger,
    // Uncommon
    Predator,
    AllOutAttack,
    Distraction,
    Footwork,
    Accuracy,
    MasterfulStab,
    Flechettes,
    Concentrate,
    BouncingFlask,
    Backstab,
    Dash,
    Eviscerate,
    Reflex,
    InfiniteBlades,
    NoxiousFumes,
    HeelHook,
    Terror,
    WellLaidPlans,
    Finisher,
    EscapePlan,
    CalculatedGamble,
    Skewer,
    RiddleWithHoles,
    EndlessAgony,
    Setup,
    Blur,
    Caltrops,
    Choke,
    Expertise,
    Tactician,
    Catalyst,
    LegSweep,
    CripplingCloud,
    // Rare
    Alchemize,
    CorpseExplosion,
    Malaise,
    PhantasmalKiller,
    DieDieDie,
    Adrenaline,
    Envenom,
    Doppelganger,
    Burst,
    WraithForm,
    ToolsOfTheTrade,
    Nightmare,
    Unload,
    AfterImage,
    BulletTime,
    StormOfSteel,
    GlassKnife,
    AThousandCuts,
    GrandFinale,
    // Defect
    // Common
    SteamBarrier,
    ColdSnap,
    Leap,
    BeamCell,
    Hologram,
    ChargeBattery,
    SweepingBeam,
    Turbo,
    Coolheaded,
    Claw,
    Rebound,
    Stack,
    Barrage,
    CompileDriver,
    Recursion,
    Streamline,
    BallLightning,
    GoForTheEyes,
    // Uncommon
    DoomAndGloom,
    Defragment,
    Capacitor,
    WhiteNoise,
    Skim,
    Recycle,
    Scrape,
    Bullseye,
    Reprogram,
    AutoShields,
    ReinforcedBody,
    DoubleEnergy,
    Darkness,
    RipAndTear,
    Ftl,
    ForceField,
    Equilibrium,
    Tempest,
    Heatsinks,
    StaticDischarge,
    BootSequence,
    Chill,
    Loop,
    SelfRepair,
    Melter,
    Chaos,
    Blizzard,
    Aggregate,
    Fusion,
    Consume,
    Glacier,
    Sunder,
    HelloWorld,
    Overclock,
    GeneticAlgorithm,
    Storm,
    // Rare
    MultiCast,
    Hyperbeam,
    ThunderStrike,
    BiasedCognition,
    MachineLearning,
    Electrodynamics,
    Buffer,
    Rainbow,
    Seek,
    MeteorStrike,
    EchoForm,
    AllForOne,
    Reboot,
    Amplify,
    CreativeAi,
    Fission,
    CoreSurge,
    // Watcher
    // Common
    EmptyFist,
    Prostrate,
    Evaluate,
    CrushJoints,
    PressurePoints,
    FollowUp,
    CutThroughFate,
    SashWhip,
    EmptyBody,
    Tranquility,
    Crescendo,
    ThirdEye,
    Protect,
    FlurryOfBlows,
    JustLucky,
    Halt,
    FlyingSleeves,
    BowlingBash,
    Consecrate,
    // Uncommon
    Pray,
    SignatureMove,
    Weave,
    EmptyMind,
    Nirvana,
    Tantrum,
    Conclude,
    Worship,
    Swivel,
    Perseverance,
    Meditate,
    Study,
    WaveOfTheHand,
    SandsOfTime,
    FearNoEvil,
    ReachHeaven,
    MentalFortress,
    DeceiveReality,
    Rushdown,
    InnerPeace,
    Collect,
    WreathOfFlame,
    Wallop,
    CarveReality,
    Fasting,
    LikeWater,
    ForeignInfluence,
    WindmillStrike,
    Indignation,
    BattleHymn,
    TalkToTheHand,
    Sanctity,
    Foresight,
    SimmeringFury,
    WheelKick,
    // Rare
    Judgment,
    ConjureBlade,
    MasterReality,
    Brilliance,
    Devotion,
    Blasphemy,
    Ragnarok,
    LessonLearned,
    Scrawl,
    Vault,
    Alpha,
    Wish,
    Omniscience,
    Establishment,
    SpiritShield,
    DevaForm,
    DeusExMachina,
    // Colorless
    // Uncommon
    BandageUp,
    Blind,
    DarkShackles,
    DeepBreath,
    Discovery,
    DramaticEntrance,
    Enlightenment,
    Finesse,
    FlashOfSteel,
    Forethought,
    GoodInstincts,
    Impatience,
    JackOfAllTrades,
    Madness,
    MindBlast,
    Panacea,
    PanicButton,
    Purity,
    SwiftStrike,
    Trip,
    // Rare
    Apotheosis,
    Chrysalis,
    HandOfGreed,
    Magnetism,
    MasterOfStrategy,
    Mayhem,
    Metamorphosis,
    Panache,
    SadisticNature,
    SecretTechnique,
    SecretWeapon,
    TheBomb,
    ThinkingAhead,
    Transmutation,
    Violence,
}
