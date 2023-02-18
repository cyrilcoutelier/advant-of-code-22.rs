use day_14_1::{get_intersection_disk_row, Pos, Segment};

#[test]
fn test_interaction_size_0_pos_0() {
    // Given
    let center = Pos { x: 2, y: 4 };
    let radius = 0;
    let y = 4;

    // When
    let segment = get_intersection_disk_row(&center, radius, y);

    // Then
    assert_eq!(
        segment,
        Some(Segment {
            start: 2,
            length: 1
        })
    );
}

#[test]
fn test_interaction_size_0_pos_1() {
    // Given
    let center = Pos { x: 2, y: 4 };
    let radius = 0;
    let y = 5;

    // When
    let segment = get_intersection_disk_row(&center, radius, y);

    // Then
    assert_eq!(segment, None);
}

#[test]
fn test_interaction_size_0_pos_m1() {
    // Given
    let center = Pos { x: 2, y: 4 };
    let radius = 0;
    let y = 3;

    // When
    let segment = get_intersection_disk_row(&center, radius, y);

    // Then
    assert_eq!(segment, None);
}

#[test]
fn test_interaction_size_1_pos_0() {
    // Given
    let center = Pos { x: 2, y: 4 };
    let radius = 1;
    let y = 4;

    // When
    let segment = get_intersection_disk_row(&center, radius, y);

    // Then
    assert_eq!(
        segment,
        Some(Segment {
            start: 1,
            length: 3
        })
    );
}

#[test]
fn test_interaction_size_1_pos_1() {
    // Given
    let center = Pos { x: 2, y: 4 };
    let radius = 1;
    let y = 3;

    // When
    let segment = get_intersection_disk_row(&center, radius, y);

    // Then
    assert_eq!(
        segment,
        Some(Segment {
            start: 2,
            length: 1
        })
    );
}

#[test]
fn test_interaction_size_1_pos_m1() {
    // Given
    let center = Pos { x: 2, y: 4 };
    let radius = 1;
    let y = 5;

    // When
    let segment = get_intersection_disk_row(&center, radius, y);

    // Then
    assert_eq!(
        segment,
        Some(Segment {
            start: 2,
            length: 1
        })
    );
}

#[test]
fn test_interaction_size_1_pos_m2() {
    // Given
    let center = Pos { x: 2, y: 4 };
    let radius = 1;
    let y = 6;

    // When
    let segment = get_intersection_disk_row(&center, radius, y);

    // Then
    assert_eq!(segment, None);
}
