use crate::{
    cursor::{ColPosition, CursorMode},
    mode::VisualMode,
    movement::Movement,
};

pub fn helix_move_cursor(
    start: usize,
    end: usize,
    count: usize,
    movement: &Movement,
    mut move_offset: impl FnMut(usize, usize, &Movement) -> (usize, Option<ColPosition>),
) -> (CursorMode, Option<ColPosition>) {
    match movement {
        Movement::Offset(_)
        | Movement::WordEndForward
        | Movement::WordForward
        | Movement::WordBackward
        | Movement::NextUnmatched(_)
        | Movement::PreviousUnmatched(_) => {
            let mut new_start = end;
            let mut new_offset = end;
            let mut horiz = Option::<ColPosition>::default();

            let (new_offset_check, _) = move_offset(1, new_offset, movement);
            if new_offset_check == new_offset + 1 {
                // Try going right
                (new_offset, _) = move_offset(1, new_offset, &Movement::Right)
            }

            for _ in 0..count {
                new_start = new_offset;
                (new_offset, horiz) = move_offset(1, new_offset, movement);
            }

            match movement {
                Movement::WordForward => {
                    (new_offset, _) = move_offset(1, new_offset, &Movement::Left);
                }
                Movement::WordBackward => {
                    let (new_start_right, _) = move_offset(1, new_start, &Movement::Right);
                    let (new_start_check, _) =
                        move_offset(1, new_start_right, &Movement::WordBackward);
                    if new_start == new_start_check {
                        (new_start, _) = move_offset(1, new_start, &Movement::Left);
                    }
                }
                _ => {}
            }

            (
                CursorMode::Visual {
                    start: new_start,
                    end: new_offset,
                    mode: VisualMode::HelixNormal,
                },
                horiz,
            )
        }
        _ => {
            let (new_offset, horiz) = move_offset(count, end, movement);

            (
                CursorMode::Visual {
                    start: new_offset,
                    end: new_offset,
                    mode: VisualMode::HelixNormal,
                },
                horiz,
            )
        }
    }
}
