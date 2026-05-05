use burbomath::{NonNeg, Positive, non_neg, positive};

#[test]
fn non_neg_macro_test() {
    assert_eq!(NonNeg::new(1).unwrap(), non_neg!(1));
    assert_eq!(NonNeg::new(1.).unwrap(), non_neg!(1.));
    assert_eq!(NonNeg::new(1.).unwrap(), non_neg!(1_f32));
    assert_eq!(NonNeg::new(1_i32).unwrap(), non_neg!(1_i32));
}

#[test]
fn positive_macro_test() {
    assert_eq!(Positive::new(1).unwrap(), positive!(1));
    assert_eq!(Positive::new(1.).unwrap(), positive!(1.));
    assert_eq!(Positive::new(1.).unwrap(), positive!(1_f32));
    assert_eq!(Positive::new(1_u32).unwrap(), positive!(1_u32));
    assert_eq!(Positive::new(1_i32).unwrap(), positive!(1_i32));
    assert_eq!(Positive::new(3f32).unwrap(), positive!(3f32));
    assert_eq!(Positive::new(3_f32).unwrap(), positive!(3_f32));
}
