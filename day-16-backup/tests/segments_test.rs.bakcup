mod add_segment {
    use day_14_1::{Segment, Segments};

    #[test]
    fn test_simple() {
        // Given
        let mut segments = Segments::new();

        // When
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // Then
        assert_eq!(format!("{:?}", segments.map), "{3: 4}");
    }

    #[test]
    fn test_overlap_inside() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.add_segment(Segment {
            start: 4,
            length: 1,
        });

        // Then
        assert_eq!(format!("{:?}", segments.map), "{3: 4}");
    }

    #[test]
    fn test_overlap_inside_begining() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.add_segment(Segment {
            start: 3,
            length: 1,
        });

        // Then
        assert_eq!(format!("{:?}", segments.map), "{3: 4}");
    }

    #[test]
    fn test_overlap_inside_end() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.add_segment(Segment {
            start: 6,
            length: 1,
        });

        // Then
        assert_eq!(format!("{:?}", segments.map), "{3: 4}");
    }

    #[test]
    fn test_overlap_full() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // Then
        assert_eq!(format!("{:?}", segments.map), "{3: 4}");
    }

    #[test]
    fn test_overflow_left_edge() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.add_segment(Segment {
            start: 2,
            length: 1,
        });

        // Then
        assert_eq!(format!("{:?}", segments.map), "{2: 5}");
    }

    #[test]
    fn test_overflow_left() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.add_segment(Segment {
            start: 2,
            length: 2,
        });

        // Then
        assert_eq!(format!("{:?}", segments.map), "{2: 5}");
    }

    #[test]
    fn test_overflow_right_edge() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.add_segment(Segment {
            start: 7,
            length: 1,
        });

        // Then
        assert_eq!(format!("{:?}", segments.map), "{3: 5}");
    }

    #[test]
    fn test_overflow_right() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.add_segment(Segment {
            start: 6,
            length: 2,
        });

        // Then
        assert_eq!(format!("{:?}", segments.map), "{3: 5}");
    }

    #[test]
    fn test_overflow_both_sides() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.add_segment(Segment {
            start: 2,
            length: 6,
        });

        // Then
        assert_eq!(format!("{:?}", segments.map), "{2: 6}");
    }

    #[test]
    fn test_before() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.add_segment(Segment {
            start: 0,
            length: 2,
        });

        // Then
        assert_eq!(format!("{:?}", segments.map), "{0: 2, 3: 4}");
    }

    #[test]
    fn test_after() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.add_segment(Segment {
            start: 9,
            length: 2,
        });

        // Then
        assert_eq!(format!("{:?}", segments.map), "{3: 4, 9: 2}");
    }
}

mod remove_dot {
    use day_14_1::{Segment, Segments};

    #[test]
    fn test_before() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.remove_dot(2);

        // Then
        assert_eq!(format!("{:?}", segments.map), "{3: 4}");
    }

    #[test]
    fn test_after() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.remove_dot(7);

        // Then
        assert_eq!(format!("{:?}", segments.map), "{3: 4}");
    }

    #[test]
    fn test_start() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.remove_dot(3);

        // Then
        assert_eq!(format!("{:?}", segments.map), "{4: 3}");
    }

    #[test]
    fn test_end() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.remove_dot(6);

        // Then
        assert_eq!(format!("{:?}", segments.map), "{3: 3}");
    }

    #[test]
    fn test_middle() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.remove_dot(5);

        // Then
        assert_eq!(format!("{:?}", segments.map), "{3: 2, 6: 1}");
    }

    #[test]
    fn test_middle_2() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 4,
        });

        // When
        segments.remove_dot(4);

        // Then
        assert_eq!(format!("{:?}", segments.map), "{3: 1, 5: 2}");
    }

    #[test]
    fn test_on_dot() {
        // Given
        let mut segments = Segments::new();
        segments.add_segment(Segment {
            start: 3,
            length: 1,
        });

        // When
        segments.remove_dot(3);

        // Then
        assert_eq!(format!("{:?}", segments.map), "{}");
    }
}
