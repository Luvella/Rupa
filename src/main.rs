fn main() {
	for pat in 0..4 {
		let mut block = "█";

		if pat == 0 {
			block = "▄";
		}

		for color in 0..8 {
			let mut panepart = ["████\u{001b}[1m", block].join("");

			if pat == 3 {
				panepart = "\u{001b}[1m ▀▀▀▀".to_string();
			}

			let line = &["\u{001b}[3", &color.to_string(), "m", &panepart, "  \u{001b}[0m"].join("");
			print!("{}", line);
		}

		print!("\n")
	}
}
