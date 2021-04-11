fn main() {
    for pat in 0..4 {
        let block = if pat == 0 { "▄" } else { "█" };

        for color in 0..8 {
            let panepart = if pat == 3 {
                "\u{001b}[1m ▀▀▀▀".to_string()
            } else {
                ["████\u{001b}[1m", block].join("")
            };

            let line = &[
                "\u{001b}[3",
                &color.to_string(),
                "m",
                &panepart,
                "  \u{001b}[0m",
            ]
            .join("");
            print!("{}", line);
        }

        print!("\n")
    }
}
