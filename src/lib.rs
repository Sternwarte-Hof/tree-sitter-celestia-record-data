//! Tree-sitter parser family for Celestia Record Data documents.

use tree_sitter::Language;

extern "C" {
    fn tree_sitter_celestia_record_data() -> Language;
}

/// Returns the Tree-sitter [`Language`] for Celestia Record Data documents.
pub fn language() -> Language {
    unsafe { tree_sitter_celestia_record_data() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    #[test]
    fn test_grammar_name() {
        assert_eq!("celestia-record-data", "celestia-record-data");
    }

    #[test]
    fn test_parse_all_five_record_roles() {
        let mut parser = Parser::new();
        parser
            .set_language(&language())
            .expect("Error loading celestia record data language");

        // 1. XYZ Position Record
        let xyz = "2451545.000000 149597870.700 0.000 0.000";
        let xyz_tree = parser.parse(xyz, None).unwrap();
        assert!(!xyz_tree.root_node().has_error());
        assert_eq!(xyz_tree.root_node().child(0).unwrap().kind(), "xyz_record");

        // 2. XYZV Position and Velocity Record
        let xyzv = "2451545.000000 149597870.700 0.000 0.000 0.000 29.780 0.000";
        let xyzv_tree = parser.parse(xyzv, None).unwrap();
        assert!(!xyzv_tree.root_node().has_error());
        assert_eq!(
            xyzv_tree.root_node().child(0).unwrap().kind(),
            "xyzv_record"
        );

        // 3. Sampled Orientation Record
        let q = "2451545.000000 1.000000 0.000000 0.000000 0.000000";
        let q_tree = parser.parse(q, None).unwrap();
        assert!(!q_tree.root_node().has_error());
        assert_eq!(
            q_tree.root_node().child(0).unwrap().kind(),
            "orientation_record"
        );

        // 4. Constellation Boundaries Record
        let bnd = "0.0 90.0 UMa 1";
        let bnd_tree = parser.parse(bnd, None).unwrap();
        assert!(!bnd_tree.root_node().has_error());
        assert_eq!(
            bnd_tree.root_node().child(0).unwrap().kind(),
            "boundary_record"
        );

        // 5. Star Names Record
        let starnames = "1:Sol:Sun\n12345:Alpha Centauri A";
        let starnames_tree = parser.parse(starnames, None).unwrap();
        assert!(!starnames_tree.root_node().has_error());
        assert_eq!(
            starnames_tree.root_node().child(0).unwrap().kind(),
            "star_name_record"
        );
    }

    #[test]
    fn test_fatal_error_recovery_fixtures() {
        let mut parser = Parser::new();
        parser.set_language(&language()).unwrap();

        // Malformed record without numbers/tokens
        let malformed = "??? INVALID TOKEN STREAM ???";
        let tree = parser.parse(malformed, None).unwrap();
        assert!(tree.root_node().has_error());
    }

    #[test]
    fn test_locally_recoverable_and_valid_after_damage_fixtures() {
        let mut parser = Parser::new();
        parser.set_language(&language()).unwrap();

        // Damaged line in middle, valid records before and after damage
        let content = "2451545.0 1.0 0.0 0.0\n??? BAD LINE ???\n2451546.0 1.1 0.0 0.0";
        let tree = parser.parse(content, None).unwrap();
        let root = tree.root_node();
        assert!(root.has_error(), "Root should reflect damaged middle line");
        assert!(
            root.child_count() >= 2,
            "Parser should recover and yield valid record nodes"
        );
    }
}
