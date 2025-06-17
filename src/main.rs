use std::{borrow::Cow, cmp::Ordering, path::PathBuf};

const FILE_EXTENSIONS: &[&str] = &["dm", "dmf"];

fn is_file(segment: &str) -> bool {
	FILE_EXTENSIONS.iter().any(|ext| segment.ends_with(ext))
}

fn strip_include(line: &str) -> Cow<'_, str> {
	let line = line.trim();
	let line = line.strip_prefix("#include \"").unwrap_or(line);
	let line = line.strip_suffix('"').unwrap_or(line);
	if line.chars().any(|c| c.is_alphabetic() && !c.is_lowercase()) {
		Cow::Owned(line.to_lowercase())
	} else {
		Cow::Borrowed(line)
	}
}

fn suffix(path: &str) -> &str {
	path.rsplit('.').next().unwrap_or_default()
}

fn compare_lines(a: &str, b: &str) -> Ordering {
	let a = strip_include(a);
	let b = strip_include(b);

	let a_suffix = suffix(&a);
	let b_suffix = suffix(&b);

	let a_segments = a.split('\\');
	let b_segments = b.split('\\');

	for (a, b) in a_segments.zip(b_segments) {
		// files always come before directories
		match (is_file(a), is_file(b)) {
			(true, false) => return Ordering::Less,
			(false, true) => return Ordering::Greater,
			_ => (),
		}

		if a == b {
			continue;
		} else if a_suffix != b_suffix {
			return a_suffix.len().cmp(&b_suffix.len());
		} else {
			return a.cmp(b);
		};
	}

	Ordering::Equal
}

struct SplitDme<'a> {
	before: Vec<&'a str>,
	includes: Vec<&'a str>,
	after: Vec<&'a str>,
}

fn dme_includes<'a>(input: &'a str) -> SplitDme<'a> {
	enum State {
		Before,
		Includes,
		After,
	}

	let mut before = Vec::new();
	let mut includes = Vec::new();
	let mut after = Vec::new();

	let mut state = State::Before;
	for line in input.lines() {
		// skip merge conflict lines, as we'll effectively automatically solve any
		// conflicts when we filter, sort, and dedup.
		if line.starts_with("<<<<<<<") || line.starts_with(">>>>>>>") || line.starts_with("=======")
		{
			continue;
		}
		match state {
			State::Before => {
				before.push(line);
				if line == "// BEGIN_INCLUDE" {
					state = State::Includes;
				}
			}
			State::Includes => {
				if line == "// END_INCLUDE" {
					after.push(line);
					state = State::After;
				} else {
					includes.push(line);
				}
			}
			State::After => after.push(line),
		}
	}

	SplitDme {
		before,
		includes,
		after,
	}
}

#[cfg(feature = "diff")]
fn get_diff(before: impl AsRef<str>, after: impl AsRef<str>) -> Option<String> {
	use imara_diff::{Algorithm, BasicLineDiffPrinter, Diff, InternedInput, UnifiedDiffConfig};

	let before = before.as_ref();
	let after = after.as_ref();

	if before == after {
		return None;
	}

	let input = InternedInput::new(before, after);
	let mut diff = Diff::compute(Algorithm::Histogram, &input);
	diff.postprocess_lines(&input);
	Some(
		diff.unified_diff(
			&BasicLineDiffPrinter(&input.interner),
			UnifiedDiffConfig::default(),
			&input,
		)
		.to_string(),
	)
}

#[cfg(not(feature = "diff"))]
fn get_diff(before: impl AsRef<str>, after: impl AsRef<str>) -> Option<String> {
	if before.as_ref() != after.as_ref() {
		Some(
			"dme file was changed/fixed (enable 'diff' feature to output a diff to stdout)"
				.to_owned(),
		)
	} else {
		None
	}
}

fn string_to_pathbuf(mut path: String) -> PathBuf {
	if cfg!(target_os = "wasi") {
		path = path.replace('\\', "/");
	}
	PathBuf::from(path)
}

fn main() {
	let mut args = std::env::args().skip(1);
	let path = match args.next() {
		Some(path) => string_to_pathbuf(path),
		None => {
			eprintln!("expected syntax: dme-sorter thing.dme [output.dme]");
			std::process::exit(1);
		}
	};
	if !path.is_file() {
		eprintln!("file not found: {}", path.display());
		std::process::exit(1);
	}
	let output_path = args
		.next()
		.map(|path| Cow::Owned(string_to_pathbuf(path)))
		.unwrap_or(Cow::Borrowed(path.as_path()));
	let base_dir = path.parent().expect("path had no parent directory");
	let file = std::fs::read_to_string(&path).expect("failed to read file");
	let SplitDme {
		before,
		mut includes,
		after,
	} = dme_includes(&file);
	includes.retain(|file| {
		let file = strip_include(file);
		if !base_dir.join(file.as_ref()).exists() {
			eprintln!("{file} did not exist, removing");
			false
		} else {
			true
		}
	});
	includes.sort_by(|a, b| compare_lines(a, b));
	includes.dedup();
	let mut reassembled = before
		.into_iter()
		.chain(includes)
		.chain(after)
		.collect::<Vec<&str>>()
		.join("\n");
	reassembled.push('\n');

	if let Some(diff) = get_diff(file, &reassembled) {
		println!("--- diff ---\n{diff}\n---");
	} else {
		println!("input and output identical :)");
	}

	std::fs::write(output_path, reassembled).expect("failed to write reassembled file");
}
