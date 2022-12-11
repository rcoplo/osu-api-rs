
/// UserType 枚举
pub enum UserType<'a> {
    /// user id
    ///
    /// https://osu.ppy.sh/users/18267600
    ///
    ///                             ^
    USERID(i64),
    /// user name
    /// <h3>osu 用户名</h3>
    USERNAME(&'a str),
}
/// 提交的数据类型不一样怎么办?
///
/// 直接定义一个枚举(
pub enum DataType<'a> {
    Int64(Option<i64>),
    Int32(Option<i32>),
    Int16(Option<i16>),
    Int8(Option<i8>),
    Mode(Option<Mode>,bool),
    String(Option<&'a str>),
    Mods(Option<Vec<Mods>>),
}
/// osu Mods
pub enum Mods{
    NONE,
    NF,
    EZ,
    MR,
    NV,
    HD,
    HR,
    SD,
    DT,
    RX,
    HT,
    NC,
    FL,
    AUTO,
    SO,
    AP,
    PF,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key10,
    FadeIn,
    Random,
    Cinema,
    Coop,
    V2,
    LM,
    KeyMod,
    FreeModAllowed,
}
/// osu mode
pub enum Mode{
    Fruits,
    Mania,
    Osu,
    Taiko,
}
