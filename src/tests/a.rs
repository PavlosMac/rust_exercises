#[cfg(test)]
use crate::model::Group;
use crate::solutions::{a, b};
use crate::tests::samples;
use crate::tests::samples::{batman, batman_poor, catman, catwoman_poor, deadshot, scandal};

#[test]
fn should_return_none_if_groups_is_empty() {
    let groups = Vec::new();
    let results = a::find_largest_group(&groups);
    assert!(results.is_none());
}

#[test]
fn should_return_group_if_groups_has_only_one() {
    let group = Group {
        name: "The missfits",
        members: Vec::new(),
    };
    let groups = vec![group];
    let result = a::find_largest_group(&groups);

    assert!(result.is_some());
    assert_eq!(result, groups.first());
}

#[test]
fn should_return_largest_group() {
    let groups = vec![samples::sinister_five(), samples::justice_league()];
    let result = a::find_largest_group(&groups);
    assert!(result.is_some());
    assert_eq!(result, groups.first());
}

#[test]

fn find_super_powers_with_sidekick() {
    let supers = vec![
        catman(),
        deadshot(),
        scandal(),
        catwoman_poor(),
        batman_poor(),
    ];

    let result = b::find_super_powers_with_sidekick(&supers);
    println!("{:?}", result);
    assert_eq!(result.len(), 3);
    // assert_eq!(*result.first().unwrap(), &catman());
}
