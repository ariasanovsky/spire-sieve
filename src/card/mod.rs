pub mod card_pool;

use strum::EnumCount;
use strum_macros::EnumCount;
use strum_macros::EnumIter;
use strum_macros::FromRepr;

use crate::character::Character;

pub const fn cards<const START: usize, const N: usize, const REVERSE: bool>() -> [Card; N] {
    let mut cards = [Card::Invalid; N];
    let mut i = 0;
    let mut index = START;
    while i < N {
        cards[i] = if let Some(card) = Card::from_repr(index) {
            card
        } else {
            continue;
        };
        i += 1;
        if REVERSE {
            index -= 1;
        } else {
            index += 1;
        }
    }
    cards
}

pub const CARDS: [Card; Card::COUNT] = cards::<0, { Card::COUNT }, false>();

// const unsafe fn cards_slice<const START: usize, const END: usize>() -> &'static [Card] {
//     unsafe { CARDS.get_unchecked(START..END) }
// } // not yet stable :(
// also, as_ref() is not defined as const

#[test]
fn test_card_array_initialization() {
    dbg!(&CARDS, CARDS.len());
    dbg!(std::mem::size_of_val(&CARDS));
    dbg!(std::mem::size_of::<Card>());
}

pub const fn card_pool_range(character: Character, rarity: Rarity) -> (Card, Card, bool) {
    let first: Card = [
        [
            Card::SwordBoomerang,
            Card::SwordBoomerang,
            Card::Evolve,
            Card::DoubleTap,
        ],
        [
            Card::FlyingKnee,
            Card::FlyingKnee,
            Card::Predator,
            Card::Alchemize,
        ],
    ][character as usize][rarity as usize];
    let last: Card = [
        [
            Card::Immolate,
            Card::Anger,
            Card::SpotWeakness,
            Card::Immolate,
        ],
        [
            Card::GrandFinale,
            Card::CloakAndDagger,
            Card::CripplingCloud,
            Card::GrandFinale,
        ],
    ][character as usize][rarity as usize];
    let reverse: bool =
        [[false, true, true, true], [false, true, true, true]][character as usize][rarity as usize];
    (first, last, reverse)
}

const _IRONCLAD_CARD_RANGE: (Card, Card, bool) =
    card_pool_range(Character::Ironclad, Rarity::Common);

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, EnumCount)]
pub enum Rarity {
    Any,
    Common,
    Uncommon,
    Rare,
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
