extern crate unicode_names;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;


// This is the list of emoji names in our alphabet.
static ALPHABET: &'static [&'static str] = &[
    //   0 --  31
    "THUMBS UP SIGN",                               "THUMBS DOWN SIGN",
    "FISTED HAND SIGN",                             "VICTORY HAND",
    "RAISED HAND",                                  "OK HAND SIGN",
    "CLAPPING HANDS SIGN",                          "WAVING HAND SIGN",
    "WHITE UP POINTING BACKHAND INDEX",             "WHITE DOWN POINTING BACKHAND INDEX",
    "WHITE LEFT POINTING BACKHAND INDEX",           "WHITE RIGHT POINTING BACKHAND INDEX",
    "WRITING HAND",                                 "EYE",
    "EAR",                                          "NOSE",
    "FOOTPRINTS",                                   "ROBOT FACE",
    "CROSS MARK",                                   "HEAVY PLUS SIGN",
    "HEAVY MINUS SIGN",                             "HEAVY DIVISION SIGN",
    "HEAVY CHECK MARK",                             "HEAVY EXCLAMATION MARK SYMBOL",
    "BLACK QUESTION MARK ORNAMENT",                 "EXCLAMATION QUESTION MARK",
    "CROSSED FLAGS",                                "HUNDRED POINTS SYMBOL",
    "KISS MARK",                                    "RING",
    "GEM STONE",                                    "HEAVY BLACK HEART",

    //  32 --  63
    "GUITAR",                                       "SAXOPHONE",
    "MUSICAL KEYBOARD",                             "VIOLIN",
    "AMERICAN FOOTBALL",                            "BASKETBALL AND HOOP",
    "SOCCER BALL",                                  "BASEBALL",
    "TROPHY",                                       "CHEQUERED FLAG",
    "BOW AND ARROW",                                "SPORTS MEDAL",
    "VOLLEYBALL",                                   "CLOUD",
    "CLOUD WITH TORNADO",                           "CLOUD WITH RAIN",
    "CLOUD WITH LIGHTNING",                         "CURLY LOOP",
    "CHICKEN",                                      "PENGUIN",
    "WHALE",                                        "CRAB",
    "MONKEY",                                       "SHEEP",
    "SNAKE",                                        "HORSE",
    "ELEPHANT",                                     "OCTOPUS",
    "TURTLE",                                       "HONEYBEE",
    "PIG",                                          "CROCODILE",

    //  64 --  95
    "MOUSE",                                        "COW",
    "BIRD",                                         "SNAIL",
    "BACTRIAN CAMEL",                               "DOLPHIN",
    "DRAGON",                                       "SPIDER",
    "SPIDER WEB",                                   "CAT",
    "RABBIT",                                       "ANT",
    "FISH",                                         "SUSHI",
    "ICE CREAM",                                    "DOUGHNUT",
    "COOKIE",                                       "CHOCOLATE BAR",
    "LOLLIPOP",                                     "HAMBURGER",
    "SLICE OF PIZZA",                               "BREAD",
    "BIRTHDAY CAKE",                                "TACO",
    "HOT DOG",                                      "CHEESE WEDGE",
    "RICE BALL",                                    "SPAGHETTI",
    "POPCORN",                                      "RED APPLE",
    "PINEAPPLE",                                    "BANANA",

    //  96 -- 127
    "GRAPES",                                       "WATERMELON",
    "CHERRIES",                                     "STRAWBERRY",
    "TANGERINE",                                    "LEMON",
    "PEACH",                                        "PEAR",
    "MUSHROOM",                                     "TOMATO",
    "AUBERGINE",                                    "HOT PEPPER",
    "EAR OF MAIZE",                                 "WINE GLASS",
    "BEER MUG",                                     "HOT BEVERAGE",
    "COCKTAIL GLASS",                               "BOTTLE WITH POPPING CORK",
    "JACK-O-LANTERN",                               "CHRISTMAS TREE",
    "FATHER CHRISTMAS",                             "WRAPPED PRESENT",
    "SNOWMAN",                                      "SNOWFLAKE",
    "WATCH",                                        "HOURGLASS",
    "ALARM CLOCK",                                  "BLACK TELEPHONE",
    "BALLOON",                                      "PARTY POPPER",
    "CONFETTI BALL",                                "FIREWORKS",

    // 128 -- 159
    "GHOST",                                        "SKULL",
    "IMP",                                          "EXTRATERRESTRIAL ALIEN",
    "BLACK SUN WITH RAYS",                          "RAINBOW",
    "CRESCENT MOON",                                "UMBRELLA WITH RAIN DROPS",
    "WHITE MEDIUM STAR",                            "CACTUS",
    "ROSE",                                         "SUNFLOWER",
    "FOUR LEAF CLOVER",                             "MAPLE LEAF",
    "SEEDLING",                                     "PALM TREE",
    "LIPSTICK",                                     "NAIL POLISH",
    "TOP HAT",                                      "MICROPHONE",
    "MOVIE CAMERA",                                 "ARTIST PALETTE",
    "GAME DIE",                                     "WARNING SIGN",
    "CAMERA",                                       "MONEY BAG",
    "CREDIT CARD",                                  "HEAVY DOLLAR SIGN",
    "BLACK SPADE SUIT",                             "BLACK CLUB SUIT",
    "BLACK HEART SUIT",                             "BLACK DIAMOND SUIT",

    // 160 -- 191
    "ROCKET",                                       "FIRE ENGINE",
    "AUTOMOBILE",                                   "SHIP",
    "NO ENTRY SIGN",                                "BICYCLE",
    "TRACTOR",                                      "HELICOPTER",
    "AIRPLANE",                                     "VERTICAL TRAFFIC LIGHT",
    "FLOPPY DISK",                                  "OPTICAL DISC",
    "SATELLITE ANTENNA",                            "OPEN BOOK",
    "CALENDAR",                                     "CLIPBOARD",
    "PAPERCLIP",                                    "STRAIGHT RULER",
    "PUSHPIN",                                      "PENCIL",
    "BLACK SCISSORS",                               "LEFT-POINTING MAGNIFYING GLASS",
    "KEY",                                          "LOCK",
    "HOCHO",                                        "PISTOL",
    "WRENCH",                                       "HAMMER",
    "FORK AND KNIFE",                               "FIRE",
    "BOMB",                                         "SMOKING SYMBOL",

    // 192 -- 223
    "HIGH-HEELED SHOE",                             "ATHLETIC SHOE",
    "T-SHIRT",                                      "JEANS",
    "BIKINI",                                       "DRESS",
    "NECKTIE",                                      "CROWN",
    "EYEGLASSES",                                   "HANDBAG",
    "PILE OF POO",                                  "TOILET",
    "SHOWER",                                       "BATH",
    "GRADUATION CAP",                               "VOLCANO",
    "CHURCH",                                       "SQUARED OK",
    "RIBBON",                                       "PILL",
    "SYRINGE",                                      "BELL",
    "MICROSCOPE",                                   "CANDLE",
    "BLACK LEFT-POINTING TRIANGLE",                 "BLACK RIGHT-POINTING TRIANGLE",
    "LEFTWARDS BLACK ARROW",                        "UPWARDS BLACK ARROW",
    "DOWNWARDS BLACK ARROW",                        "NORTH EAST ARROW",
    "NORTH WEST ARROW",                             "SOUTH EAST ARROW",

    // NOTE: we explicitly don't have "BLACK RIGHTWARDS ARROW" or "RIGHTWARDS BLACK ARROW" (yes,
    // they're different) in the above list, due to the following problem:
    //  http://xahlee.info/comp/unicode_BLACK_RIGHTWARDS_problem.html

    // 224 - 255
    "SOUTH WEST ARROW",                             "LEFTWARDS ARROW WITH HOOK",
    "RIGHTWARDS ARROW WITH HOOK",                   "ANTICLOCKWISE DOWNWARDS AND UPWARDS OPEN CIRCLE ARROWS",
    "BLACK RIGHT-POINTING DOUBLE TRIANGLE",         "BLACK LEFT-POINTING DOUBLE TRIANGLE",
    "BLACK UP-POINTING DOUBLE TRIANGLE",            "BLACK DOWN-POINTING DOUBLE TRIANGLE",
    "DOUBLE VERTICAL BAR",                          "SPARKLES",
    "PEACE SYMBOL",                                 "RADIOACTIVE SIGN",
    "YIN YANG",                                     "STAR OF DAVID",
    "ANCHOR",                                       "GEAR",
    "ROLLER COASTER",                               "FERRIS WHEEL",
    "CIRCUS TENT",                                  "TRIANGULAR FLAG ON POST",
    "CLAPPER BOARD",                                "VIDEO GAME",
    "SLOT MACHINE",                                 "BILLIARDS",
    "MUSICAL NOTE",                                 "TRUMPET",
    "SKI AND SKI BOOT",                             "WEIGHT LIFTER",
    "FACTORY",                                      "TONGUE",
    "EYES",                                         "WOMAN WITH BUNNY EARS",
];


fn main() {
    assert_eq!(ALPHABET.len(), 256);

    let out_dir = env::var("OUT_DIR").expect("no OUT_DIR environment variable");
    let dest_path = Path::new(&out_dir).join("table.rs");
    let mut f = File::create(&dest_path).expect("couldn't create file");

    write!(f, "static LOOKUP_TABLE: &'static [char] = &[\n").unwrap();

    for name in ALPHABET.iter() {
        if let Some(ch) = unicode_names::character(name) {
            write!(f, "  '{}',\n", ch).unwrap();
        } else {
            panic!("could not get Unicode character for name: {}", name);
        }
    }

    write!(f, "];\n").unwrap();
}
