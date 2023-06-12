// use libm::floorf;
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from(color: u32) -> Self {
        Self {
            r: ((color & 0xFF000000) >> 24) as u8,
            g: ((color & 0x00FF0000) >> 16) as u8,
            b: ((color & 0x0000FF00) >> 8) as u8,
            a: (color & 0x000000FF) as u8,
        }
    }
    pub fn rgb(r: u8, b: u8, g: u8) -> Self {
        Self { r, g, b, a: 0xFF }
    }
    pub fn rgba(r: u8, b: u8, g: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    // pub fn grayscale(&self) -> u8 {
    //     floorf(0.2126 * self.r as f32 + 0.7152 * self.g as f32 + 0.0722 * self.b as f32) as u8
    // }
}

// Translated from C# colors
pub static ALICE_BLUE: &'static Color = &Color {
    r: 240,
    b: 248,
    g: 255,
    a: 255,
};
pub static LIGHT_SALMON: &'static Color = &Color {
    r: 255,
    b: 160,
    g: 122,
    a: 255,
};
pub static ANTIQUE_WHITE: &'static Color = &Color {
    r: 250,
    b: 235,
    g: 215,
    a: 255,
};
pub static LIGHT_SEA_GREEN: &'static Color = &Color {
    r: 32,
    b: 178,
    g: 170,
    a: 255,
};
pub static AQUA: &'static Color = &Color {
    r: 0,
    b: 255,
    g: 255,
    a: 255,
};
pub static LIGHT_SKY_BLUE: &'static Color = &Color {
    r: 135,
    b: 206,
    g: 250,
    a: 255,
};
pub static AQUAMARINE: &'static Color = &Color {
    r: 127,
    b: 255,
    g: 212,
    a: 255,
};
pub static LIGHT_SLATE_GRAY: &'static Color = &Color {
    r: 119,
    b: 136,
    g: 153,
    a: 255,
};
pub static AZURE: &'static Color = &Color {
    r: 240,
    b: 255,
    g: 255,
    a: 255,
};
pub static LIGHT_STEEL_BLUE: &'static Color = &Color {
    r: 176,
    b: 196,
    g: 222,
    a: 255,
};
pub static BEIGE: &'static Color = &Color {
    r: 245,
    b: 245,
    g: 220,
    a: 255,
};
pub static LIGHT_YELLOW: &'static Color = &Color {
    r: 255,
    b: 255,
    g: 224,
    a: 255,
};
pub static BISQUE: &'static Color = &Color {
    r: 255,
    b: 228,
    g: 196,
    a: 255,
};
pub static LIME: &'static Color = &Color {
    r: 0,
    b: 255,
    g: 0,
    a: 255,
};
pub static BLACK: &'static Color = &Color {
    r: 0,
    b: 0,
    g: 0,
    a: 255,
};
pub static LIME_GREEN: &'static Color = &Color {
    r: 50,
    b: 205,
    g: 50,
    a: 255,
};
pub static BLANCHED_ALMOND: &'static Color = &Color {
    r: 255,
    b: 255,
    g: 205,
    a: 255,
};
pub static LINEN: &'static Color = &Color {
    r: 250,
    b: 240,
    g: 230,
    a: 255,
};
pub static BLUE: &'static Color = &Color {
    r: 0,
    b: 0,
    g: 255,
    a: 255,
};
pub static MAGENTA: &'static Color = &Color {
    r: 255,
    b: 0,
    g: 255,
    a: 255,
};
pub static BLUE_VIOLET: &'static Color = &Color {
    r: 138,
    b: 43,
    g: 226,
    a: 255,
};
pub static MAROON: &'static Color = &Color {
    r: 128,
    b: 0,
    g: 0,
    a: 255,
};
pub static BROWN: &'static Color = &Color {
    r: 165,
    b: 42,
    g: 42,
    a: 255,
};
pub static MEDIUM_AQUAMARINE: &'static Color = &Color {
    r: 102,
    b: 205,
    g: 170,
    a: 255,
};
pub static BURLY_WOOD: &'static Color = &Color {
    r: 222,
    b: 184,
    g: 135,
    a: 255,
};
pub static MEDIUM_BLUE: &'static Color = &Color {
    r: 0,
    b: 0,
    g: 205,
    a: 255,
};
pub static CADET_BLUE: &'static Color = &Color {
    r: 95,
    b: 158,
    g: 160,
    a: 255,
};
pub static MEDIUM_ORCHID: &'static Color = &Color {
    r: 186,
    b: 85,
    g: 211,
    a: 255,
};
pub static CHARTREUSE: &'static Color = &Color {
    r: 127,
    b: 255,
    g: 0,
    a: 255,
};
pub static MEDIUM_PURPLE: &'static Color = &Color {
    r: 147,
    b: 112,
    g: 219,
    a: 255,
};
pub static CHOCOLATE: &'static Color = &Color {
    r: 210,
    b: 105,
    g: 30,
    a: 255,
};
pub static MEDIUM_SEA_GREEN: &'static Color = &Color {
    r: 60,
    b: 179,
    g: 113,
    a: 255,
};
pub static CORAL: &'static Color = &Color {
    r: 255,
    b: 127,
    g: 80,
    a: 255,
};
pub static MEDIUM_SLATE_BLUE: &'static Color = &Color {
    r: 123,
    b: 104,
    g: 238,
    a: 255,
};
pub static CORNFLOWER_BLUE: &'static Color = &Color {
    r: 100,
    b: 149,
    g: 237,
    a: 255,
};
pub static MEDIUM_SPRING_GREEN: &'static Color = &Color {
    r: 0,
    b: 250,
    g: 154,
    a: 255,
};
pub static CORNSILK: &'static Color = &Color {
    r: 255,
    b: 248,
    g: 220,
    a: 255,
};
pub static MEDIUM_TURQUOISE: &'static Color = &Color {
    r: 72,
    b: 209,
    g: 204,
    a: 255,
};
pub static CRIMSON: &'static Color = &Color {
    r: 220,
    b: 20,
    g: 60,
    a: 255,
};
pub static MEDIUM_VIOLET_RED: &'static Color = &Color {
    r: 199,
    b: 21,
    g: 112,
    a: 255,
};
pub static CYAN: &'static Color = &Color {
    r: 0,
    b: 255,
    g: 255,
    a: 255,
};
pub static MIDNIGHT_BLUE: &'static Color = &Color {
    r: 25,
    b: 25,
    g: 112,
    a: 255,
};
pub static DARK_BLUE: &'static Color = &Color {
    r: 0,
    b: 0,
    g: 139,
    a: 255,
};
pub static MINT_CREAM: &'static Color = &Color {
    r: 245,
    b: 255,
    g: 250,
    a: 255,
};
pub static DARK_CYAN: &'static Color = &Color {
    r: 0,
    b: 139,
    g: 139,
    a: 255,
};
pub static MISTY_ROSE: &'static Color = &Color {
    r: 255,
    b: 228,
    g: 225,
    a: 255,
};
pub static DARK_GOLDENROD: &'static Color = &Color {
    r: 184,
    b: 134,
    g: 11,
    a: 255,
};
pub static MOCCASIN: &'static Color = &Color {
    r: 255,
    b: 228,
    g: 181,
    a: 255,
};
pub static DARK_GRAY: &'static Color = &Color {
    r: 169,
    b: 169,
    g: 169,
    a: 255,
};
pub static NAVAJO_WHITE: &'static Color = &Color {
    r: 255,
    b: 222,
    g: 173,
    a: 255,
};
pub static DARK_GREEN: &'static Color = &Color {
    r: 0,
    b: 100,
    g: 0,
    a: 255,
};
pub static NAVY: &'static Color = &Color {
    r: 0,
    b: 0,
    g: 128,
    a: 255,
};
pub static DARK_KHAKI: &'static Color = &Color {
    r: 189,
    b: 183,
    g: 107,
    a: 255,
};
pub static OLD_LACE: &'static Color = &Color {
    r: 253,
    b: 245,
    g: 230,
    a: 255,
};
pub static DARK_MAGENA: &'static Color = &Color {
    r: 139,
    b: 0,
    g: 139,
    a: 255,
};
pub static OLIVE: &'static Color = &Color {
    r: 128,
    b: 128,
    g: 0,
    a: 255,
};
pub static DARK_OLIVE_GREEN: &'static Color = &Color {
    r: 85,
    b: 107,
    g: 47,
    a: 255,
};
pub static OLIVE_DRAB: &'static Color = &Color {
    r: 107,
    b: 142,
    g: 45,
    a: 255,
};
pub static DARK_ORANGE: &'static Color = &Color {
    r: 255,
    b: 140,
    g: 0,
    a: 255,
};
pub static ORANGE: &'static Color = &Color {
    r: 255,
    b: 165,
    g: 0,
    a: 255,
};
pub static DARK_ORCHID: &'static Color = &Color {
    r: 153,
    b: 50,
    g: 204,
    a: 255,
};
pub static ORANGE_RED: &'static Color = &Color {
    r: 255,
    b: 69,
    g: 0,
    a: 255,
};
pub static DARK_RED: &'static Color = &Color {
    r: 139,
    b: 0,
    g: 0,
    a: 255,
};
pub static ORCHID: &'static Color = &Color {
    r: 218,
    b: 112,
    g: 214,
    a: 255,
};
pub static DARK_SALMON: &'static Color = &Color {
    r: 233,
    b: 150,
    g: 122,
    a: 255,
};
pub static PALE_GOLDENROD: &'static Color = &Color {
    r: 238,
    b: 232,
    g: 170,
    a: 255,
};
pub static DARK_SEA_GREEN: &'static Color = &Color {
    r: 143,
    b: 188,
    g: 143,
    a: 255,
};
pub static PALE_GREEN: &'static Color = &Color {
    r: 152,
    b: 251,
    g: 152,
    a: 255,
};
pub static DARK_SLATE_BLUE: &'static Color = &Color {
    r: 72,
    b: 61,
    g: 139,
    a: 255,
};
pub static PALE_TURQUOISE: &'static Color = &Color {
    r: 175,
    b: 238,
    g: 238,
    a: 255,
};
pub static DARK_SLATE_GRAY: &'static Color = &Color {
    r: 40,
    b: 79,
    g: 79,
    a: 255,
};
pub static PALE_VIOLET_RED: &'static Color = &Color {
    r: 219,
    b: 112,
    g: 147,
    a: 255,
};
pub static DARK_TURQUOISE: &'static Color = &Color {
    r: 0,
    b: 206,
    g: 209,
    a: 255,
};
pub static PAPAYA_WHIP: &'static Color = &Color {
    r: 255,
    b: 239,
    g: 213,
    a: 255,
};
pub static DARK_VIOLET: &'static Color = &Color {
    r: 148,
    b: 0,
    g: 211,
    a: 255,
};
pub static PEACH_PUFF: &'static Color = &Color {
    r: 255,
    b: 218,
    g: 155,
    a: 255,
};
pub static DEEP_PINK: &'static Color = &Color {
    r: 255,
    b: 20,
    g: 147,
    a: 255,
};
pub static PERU: &'static Color = &Color {
    r: 205,
    b: 133,
    g: 63,
    a: 255,
};
pub static DEEP_SKY_BLUE: &'static Color = &Color {
    r: 0,
    b: 191,
    g: 255,
    a: 255,
};
pub static PINK: &'static Color = &Color {
    r: 255,
    b: 192,
    g: 203,
    a: 255,
};
pub static DIM_GRAY: &'static Color = &Color {
    r: 105,
    b: 105,
    g: 105,
    a: 255,
};
pub static PLUM: &'static Color = &Color {
    r: 221,
    b: 160,
    g: 221,
    a: 255,
};
pub static DODGER_BLUE: &'static Color = &Color {
    r: 30,
    b: 144,
    g: 255,
    a: 255,
};
pub static POWDER_BLUE: &'static Color = &Color {
    r: 176,
    b: 224,
    g: 230,
    a: 255,
};
pub static FIREBRICK: &'static Color = &Color {
    r: 178,
    b: 34,
    g: 34,
    a: 255,
};
pub static PURPLE: &'static Color = &Color {
    r: 128,
    b: 0,
    g: 128,
    a: 255,
};
pub static FLORAL_WHITE: &'static Color = &Color {
    r: 255,
    b: 250,
    g: 240,
    a: 255,
};
pub static RED: &'static Color = &Color {
    r: 255,
    b: 0,
    g: 0,
    a: 255,
};
pub static FOREST_GREEN: &'static Color = &Color {
    r: 34,
    b: 139,
    g: 34,
    a: 255,
};
pub static ROSY_BROWN: &'static Color = &Color {
    r: 188,
    b: 143,
    g: 143,
    a: 255,
};
pub static FUSCHIA: &'static Color = &Color {
    r: 255,
    b: 0,
    g: 255,
    a: 255,
};
pub static ROYAL_BLUE: &'static Color = &Color {
    r: 65,
    b: 105,
    g: 225,
    a: 255,
};
pub static GAINSBORO: &'static Color = &Color {
    r: 220,
    b: 220,
    g: 220,
    a: 255,
};
pub static SADDLE_BROWN: &'static Color = &Color {
    r: 139,
    b: 69,
    g: 19,
    a: 255,
};
pub static GHOST_WHITE: &'static Color = &Color {
    r: 248,
    b: 248,
    g: 255,
    a: 255,
};
pub static SALMON: &'static Color = &Color {
    r: 250,
    b: 128,
    g: 114,
    a: 255,
};
pub static GOLD: &'static Color = &Color {
    r: 255,
    b: 215,
    g: 0,
    a: 255,
};
pub static SANDY_BROWN: &'static Color = &Color {
    r: 244,
    b: 164,
    g: 96,
    a: 255,
};
pub static GOLDENROD: &'static Color = &Color {
    r: 218,
    b: 165,
    g: 32,
    a: 255,
};
pub static SEA_GREEN: &'static Color = &Color {
    r: 46,
    b: 139,
    g: 87,
    a: 255,
};
pub static GRAY: &'static Color = &Color {
    r: 128,
    b: 128,
    g: 128,
    a: 255,
};
pub static SEASHELL: &'static Color = &Color {
    r: 255,
    b: 245,
    g: 238,
    a: 255,
};
pub static GREEN: &'static Color = &Color {
    r: 0,
    b: 128,
    g: 0,
    a: 255,
};
pub static SIENNA: &'static Color = &Color {
    r: 160,
    b: 82,
    g: 45,
    a: 255,
};
pub static GREEN_YELLOW: &'static Color = &Color {
    r: 173,
    b: 255,
    g: 47,
    a: 255,
};
pub static SILVER: &'static Color = &Color {
    r: 192,
    b: 192,
    g: 192,
    a: 255,
};
pub static HONEYDEW: &'static Color = &Color {
    r: 240,
    b: 255,
    g: 240,
    a: 255,
};
pub static SKY_BLUE: &'static Color = &Color {
    r: 135,
    b: 206,
    g: 235,
    a: 255,
};
pub static HOT_PINK: &'static Color = &Color {
    r: 255,
    b: 105,
    g: 180,
    a: 255,
};
pub static SLATE_BLUE: &'static Color = &Color {
    r: 106,
    b: 90,
    g: 205,
    a: 255,
};
pub static INDIAN_RED: &'static Color = &Color {
    r: 205,
    b: 92,
    g: 92,
    a: 255,
};
pub static SLATE_GRAY: &'static Color = &Color {
    r: 112,
    b: 128,
    g: 144,
    a: 255,
};
pub static INDIGO: &'static Color = &Color {
    r: 75,
    b: 0,
    g: 130,
    a: 255,
};
pub static SNOW: &'static Color = &Color {
    r: 255,
    b: 250,
    g: 250,
    a: 255,
};
pub static IVORY: &'static Color = &Color {
    r: 255,
    b: 240,
    g: 240,
    a: 255,
};
pub static SPRING_GREEN: &'static Color = &Color {
    r: 0,
    b: 255,
    g: 127,
    a: 255,
};
pub static KHAKI: &'static Color = &Color {
    r: 240,
    b: 230,
    g: 140,
    a: 255,
};
pub static STEEL_BLUE: &'static Color = &Color {
    r: 70,
    b: 130,
    g: 180,
    a: 255,
};
pub static LAVENDER: &'static Color = &Color {
    r: 230,
    b: 230,
    g: 250,
    a: 255,
};
pub static TAN: &'static Color = &Color {
    r: 210,
    b: 180,
    g: 140,
    a: 255,
};
pub static LAVENDER_BLUSH: &'static Color = &Color {
    r: 255,
    b: 240,
    g: 245,
    a: 255,
};
pub static TEAL: &'static Color = &Color {
    r: 0,
    b: 128,
    g: 128,
    a: 255,
};
pub static LAWN_GREEN: &'static Color = &Color {
    r: 124,
    b: 252,
    g: 0,
    a: 255,
};
pub static THISTLE: &'static Color = &Color {
    r: 216,
    b: 191,
    g: 216,
    a: 255,
};
pub static LEMON_CHIFFON: &'static Color = &Color {
    r: 255,
    b: 250,
    g: 205,
    a: 255,
};
pub static TOMATO: &'static Color = &Color {
    r: 253,
    b: 99,
    g: 71,
    a: 255,
};
pub static LIGHT_BLUE: &'static Color = &Color {
    r: 173,
    b: 216,
    g: 230,
    a: 255,
};
pub static TURQUOISE: &'static Color = &Color {
    r: 64,
    b: 224,
    g: 208,
    a: 255,
};
pub static LIGHT_CORAL: &'static Color = &Color {
    r: 240,
    b: 128,
    g: 128,
    a: 255,
};
pub static VIOLET: &'static Color = &Color {
    r: 238,
    b: 130,
    g: 238,
    a: 255,
};
pub static LIGHT_CYAN: &'static Color = &Color {
    r: 224,
    b: 255,
    g: 255,
    a: 255,
};
pub static WHEAT: &'static Color = &Color {
    r: 245,
    b: 222,
    g: 179,
    a: 255,
};
pub static LIGHT_GOLDENROD_YELLOW: &'static Color = &Color {
    r: 250,
    b: 250,
    g: 210,
    a: 255,
};
pub static WHITE: &'static Color = &Color {
    r: 255,
    b: 255,
    g: 255,
    a: 255,
};
pub static LIGHT_GREEN: &'static Color = &Color {
    r: 144,
    b: 238,
    g: 144,
    a: 255,
};
pub static WHITE_SMOKE: &'static Color = &Color {
    r: 245,
    b: 245,
    g: 245,
    a: 255,
};
pub static LIGHT_GRAY: &'static Color = &Color {
    r: 211,
    b: 211,
    g: 211,
    a: 255,
};
pub static YELLOW: &'static Color = &Color {
    r: 255,
    b: 255,
    g: 0,
    a: 255,
};
pub static LIGHT_PINK: &'static Color = &Color {
    r: 255,
    b: 182,
    g: 193,
    a: 255,
};
pub static YELLOW_GREEN: &'static Color = &Color {
    r: 154,
    b: 205,
    g: 50,
    a: 255,
};
