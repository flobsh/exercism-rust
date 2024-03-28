struct Rule {
    diviser: u32,
    sound: String,
}

impl Rule {
    pub fn new(diviser: u32, sound: impl Into<String>) -> Self {
        Rule {
            diviser,
            sound: sound.into(),
        }
    }
}

pub fn raindrops(n: u32) -> String {
    let ruleset = vec![
        Rule::new(3, "Pling"),
        Rule::new(5, "Plang"),
        Rule::new(7, "Plong"),
    ];

    let mut raindrop_sound = String::new();

    for rule in ruleset {
        if n % rule.diviser == 0 {
            raindrop_sound.push_str(&rule.sound);
        }
    }

    if raindrop_sound.is_empty() {
        n.to_string()
    } else {
        raindrop_sound
    }
}
