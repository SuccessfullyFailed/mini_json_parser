pub struct OpenCloseEscapeList {
	sets:&'static [OpenCloseEscape]
}
impl OpenCloseEscapeList {

	/* CONSTRUCTOR METHODS */

	/// Create a new set.
	pub const fn new(sets:&'static [OpenCloseEscape]) -> OpenCloseEscapeList {
		OpenCloseEscapeList {
			sets
		}
	}



	/* USAGE METHODS */

	/// If any of the owned sets match the given source str, returns the full match including tags.
	pub fn match_str<'a>(&self, source:&'a str) -> Option<(&'a str, usize)> {
		for set in self.sets {
			if let Some(set_match) = set.match_str(source) {
				return Some(set_match);
			}
		}
		None
	}
}


pub struct OpenCloseEscape {
	open_tag:&'static str,
	close_tag:&'static str,
	escape_tags:&'static [(&'static str, usize)] // If text matches the string, skip the length of the string + the number.
}
impl OpenCloseEscape {

	/* CONSTRUCTOR METHODS */

	/// Create a new set.
	pub const fn new(open:&'static str, close:&'static str, escapes:&'static [(&'static str, usize)]) -> OpenCloseEscape {
		OpenCloseEscape {
			open_tag: open,
			close_tag: close,
			escape_tags: escapes
		}
	}



	/* USAGE METHODS */

	/// If this sets matches the given source str, returns the full match including tags.
	pub fn match_str<'a>(&self, source:&'a str) -> Option<(&'a str, usize)> {
		let source_len:usize = source.len();
		let open_tag_len:usize = self.open_tag.len();
		let close_tag_len:usize = self.close_tag.len();

		// Try to match open-tag.
		if open_tag_len < source_len && &source[..open_tag_len] == self.open_tag {
			let cursor_max:usize = source_len - close_tag_len + 1;
			let mut cursor:usize = open_tag_len;
			while cursor < cursor_max {

				// Try to match end-tag.
				if &source[cursor..cursor + close_tag_len] == self.close_tag {
					cursor += close_tag_len;
					return Some((&source[..cursor], cursor));
				}

				// Try to match escapes.
				for (escape_tag, escape_skip) in self.escape_tags {
					let escape_tag_len:usize = escape_tag.len();
					let cursor_escape_end:usize = cursor + escape_tag_len;
					if cursor < cursor_escape_end && &source[cursor..cursor + escape_tag_len] == *escape_tag {
						cursor += escape_tag_len + escape_skip - 1; // Remove one for the cursor incrementation at the end of the loop.
					}
				}

				// Increment cursor.
				cursor += 1;
			}
		}

		// No match was found.
		None
	}
}