use repeto;


#[test]
pub fn integration_test() {
    repeto::predict::run(b"AAATTT", 1, 1);
}