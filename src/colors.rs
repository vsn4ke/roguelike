use bracket_lib::terminal::RGB;

pub const BLACK: &str = "#000000";
pub const WHITE: &str = "#FFFFFF";
pub const GREY: &str = "#999999";

// wood floor
pub const BROWN1: &str = "#833e20";
pub const BROWN2: &str = "#8f4c28";
pub const BROWN3: &str = "#965a31";
pub const BROWN4: &str = "#a67c51";
pub const BROWN5: &str = "#b99d67";

// forest green
pub const GREEN1: &str = "#003101";
pub const GREEN2: &str = "#053800";
pub const GREEN3: &str = "#024000";
pub const GREEN4: &str = "#064900";
pub const GREEN5: &str = "#00510a";
pub const GREEN6: &str = "#008800";

// rock gray
pub const GRAY1: &str = "#2d2c2c";
pub const GRAY2: &str = "#3a3232";
pub const GRAY3: &str = "#493c3c";
pub const GRAY4: &str = "#5c4949";
pub const GRAY5: &str = "#655353";
pub const GRAY6: &str = "#858893";

// reds
pub const RED1: &str = "#b62020";
pub const RED2: &str = "#cb2424";
pub const RED3: &str = "#fe2e2e";
pub const RED4: &str = "#fe5757";
pub const RED5: &str = "#fe8181";

// yellow
pub const YELLOW1: &str = "#f9e909";
pub const YELLOW2: &str = "#fdf25d";
pub const YELLOW3: &str = "#fcff83";
pub const YELLOW4: &str = "#fbfd9e";
pub const YELLOW5: &str = "#feffc3";

// blues
pub const BLUE1: &str = "#001eff";
pub const BLUE2: &str = "#001be7";
pub const BLUE3: &str = "#0119cb";
pub const BLUE4: &str = "#021496";
pub const BLUE5: &str = "#000b5e";

pub const DEEPSEA1: &str = "#0200c5";
pub const DEEPSEA2: &str = "#0100af";
pub const DEEPSEA3: &str = "#0100a0";
pub const DEEPSEA4: &str = "#010090";
pub const DEEPSEA5: &str = "#010088";

pub const SHALLOWWATERS1: &str = "#77d9d9";
pub const SHALLOWWATERS2: &str = "#77c5d9";
pub const SHALLOWWATERS3: &str = "#77b2d9";
pub const SHALLOWWATERS4: &str = "#779ed9";
pub const SHALLOWWATERS5: &str = "#778bd9";

pub fn c(color: &str) -> RGB {
    RGB::from_hex(color).unwrap()
}
