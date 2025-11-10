pub type PatternCoords = &'static [(usize, usize)];
pub type Pattern = &'static (&'static str, PatternCoords);

#[derive(Debug)]
pub struct EditorPattern {
    id: usize,
    name: &'static str,
    coords: PatternCoords,
}

impl EditorPattern {
    pub fn new(id: usize, name: &'static str, coords: PatternCoords) -> EditorPattern {
        EditorPattern { id, name, coords }
    }

    // Getter for id
    pub fn id(&self) -> usize {
        self.id
    }

    // Getter for name
    pub fn name(&self) -> &'static str {
        self.name
    }

    // Getter for coords
    pub fn coords(&self) -> PatternCoords {
        self.coords
    }
}

// TODO: Make macro for this
pub static GLIDER_GUN: EditorPattern = EditorPattern {
    id: 0,
    name: "glider gun",
    coords: &[
        (1, 25),
        (2, 23),
        (2, 25),
        (3, 13),
        (3, 14),
        (3, 21),
        (3, 22),
        (3, 35),
        (3, 36),
        (4, 12),
        (4, 16),
        (4, 21),
        (4, 22),
        (4, 35),
        (4, 36),
        (5, 1),
        (5, 2),
        (5, 11),
        (5, 17),
        (5, 21),
        (5, 22),
        (6, 1),
        (6, 2),
        (6, 11),
        (6, 15),
        (6, 17),
        (6, 18),
        (6, 23),
        (6, 25),
        (7, 11),
        (7, 17),
        (7, 25),
        (8, 12),
        (8, 16),
        (9, 13),
        (9, 14),
    ],
};

pub static GLIDER: EditorPattern = EditorPattern {
    id: 1,
    name: "glider",
    coords: &[(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)],
};

pub static PULSAR: EditorPattern = EditorPattern {
    id: 2,
    name: "pulsar",
    coords: &[
        (2, 4),
        (2, 5),
        (2, 6),
        (2, 10),
        (2, 11),
        (2, 12),
        (4, 2),
        (4, 7),
        (4, 9),
        (4, 14),
        (5, 2),
        (5, 7),
        (5, 9),
        (5, 14),
        (6, 2),
        (6, 7),
        (6, 9),
        (6, 14),
        (7, 4),
        (7, 5),
        (7, 6),
        (7, 10),
        (7, 11),
        (7, 12),
        (9, 4),
        (9, 5),
        (9, 6),
        (9, 10),
        (9, 11),
        (9, 12),
        (10, 2),
        (10, 7),
        (10, 9),
        (10, 14),
        (11, 2),
        (11, 7),
        (11, 9),
        (11, 14),
        (12, 2),
        (12, 7),
        (12, 9),
        (12, 14),
        (14, 4),
        (14, 5),
        (14, 6),
        (14, 10),
        (14, 11),
        (14, 12),
    ],
};

pub static PENTADECATHLON: EditorPattern = EditorPattern {
    id: 3,
    name: "pentadecathlon",
    coords: &[
        (0, 1),
        (1, 1),
        (2, 0),
        (2, 2),
        (3, 1),
        (4, 1),
        (5, 1),
        (6, 0),
        (6, 2),
        (7, 1),
    ],
};

pub static BLINKER: EditorPattern = EditorPattern {
    id: 4,
    name: "blinker",
    coords: &[(0, 0), (0, 1), (0, 2)],
};

pub static TOAD: EditorPattern = EditorPattern {
    id: 5,
    name: "toad",
    coords: &[(1, 1), (1, 2), (1, 3), (2, 0), (2, 1), (2, 2)],
};

pub static BEACON: EditorPattern = EditorPattern {
    id: 6,
    name: "beacon",
    coords: &[
        (1, 1),
        (1, 2),
        (2, 1),
        (2, 2),
        (3, 3),
        (3, 4),
        (4, 3),
        (4, 4),
    ],
};

#[derive(Debug, Default)]
enum EditorState {
    PatternSelected {
        pattern: &'static EditorPattern,
    },
    #[default]
    Idle,
}

#[derive(Debug)]
pub struct GameEditor {
    state: EditorState,
    editor_patterns: Vec<&'static EditorPattern>,
}

impl GameEditor {
    pub fn new() -> GameEditor {
        let editor_patterns = vec![
            &GLIDER_GUN,
            &GLIDER,
            &PULSAR,
            &PENTADECATHLON,
            &BLINKER,
            &TOAD,
            &BEACON,
        ];

        GameEditor {
            state: EditorState::default(),
            editor_patterns,
        }
    }

    pub fn get_editor_patterns(&self) -> &Vec<&'static EditorPattern> {
        &self.editor_patterns
    }

    pub fn pattern_selected(&self) -> Option<&EditorPattern> {
        match &self.state {
            EditorState::PatternSelected { pattern } => Some(pattern),
            _ => None,
        }
    }

    pub fn select_pattern(&mut self, id: usize) -> Result<(), &'static str> {
        if let Some(pattern) = self.editor_patterns.iter().find(|p| p.id == id) {
            self.state = EditorState::PatternSelected { pattern };
            Ok(())
        } else {
            Err("Pattern not found")
        }
    }

    pub fn unselect_pattern(&mut self) {
        self.state = EditorState::Idle;
    }

    pub fn patterns(&self) -> &Vec<&EditorPattern> {
        &self.editor_patterns
    }
}
