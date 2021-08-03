pub mod a {
    use crate::model::Group;

    pub fn find_largest_group<'a>(groups: &'a Vec<Group>) -> Option<&'a Group<'a>> {
        groups
            .iter()
            .max_by(|&g1, &g2| g1.members.len().partial_cmp(&g2.members.len()).unwrap())
    }
}

pub mod b {
    use crate::model::Super;

    pub fn find_super_powers_with_sidekick<'a>(supers: &'a Vec<Super<'a>>) -> Vec<&Super<'a>> {
        supers.iter().filter(|&s| s.sidekick.is_some()).collect()
    }
}
