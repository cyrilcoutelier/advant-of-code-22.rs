use day_14_1::Pos;

mod get_circle_iter {
    use crate::get_pos_array;
    use day_14_1::Pos;

    #[test]
    fn test_0() {
        // Given
        let pos = Pos { x: 2, y: 4 };

        // When
        let result = get_pos_array(pos.get_circle_iter(0));

        // Then
        assert_eq!(format!("{:?}", result), "[(2, 4)]");
    }

    #[test]
    fn test_1() {
        // Given
        let pos = Pos { x: 2, y: 4 };

        // When
        let result = get_pos_array(pos.get_circle_iter(1));

        // Then
        assert_eq!(format!("{:?}", result), "[(2, 5), (1, 4), (3, 4), (2, 3)]");
    }

    #[test]
    fn test_2() {
        // Given
        let pos = Pos { x: 2, y: 4 };

        // When
        let result = get_pos_array(pos.get_circle_iter(2));

        // Then
        assert_eq!(
            format!("{:?}", result),
            "[(2, 6), (1, 5), (3, 5), (0, 4), (4, 4), (1, 3), (3, 3), (2, 2)]"
        );
    }
}

fn get_pos_array<T: Iterator<Item = Pos>>(iter: T) -> Vec<(isize, isize)> {
    iter.map(|pos| (pos.x, pos.y)).collect()
}
