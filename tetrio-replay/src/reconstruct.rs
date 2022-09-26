

enum Action {
    Garbage {
        column: u8,
        height: u8,
    },
    Reposition {
        // TODO: better tetromino position descriptors
    },
    LineClear {
        line: u8,
    }
}