use crate::utils::BiRange;

#[test]
fn test_birange() {
    assert_eq!(BiRange::new(0,0).collect::<Vec<usize>>(), vec![]);
    assert_eq!(BiRange::new(0,1).collect::<Vec<usize>>(), vec![]);
    assert_eq!(BiRange::new(1,0).collect::<Vec<usize>>(), vec![]);
    assert_eq!(BiRange::new(0,2).collect::<Vec<usize>>(), vec![1]);
    assert_eq!(BiRange::new(2,0).collect::<Vec<usize>>(), vec![1]);
    assert_eq!(BiRange::new(3,8).collect::<Vec<usize>>(), vec![4, 5, 6, 7]);
    assert_eq!(BiRange::new(0,8).collect::<Vec<usize>>(), vec![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(BiRange::new(8,0).collect::<Vec<usize>>(), vec![7, 6, 5, 4, 3, 2, 1]);
    assert_eq!(BiRange::new(8,3).collect::<Vec<usize>>(), vec![7, 6, 5, 4]);
}
