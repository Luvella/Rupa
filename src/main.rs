fn main() {
	for pat in 0..4 {
		let mut block = "█";

		if pat == 0 {
			block = "▄";
		}

		for color in 0..8 {
			let mut panepart = format!("████\u{001b}[1m{}", block);
			if pat == 3 {
				panepart = String::from("\u{001b}[1m ▀▀▀▀");
			}
			print!("\u{001b}[3{}m{}  \u{001b}[0m", color, panepart);
		}

		print!("\n")
	}
}
