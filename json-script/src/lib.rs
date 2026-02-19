// use proc_hitbox::{BASE_ARTICLE_HITBOX, BASE_HITBOX};
// use proc_hitbox_macro::*;
use smash::{lib::L2CValue, phx::Hash40};

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct CollisionPart : u32 {
        const BODY = 1 << 0;
        const ETC = 1 << 1;
        const LEGS = 1 << 2;
        const BODY_LEGS = 1 << 3;
        const HEAD = 1 << 4;
    }

    #[derive(Debug, Copy, Clone)]
    pub struct CollisionSituation : u32 {
        const G = 1 << 0;
        const A = 1 << 1;
        const ODD = 1 << 2;
        const IGNORE_DOWN = 1 << 31;

        const GA = 0x3;
        const G_d = 1 << 31 | 1 << 0;
        const GA_d = 1 << 31 | 0x3;
    }

    #[derive(Debug, Copy, Clone)]
    pub struct CollisionCategory : u32 {
        const FIGHTER = 1 << 0;
        const ENEMY = 1 << 1;
        const ITEM = 1 << 2;
        const GIMMICK = 1 << 3;
        const ITEM_ENEMY = 1 << 4;
        const FLOOR = 1 << 5;
        const WEAPON = 1 << 7;
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ShieldDamage {
    Add(f32),
    Transcendent,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum SoundLevel {
    S = 0,
    M = 1,
    L = 2,
    LL = 3,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum CollisionSound {
    None = 0x0,
    Punch = 0x1,
    Kick = 0x2,
    Cutup = 0x3,
    Coin = 0x4,
    Bat = 0x5,
    Harisen = 0x6,
    Elec = 0x7,
    Fire = 0x8,
    Water = 0x9,
    Grass = 0xa,
    Bomb = 0xb,
    Peachweapon = 0xc,
    Snake = 0xd,
    Ike = 0xe,
    Dedede = 0xf,
    Magic = 0x10,
    Kamehit = 0x11,
    PeachBinta = 0x12,
    PeachFryingpan = 0x13,
    PeachGolf = 0x14,
    PeachTennis = 0x15,
    PeachParasol = 0x16,
    DaisyBinta = 0x17,
    DaisyFryingpan = 0x18,
    DaisyGolf = 0x19,
    DaisyTennis = 0x1a,
    DaisyParasol = 0x1b,
    Lucario = 0x1c,
    MarthSword = 0x1d,
    MarthFinal = 0x1e,
    MarioCoin = 0x1f,
    MarioFinal = 0x20,
    LuigiCoin = 0x21,
    NessBat = 0x22,
    Freeze = 0x23,
    MarioFireball = 0x24,
    MarioCoinLast = 0x25,
    MarioMant = 0x26,
    FoxBlaster = 0x27,
    LuigiAttackdash = 0x28,
    LuigiSmash = 0x29,
    MarioWaterPump = 0x2a,
    PacmanBell = 0x2b,
    GuruguruHit = 0x2c,
    LizardonFire = 0x2d,
    TrainHit = 0x2e,
    MariodCoinLast = 0x2f,
    MariodMant = 0x30,
    MariodCapsule = 0x31,
    PacmanWater = 0x32,
    MiigunnerBlaster = 0x33,
    RefletFinalSword = 0x34,
    RefletFinalFire = 0x35,
    RefletFinalElec = 0x36,
    DuckhuntFinal = 0x37,
    ShulkFinalDanban = 0x38,
    ShulkFinalRiki = 0x39,
    FalcoBlaster = 0x3a,
    RyuPunch = 0x3b,
    RyuKick = 0x3c,
    LucasBat = 0x3d,
    RyuFinal01 = 0x3e,
    RyuFinal02 = 0x3f,
    RyuFinal03 = 0x40,
    CloudHit = 0x41,
    CloudSmash01 = 0x42,
    CloudSmash02 = 0x43,
    CloudSmash03 = 0x44,
    CloudFinal01 = 0x45,
    CloudFinal02 = 0x46,
    CloudFinal03 = 0x47,
    BayonettaHit01 = 0x48,
    BayonettaHit02 = 0x49,
    YoshiBiteHit = 0x4a,
    YoshiEggHit = 0x4b,
    RoyHit = 0x4c,
    ChromHit = 0x4d,
    FoxTail = 0x4e,
    Heavy = 0x4f,
    Slap = 0x50,
    ItemHammer = 0x51,
    InklingHit = 0x52,
    MarioLocalCoin = 0x53,
    MarioLocalCoinLast = 0x54,
    FamicomHit = 0x55,
    ZenigameShellhit = 0x56,
    SamusScrew = 0x57,
    SamusdScrew = 0x58,
    SamusScrewFinish = 0x59,
    SamusdScrewFinish = 0x5a,
    KenPunch = 0x5b,
    KenKick = 0x5c,
    KenFinal01 = 0x5d,
    KenFinal02 = 0x5e,
    KenFinal03 = 0x5f,
    ShizueHammer = 0x60,
    SimonWhip = 0x61,
    SimonCross = 0x62,
    RichterWhip = 0x63,
    RichterCross = 0x64,
    SheikFinalpunch = 0x65,
    SheikFinalkick = 0x66,
    MetaknightFinalhit = 0x67,
    RobotFinalhit = 0x68,
    KenShoryu = 0x69,
    DiddyScratch = 0x6a,
    MiienemygBlaster = 0x6b,
    ToonlinkHit = 0x6c,
    JackShot = 0x6d,
    BraveCriticalhit = 0x6e,
    BuddyWing = 0x6f,
    DollyPunch = 0x70,
    DollyKick = 0x71,
    DollyCritical = 0x72,
    DollySuperspecial01 = 0x73,
    MasterAxe = 0x74,
    MasterArrowMax = 0x75,
    MasterAttack100end = 0x76,
    TantanPunch01 = 0x77,
    TantanPunch02 = 0x78,
    TantanPunch03 = 0x79,
    TantanFinal = 0x7a,
    CloudFinalAppendhit01 = 0x7b,
    CloudFinalAppendhit02 = 0x7c,
    FlameFinal = 0x7d,
    DemonPunch01 = 0x7e,
    DemonPunch02 = 0x7f,
    DemonKick = 0x80,
    DemonCatchattack = 0x81,
    DemonFinal = 0x82,
    DemonThrowcommand = 0x83,
    DemonAttacksquat4 = 0x84,
    DemonAttacklw3 = 0x85,
    DemonAppeal = 0x86,
    TrailSlash = 0x87,
    TrailStab = 0x88,
    TrailCleave = 0x89,
    TrailCleaveSingle = 0x8a,
    TrailKick = 0x8b,
    TrailFinal = 0x8c,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum AttackRegion {
    None = 0,
    Head = 1,
    Body = 2,
    Hip = 3,
    Punch = 4,
    Elbow = 5,
    Kick = 6,
    Knee = 7,
    Throw = 8,
    Object = 9,
    Sword = 0xA,
    Hammer = 0xB,
    Bomb = 0xC,
    Spin = 0xD,
    Bite = 0xE,
    Magic = 0xF,
    Psi = 0x10,
    Palutena = 0x11,
    Aura = 0x12,
    Bat = 0x13,
    Parasol = 0x14,
    Pikmin = 0x15,
    Water = 0x16,
    Whip = 0x17,
    Tail = 0x18,
    Energy = 0x19,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum SetOff {
    Off = 0,
    On = 1,
    Thru = 2,
    NoStop = 3,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum LrCheck {
    Pos = 0,
    Speed = 1,
    Lr = 2,
    F = 3,
    B = 4,
    Part = 5,
    BackSlash = 6,
    Left = 7,
    Right = 8,
}

// hitbox_templates! {
//     pub BASE_HITBOX = {
//         fkb: 0,
//         hitlag: 1,
//         sdi: 1,
//         clank: SetOff::On,
//         facing: LrCheck::Pos,
//         set_weight: false,
//         shield_dmg: ShieldDamage::Add(0.0),
//         trip: 0,
//         rehit: 0,
//         reflectable: false,
//         absorbable: false,
//         flinchless: false,
//         disable_hitlag: false,
//         direct: true,
//         friendly_fire: false,
//         situation: CollisionSituation::GA,
//         category: CollisionCategory::all(),
//         hit_part: CollisionPart::all(),
//         effect: "collision_attr_normal",
//     };

//     pub BAYONETTA_KICK = {
//         extends: BASE_HITBOX,
//         sound_level: SoundLevel::M,
//         hit_sound: CollisionSound::Kick,
//         region: AttackRegion::Kick,
//     };
// }

// fn acmd(agent: &mut smash::lua2cpp::L2CAgentBase) {
//     const HITBOX_0: HitboxData = decl_hitbox! {
//         extends: BAYONETTA_KICK, id: 0, part: 0, bone: "hip", dmg: 8, angle: 48, kbg: 103, bkb: 35, size: 4.3, x: -2.6, y: 0, z: 0
//     };
//     hitbox!(agent, {extends: BAYONETTA_KICK, id: 0, part: 0, bone: "hip", dmg: 8, angle: 48, kbg: 103, bkb: 35, size: 4.3, x: -2.6, y: 0, z: 0});
//     create_hitbox(agent, &HITBOX_0);
// }

#[derive(Copy, Clone)]
pub struct HitboxTemplate {
    pub id: Option<u64>,
    pub part: Option<u64>,
    pub bone: Option<Hash40>,
    pub dmg: Option<f32>,
    pub angle: Option<f32>,
    pub kbg: Option<f32>,
    pub fkb: Option<f32>,
    pub bkb: Option<f32>,
    pub size: Option<f32>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
    pub x2: Option<f32>,
    pub y2: Option<f32>,
    pub z2: Option<f32>,
    pub hitlag: Option<f32>,
    pub sdi: Option<f32>,
    pub clank: Option<SetOff>,
    pub facing: Option<LrCheck>,
    pub set_weight: Option<bool>,
    pub shield_dmg: Option<ShieldDamage>,
    pub trip: Option<f32>,
    pub rehit: Option<i32>,
    pub reflectable: Option<bool>,
    pub absorbable: Option<bool>,
    pub flinchless: Option<bool>,
    pub disable_hitlag: Option<bool>,
    pub direct: Option<bool>,
    pub situation: Option<CollisionSituation>,
    pub category: Option<CollisionCategory>,
    pub hit_part: Option<CollisionPart>,
    pub friendly_fire: Option<bool>,
    pub effect: Option<Hash40>,
    pub sound_level: Option<SoundLevel>,
    pub hit_sound: Option<CollisionSound>,
    pub region: Option<AttackRegion>,
}

impl HitboxTemplate {
    pub const fn new() -> Self {
        Self {
            id: None,
            part: None,
            bone: None,
            dmg: None,
            angle: None,
            kbg: None,
            fkb: None,
            bkb: None,
            size: None,
            x: None,
            y: None,
            z: None,
            x2: None,
            y2: None,
            z2: None,
            hitlag: None,
            sdi: None,
            clank: None,
            facing: None,
            set_weight: None,
            shield_dmg: None,
            trip: None,
            rehit: None,
            reflectable: None,
            absorbable: None,
            flinchless: None,
            disable_hitlag: None,
            direct: None,
            situation: None,
            category: None,
            hit_part: None,
            friendly_fire: None,
            effect: None,
            sound_level: None,
            hit_sound: None,
            region: None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HitboxData {
    pub id: u64,
    pub part: u64,
    pub bone: Hash40,
    pub dmg: f32,
    pub angle: f32,
    pub kbg: f32,
    pub fkb: f32,
    pub bkb: f32,
    pub size: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub x2: Option<f32>,
    pub y2: Option<f32>,
    pub z2: Option<f32>,
    pub hitlag: f32,
    pub sdi: f32,
    pub clank: SetOff,
    pub facing: LrCheck,
    pub set_weight: bool,
    pub shield_dmg: ShieldDamage,
    pub trip: f32,
    pub rehit: i32,
    pub reflectable: bool,
    pub absorbable: bool,
    pub flinchless: bool,
    pub disable_hitlag: bool,
    pub direct: bool,
    pub situation: CollisionSituation,
    pub category: CollisionCategory,
    pub hit_part: CollisionPart,
    pub friendly_fire: bool,
    pub effect: Hash40,
    pub sound_level: SoundLevel,
    pub hit_sound: CollisionSound,
    pub region: AttackRegion,
}

impl HitboxData {
    pub const fn from_template_or_panic(template: HitboxTemplate) -> Self {
        macro_rules! or_const_panic {
            ($id:ident) => {{
                let Some(value) = template.$id else {
                    panic!(concat!("Hitbox is missing field '", stringify!($id), "'"));
                };
                value
            }};
        }

        Self {
            id: or_const_panic!(id),
            part: or_const_panic!(part),
            bone: or_const_panic!(bone),
            dmg: or_const_panic!(dmg),
            angle: or_const_panic!(angle),
            kbg: or_const_panic!(kbg),
            fkb: or_const_panic!(fkb),
            bkb: or_const_panic!(bkb),
            size: or_const_panic!(size),
            x: or_const_panic!(x),
            y: or_const_panic!(y),
            z: or_const_panic!(z),
            x2: template.x2,
            y2: template.y2,
            z2: template.z2,
            hitlag: or_const_panic!(hitlag),
            sdi: or_const_panic!(sdi),
            clank: or_const_panic!(clank),
            facing: or_const_panic!(facing),
            set_weight: or_const_panic!(set_weight),
            shield_dmg: or_const_panic!(shield_dmg),
            trip: or_const_panic!(trip),
            rehit: or_const_panic!(rehit),
            reflectable: or_const_panic!(reflectable),
            absorbable: or_const_panic!(absorbable),
            flinchless: or_const_panic!(flinchless),
            disable_hitlag: or_const_panic!(disable_hitlag),
            direct: or_const_panic!(direct),
            situation: or_const_panic!(situation),
            category: or_const_panic!(category),
            hit_part: or_const_panic!(hit_part),
            friendly_fire: or_const_panic!(friendly_fire),
            effect: or_const_panic!(effect),
            sound_level: or_const_panic!(sound_level),
            hit_sound: or_const_panic!(hit_sound),
            region: or_const_panic!(region),
        }
    }
}

#[macro_export]
macro_rules! lua_args {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        unsafe {
            $(
                $agent.push_lua_stack(&mut $arg.into());
            )*
        }
    }
}

fn opt_to_l2cvalue<T: Into<L2CValue>>(o: Option<T>) -> L2CValue {
    match o {
        Some(value) => value.into(),
        None => unsafe { L2CValue::new() },
    }
}

#[inline(never)]
pub fn create_hitbox(agent: &mut smash::lua2cpp::L2CAgentBase, hitbox: &HitboxData) {
    unsafe {
        agent.clear_lua_stack();
    }

    lua_args!(
        agent,
        hitbox.id,
        hitbox.part,
        hitbox.bone,
        hitbox.dmg,
        hitbox.angle,
        hitbox.kbg,
        hitbox.fkb,
        hitbox.bkb,
        hitbox.size,
        hitbox.x,
        hitbox.y,
        hitbox.z,
        opt_to_l2cvalue(hitbox.x2),
        opt_to_l2cvalue(hitbox.y2),
        opt_to_l2cvalue(hitbox.z2),
        hitbox.hitlag,
        hitbox.sdi,
        hitbox.clank as i32,
        hitbox.facing as i32,
        hitbox.set_weight,
        match hitbox.shield_dmg {
            ShieldDamage::Add(normal) => L2CValue::F32(normal),
            ShieldDamage::Transcendent => L2CValue::Hash40s("no"),
        },
        hitbox.trip,
        hitbox.rehit,
        hitbox.reflectable,
        hitbox.absorbable,
        hitbox.flinchless,
        hitbox.disable_hitlag,
        hitbox.direct,
        hitbox.situation.bits(),
        hitbox.category.bits(),
        hitbox.hit_part.bits(),
        hitbox.friendly_fire,
        hitbox.effect,
        hitbox.sound_level as i32,
        hitbox.hit_sound as i32,
        hitbox.region as i32,
    );
    unsafe { smash::app::sv_animcmd::ATTACK(agent.lua_state_agent) }
}
