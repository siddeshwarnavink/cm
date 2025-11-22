use super::*;
use ncurses::*;
use std::fs;

pub fn render_preview(win: WINDOW, file_path: &str, highlight_line: usize) {
    wclear(win);
    box_(win, 0, 0);

    let mut max_y = 0;
    let mut max_x = 0;
    getmaxyx(win, &mut max_y, &mut max_x);

    let available_width = (max_x-4).max(0) as usize;

    if let Ok(contents) = fs::read_to_string(file_path) {
        let lines: Vec<&str> = contents.lines().collect();

        // TODO: Choose context size based on available height.
        let context = 10;
        let target = highlight_line.saturating_sub(1);

        let start = target.saturating_sub(context);
        let end = (target+context).min(lines.len().saturating_sub(1));

        let mut pos = 1;

        for i in start..=end {
            let pair = if i+1 == highlight_line {
                UNFOCUSED_CURSOR_PAIR
            } else {
                REGULAR_PAIR
            };

            wattron(win, COLOR_PAIR(pair));

            let mut text = format!("{:>6} {}", i+1, lines[i]);
            if text.len() > available_width {
                text.truncate(available_width);
            }
            mvwaddnstr(win, pos, 2, &text, available_width as i32);

            wattroff(win, COLOR_PAIR(pair));
            pos += 1;
        }
    }

    wrefresh(win);
}

