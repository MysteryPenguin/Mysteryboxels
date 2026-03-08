use crate::Game;

pub fn gravity([x, y]: [usize; 2], game: &mut Game) {
    let Some(cell) = game.cells.get_mut(x, y) else {
        return;
    };
}
