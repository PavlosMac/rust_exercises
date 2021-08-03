use crate::model::{Group, Super};

pub(in crate::tests) fn catman<'a>() -> Super<'a> {
    Super {
        super_name: "Catman",
        real_name: "Thomas Blake",
        power: 60,
        sidekick: Some(Box::from(Some(batman()))),
    }
}

pub(in crate::tests) fn deadshot<'a>() -> Super<'a> {
    Super {
        super_name: "Catman",
        real_name: "Floyd Lawton",
        power: 40,
        sidekick: Some(Box::from(Some(scandal()))),
    }
}

pub(in crate::tests) fn scandal<'a>() -> Super<'a> {
    Super {
        super_name: "Scandal",
        real_name: "Scandal Savage",
        power: 50,
        sidekick: Some(Box::from(Some(deadshot()))),
    }
}

pub(in crate::tests) fn batman<'a>() -> Super<'a> {
    Super {
        super_name: "Batman",
        real_name: "Brucey",
        power: 50,
        sidekick: None,
    }
}

pub(in crate::tests) fn rag_doll<'a>() -> Super<'a> {
    Super {
        super_name: "Rag Doll",
        real_name: "Peter Merkel Jr",
        power: 60,
        sidekick: None,
    }
}

pub(in crate::tests) fn catwoman_poor<'a>() -> Super<'a> {
    Super {
        super_name: "Catwoman",
        real_name: "Cloey",
        power: 50,
        sidekick: None,
    }
}

pub(in crate::tests) fn batman_poor<'a>() -> Super<'a> {
    Super {
        super_name: "Batman",
        real_name: "Brucey",
        power: 50,
        sidekick: None,
    }
}

pub(in crate::tests) fn justice_league<'a>() -> Group<'a> {
    Group {
        name: "Justice League",
        members: vec![deadshot()],
    }
}

pub(in crate::tests) fn sinister_five<'a>() -> Group<'a> {
    Group {
        name: "Sinister 4",
        members: vec![catman(), deadshot(), scandal(), rag_doll(), batman()],
    }
}
