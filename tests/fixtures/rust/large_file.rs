// Large fixture file for testing metrics on a realistic Rust module.
// Contains 20+ functions with varying cognitive complexity levels.

use std::collections::HashMap;
use std::io::{self, Read, Write};

// ---------------------------------------------------------------------------
// Simple functions (cognitive complexity = 0)
// ---------------------------------------------------------------------------

/// Returns the sum of two integers.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Returns a default greeting string.
fn greeting() -> &'static str {
    "hello, world"
}

/// Wraps a value in Some.
fn wrap_option(val: u64) -> Option<u64> {
    Some(val)
}

/// Squares a floating-point number.
fn square(x: f64) -> f64 {
    x * x
}

/// Identity function for a string slice.
fn identity(s: &str) -> &str {
    s
}

/// Returns the length of a slice.
fn slice_len(data: &[u8]) -> usize {
    data.len()
}

/// Creates an empty HashMap.
fn empty_map() -> HashMap<String, i32> {
    HashMap::new()
}

// ---------------------------------------------------------------------------
// Low complexity functions (cognitive 1-3)
// ---------------------------------------------------------------------------

/// Returns the absolute value of an integer.
fn absolute(x: i32) -> i32 {
    if x < 0 {          // +1
        -x
    } else {
        x
    }
}

/// Clamps a value to a range.
fn clamp(val: i32, lo: i32, hi: i32) -> i32 {
    if val < lo {        // +1
        lo
    } else if val > hi { // +1
        hi
    } else {
        val
    }
}

/// Checks if a number is even.
fn is_even(n: i32) -> bool {
    if n % 2 == 0 {      // +1
        true
    } else {
        false
    }
}

/// Returns the maximum of three values.
fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    let mut max = a;
    if b > max {          // +1
        max = b;
    }
    if c > max {          // +1
        max = c;
    }
    max
}

/// Finds the first positive number in a slice.
fn first_positive(nums: &[i32]) -> Option<i32> {
    for n in nums {       // +1
        if *n > 0 {       // +2 (nesting)
            return Some(*n);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Medium complexity functions (cognitive 3-8)
// ---------------------------------------------------------------------------

/// Categorizes a temperature reading.
fn temperature_category(temp: f64) -> &'static str {
    if temp < -20.0 {           // +1
        "extreme cold"
    } else if temp < 0.0 {     // +1
        "freezing"
    } else if temp < 15.0 {    // +1
        "cold"
    } else if temp < 25.0 {    // +1
        "comfortable"
    } else if temp < 35.0 {    // +1
        "warm"
    } else {
        "hot"
    }
}

/// Counts vowels in a string using a match.
fn count_vowels(s: &str) -> usize {
    let mut count = 0;
    for ch in s.chars() {       // +1
        match ch {              // +2 (nesting)
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            'A' | 'E' | 'I' | 'O' | 'U' => count += 1,
            _ => {}
        }
    }
    count
}

/// Sums only positive even numbers from a slice.
fn sum_positive_evens(nums: &[i32]) -> i32 {
    let mut total = 0;
    for n in nums {                     // +1
        if *n > 0 && *n % 2 == 0 {     // +2 (nesting) +1 (&&)
            total += *n;
        }
    }
    total
}

/// Simple FizzBuzz for a single number.
fn fizzbuzz(n: u32) -> String {
    if n % 15 == 0 {         // +1
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {   // +1
        "Fizz".to_string()
    } else if n % 5 == 0 {   // +1
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}

/// Generates FizzBuzz for a range.
fn fizzbuzz_range(start: u32, end: u32) -> Vec<String> {
    let mut results = Vec::new();
    for i in start..=end {                // +1
        if i % 15 == 0 {                 // +2 (nesting)
            results.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {           // +2 (nesting)
            results.push("Fizz".to_string());
        } else if i % 5 == 0 {           // +2 (nesting)
            results.push("Buzz".to_string());
        } else {
            results.push(i.to_string());
        }
    }
    results
}

/// Describes an HTTP status code.
fn describe_status(code: u16) -> &'static str {
    match code {                     // +1
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _ => "Unknown",
    }
}

/// Filters and transforms a list: keeps positive, doubles them.
fn filter_and_double(nums: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &n in nums {               // +1
        if n > 0 {                 // +2 (nesting)
            result.push(n * 2);
        }
    }
    result
}

/// Flattens a 2D vector.
fn flatten_matrix(matrix: &[Vec<i32>]) -> Vec<i32> {
    let mut flat = Vec::new();
    for row in matrix {           // +1
        for val in row {          // +2 (nesting)
            flat.push(*val);
        }
    }
    flat
}

// ---------------------------------------------------------------------------
// Higher complexity functions (cognitive 10+)
// ---------------------------------------------------------------------------

/// Processes a list of user records. Complex validation and categorization.
fn process_user_records(
    records: &[(String, i32, bool)],
    min_age: i32,
    require_active: bool,
) -> Vec<String> {
    let mut output = Vec::new();

    for (name, age, active) in records {             // +1
        if require_active && !active {               // +2 (nesting) +1 (&&)
            continue;
        }

        if *age < min_age {                          // +2 (nesting)
            continue;
        }

        if name.is_empty() {                         // +2 (nesting)
            continue;
        }

        let category = if *age < 18 {               // +2 (nesting)
            "minor"
        } else if *age < 65 {                       // +2 (nesting)
            "adult"
        } else {
            "senior"
        };

        let status = if *active { "active" } else { "inactive" }; // +2 (nesting)

        output.push(format!("{}: {} ({})", name, category, status));
    }

    output
}

/// Analyzes text: counts words, lines, and character frequency.
fn analyze_text(text: &str) -> (usize, usize, HashMap<char, usize>) {
    let mut word_count = 0;
    let mut line_count = 0;
    let mut freq: HashMap<char, usize> = HashMap::new();
    let mut in_word = false;

    for ch in text.chars() {                  // +1
        *freq.entry(ch).or_insert(0) += 1;

        if ch == '\n' {                       // +2 (nesting)
            line_count += 1;
            if in_word {                      // +3 (nesting)
                word_count += 1;
                in_word = false;
            }
        } else if ch.is_whitespace() {        // +2 (nesting)
            if in_word {                      // +3 (nesting)
                word_count += 1;
                in_word = false;
            }
        } else {
            in_word = true;
        }
    }

    if in_word {                              // +1
        word_count += 1;
    }

    if !text.is_empty() {                     // +1
        line_count += 1;
    }

    (word_count, line_count, freq)
}

/// Validates a password with multiple rules.
fn validate_password(password: &str) -> Result<(), Vec<&'static str>> {
    let mut errors = Vec::new();

    if password.len() < 8 {                               // +1
        errors.push("too short");
    }

    if password.len() > 128 {                             // +1
        errors.push("too long");
    }

    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;
    let mut has_special = false;

    for ch in password.chars() {                          // +1
        if ch.is_uppercase() {                            // +2 (nesting)
            has_upper = true;
        } else if ch.is_lowercase() {                     // +2 (nesting)
            has_lower = true;
        } else if ch.is_ascii_digit() {                   // +2 (nesting)
            has_digit = true;
        } else {
            has_special = true;
        }
    }

    if !has_upper {                                       // +1
        errors.push("missing uppercase letter");
    }
    if !has_lower {                                       // +1
        errors.push("missing lowercase letter");
    }
    if !has_digit {                                       // +1
        errors.push("missing digit");
    }
    if !has_special {                                     // +1
        errors.push("missing special character");
    }

    if errors.is_empty() {                                // +1
        Ok(())
    } else {
        Err(errors)
    }
}

/// Sorts with bubble sort, counting swaps.
fn bubble_sort_counted(data: &mut Vec<i32>) -> usize {
    let mut swaps = 0;
    let n = data.len();

    if n <= 1 {                          // +1
        return 0;
    }

    for i in 0..n {                      // +1
        let mut swapped = false;
        for j in 0..n - 1 - i {         // +2 (nesting)
            if data[j] > data[j + 1] {  // +3 (nesting)
                data.swap(j, j + 1);
                swaps += 1;
                swapped = true;
            }
        }
        if !swapped {                    // +2 (nesting)
            break;
        }
    }

    swaps
}

/// Evaluates a simple postfix expression.
fn eval_postfix(tokens: &[&str]) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {                              // +1
        match *token {                                 // +2 (nesting)
            "+" | "-" | "*" | "/" => {
                if stack.len() < 2 {                   // +3 (nesting)
                    return Err("insufficient operands".to_string());
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match *token {            // +3 (nesting)
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0.0 {                  // +4 (nesting)
                            return Err("division by zero".to_string());
                        }
                        a / b
                    }
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            num_str => {
                match num_str.parse::<f64>() {         // +3 (nesting)
                    Ok(val) => stack.push(val),
                    Err(_) => return Err(format!("invalid token: {}", num_str)),
                }
            }
        }
    }

    if stack.len() == 1 {                              // +1
        Ok(stack[0])
    } else {
        Err("invalid expression".to_string())
    }
}

/// A complex state machine parser for simple CSV-like data.
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut prev_was_quote = false;

    for ch in line.chars() {                   // +1
        if in_quotes {                         // +2 (nesting)
            if ch == '"' {                     // +3 (nesting)
                if prev_was_quote {            // +4 (nesting)
                    current.push('"');
                    prev_was_quote = false;
                } else {
                    prev_was_quote = true;
                }
            } else {
                if prev_was_quote {            // +4 (nesting)
                    in_quotes = false;
                    prev_was_quote = false;
                    if ch == ',' {             // +5 (nesting)
                        fields.push(current.clone());
                        current.clear();
                    }
                } else {
                    current.push(ch);
                }
            }
        } else {
            if ch == '"' && current.is_empty() { // +3 (nesting) +1 (&&)
                in_quotes = true;
            } else if ch == ',' {              // +3 (nesting)
                fields.push(current.clone());
                current.clear();
            } else {
                current.push(ch);
            }
        }
    }

    if prev_was_quote || !current.is_empty() { // +1 +1 (||)
        fields.push(current);
    }

    fields
}

/// Merges two sorted slices into a sorted vector.
fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {       // +1 +1 (&&)
        if a[i] <= b[j] {                     // +2 (nesting)
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }

    while i < a.len() {                       // +1
        result.push(a[i]);
        i += 1;
    }

    while j < b.len() {                       // +1
        result.push(b[j]);
        j += 1;
    }

    result
}

/// Groups items by a key derived from a closure, with filtering.
fn group_and_filter(
    items: &[(&str, i32)],
    threshold: i32,
) -> HashMap<char, Vec<(&str, i32)>> {
    let mut groups: HashMap<char, Vec<(&str, i32)>> = HashMap::new();

    for &(name, value) in items {              // +1
        if value < threshold {                 // +2 (nesting)
            continue;
        }

        if name.is_empty() {                  // +2 (nesting)
            continue;
        }

        let key = match name.chars().next() {  // +2 (nesting)
            Some(ch) => {
                if ch.is_ascii_alphabetic() {  // +3 (nesting)
                    ch.to_ascii_uppercase()
                } else {
                    '#'
                }
            }
            None => continue,
        };

        groups.entry(key).or_insert_with(Vec::new).push((name, value));
    }

    groups
}

// ---------------------------------------------------------------------------
// Functions using closures
// ---------------------------------------------------------------------------

/// Applies a transformation via closure and collects results.
fn transform_with<F>(data: &[i32], predicate: F) -> Vec<i32>
where
    F: Fn(i32) -> Option<i32>,
{
    let mut results = Vec::new();
    for &item in data {                        // +1
        if let Some(val) = predicate(item) {   // +2 (nesting)
            results.push(val);
        }
    }
    results
}

/// Demonstrates higher-order function usage with closures.
fn apply_pipeline(input: &[f64]) -> Vec<f64> {
    let scale = |x: f64| -> f64 { x * 2.0 };
    let offset = |x: f64| -> f64 { x + 10.0 };
    let clamp_val = |x: f64| -> f64 {
        if x < 0.0 {          // +1
            0.0
        } else if x > 100.0 { // +1
            100.0
        } else {
            x
        }
    };

    let mut result = Vec::with_capacity(input.len());
    for &val in input {                  // +1
        let v = clamp_val(offset(scale(val)));
        result.push(v);
    }
    result
}

/// Finds items matching a complex predicate built from closures.
fn find_matching_items(
    items: &[(String, u32, bool)],
    name_contains: &str,
    min_score: u32,
    must_be_active: bool,
) -> Vec<&(String, u32, bool)> {
    let name_filter = |item: &(String, u32, bool)| -> bool {
        if name_contains.is_empty() {       // +1
            true
        } else {
            item.0.contains(name_contains)
        }
    };

    let score_filter = |item: &(String, u32, bool)| -> bool {
        item.1 >= min_score
    };

    let active_filter = |item: &(String, u32, bool)| -> bool {
        if must_be_active {                 // +1
            item.2
        } else {
            true
        }
    };

    let mut result = Vec::new();
    for item in items {                     // +1
        if name_filter(item) && score_filter(item) && active_filter(item) { // +2 (nesting) +2 (&&)
            result.push(item);
        }
    }
    result
}

// ---------------------------------------------------------------------------
// Complex function with deep nesting and boolean operators
// ---------------------------------------------------------------------------

/// Processes a grid to find connected regions above a threshold.
fn find_regions(
    grid: &[Vec<f64>],
    threshold: f64,
    min_region_size: usize,
) -> Vec<Vec<(usize, usize)>> {
    let rows = grid.len();
    if rows == 0 {                                      // +1
        return Vec::new();
    }
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions: Vec<Vec<(usize, usize)>> = Vec::new();

    for r in 0..rows {                                  // +1
        for c in 0..cols {                              // +2 (nesting)
            if visited[r][c] {                          // +3 (nesting)
                continue;
            }

            if grid[r][c] < threshold {                 // +3 (nesting)
                visited[r][c] = true;
                continue;
            }

            // BFS to find connected region
            let mut region = Vec::new();
            let mut queue = vec![(r, c)];
            visited[r][c] = true;

            while let Some((cr, cc)) = queue.pop() {   // +3 (nesting)
                region.push((cr, cc));

                // Check four neighbors
                let neighbors: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for &(dr, dc) in &neighbors {          // +4 (nesting)
                    let nr = cr as isize + dr;
                    let nc = cc as isize + dc;

                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize { // +5 (nesting) +3 (&&)
                        let nr = nr as usize;
                        let nc = nc as usize;

                        if !visited[nr][nc] && grid[nr][nc] >= threshold { // +6 (nesting) +1 (&&)
                            visited[nr][nc] = true;
                            queue.push((nr, nc));
                        }
                    }
                }
            }

            if region.len() >= min_region_size {       // +3 (nesting)
                regions.push(region);
            }
        }
    }

    regions
}

/// Parses a simplified version string like "1.2.3-beta.4+build.567".
fn parse_version(input: &str) -> Result<(u32, u32, u32, Option<String>, Option<String>), String> {
    let mut main_part = input;
    let mut build_meta = None;
    let mut pre_release = None;

    // Split off build metadata
    if let Some(pos) = input.find('+') {                // +1
        build_meta = Some(input[pos + 1..].to_string());
        main_part = &input[..pos];
    }

    // Split off pre-release
    if let Some(pos) = main_part.find('-') {            // +1
        pre_release = Some(main_part[pos + 1..].to_string());
        main_part = &main_part[..pos];
    }

    let parts: Vec<&str> = main_part.split('.').collect();

    if parts.len() != 3 {                               // +1
        return Err("expected exactly three version components".to_string());
    }

    let major = parts[0].parse::<u32>().map_err(|_| "invalid major version".to_string());
    let minor = parts[1].parse::<u32>().map_err(|_| "invalid minor version".to_string());
    let patch = parts[2].parse::<u32>().map_err(|_| "invalid patch version".to_string());

    match (major, minor, patch) {                        // +1
        (Ok(ma), Ok(mi), Ok(pa)) => {
            if ma > 999 || mi > 999 || pa > 999 {       // +2 (nesting) +2 (||)
                return Err("version component too large".to_string());
            }
            Ok((ma, mi, pa, pre_release, build_meta))
        }
        _ => Err("failed to parse version components".to_string()),
    }
}

// ---------------------------------------------------------------------------
// Struct with methods to add more variety
// ---------------------------------------------------------------------------

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    /// Creates a new matrix filled with zeros.
    fn zeros(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }

    /// Matrix multiplication with dimension checks.
    fn multiply(&self, other: &Matrix) -> Result<Matrix, String> {
        if self.cols != other.rows {                      // +1
            return Err(format!(
                "dimension mismatch: {}x{} * {}x{}",
                self.rows, self.cols, other.rows, other.cols
            ));
        }

        let mut result = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {                          // +1
            for j in 0..other.cols {                     // +2 (nesting)
                let mut sum = 0.0;
                for k in 0..self.cols {                  // +3 (nesting)
                    sum += self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }

        Ok(result)
    }

    /// Finds the maximum value and its position.
    fn max_element(&self) -> Option<(f64, usize, usize)> {
        if self.rows == 0 || self.cols == 0 {            // +1 +1 (||)
            return None;
        }

        let mut max_val = self.data[0][0];
        let mut max_r = 0;
        let mut max_c = 0;

        for r in 0..self.rows {                          // +1
            for c in 0..self.cols {                      // +2 (nesting)
                if self.data[r][c] > max_val {           // +3 (nesting)
                    max_val = self.data[r][c];
                    max_r = r;
                    max_c = c;
                }
            }
        }

        Some((max_val, max_r, max_c))
    }

    /// Transposes the matrix.
    fn transpose(&self) -> Matrix {
        let mut result = Matrix::zeros(self.cols, self.rows);
        for r in 0..self.rows {                          // +1
            for c in 0..self.cols {                      // +2 (nesting)
                result.data[c][r] = self.data[r][c];
            }
        }
        result
    }
}

// ---------------------------------------------------------------------------
// Enum with complex match arms
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Command {
    Help,
    Version,
    Run { script: String, args: Vec<String> },
    Config { key: String, value: Option<String> },
    Unknown(String),
}

/// Parses a command from string tokens. Complex matching logic.
fn parse_command(tokens: &[String]) -> Result<Command, String> {
    if tokens.is_empty() {                                 // +1
        return Err("no command provided".to_string());
    }

    match tokens[0].as_str() {                             // +1
        "help" | "--help" | "-h" => Ok(Command::Help),
        "version" | "--version" | "-v" => Ok(Command::Version),
        "run" => {
            if tokens.len() < 2 {                          // +2 (nesting)
                return Err("run requires a script name".to_string());
            }
            let script = tokens[1].clone();
            let args = if tokens.len() > 2 {               // +2 (nesting)
                tokens[2..].to_vec()
            } else {
                Vec::new()
            };
            Ok(Command::Run { script, args })
        }
        "config" => {
            if tokens.len() < 2 {                          // +2 (nesting)
                return Err("config requires a key".to_string());
            }
            let key = tokens[1].clone();
            let value = if tokens.len() > 2 {              // +2 (nesting)
                Some(tokens[2].clone())
            } else {
                None
            };
            Ok(Command::Config { key, value })
        }
        other => Ok(Command::Unknown(other.to_string())),
    }
}

/// Executes a parsed command, producing output text.
fn execute_command(cmd: &Command, verbose: bool) -> String {
    match cmd {                                             // +1
        Command::Help => {
            let mut text = "Available commands:\n".to_string();
            text.push_str("  help      Show this help\n");
            text.push_str("  version   Show version\n");
            text.push_str("  run       Run a script\n");
            text.push_str("  config    Get/set config\n");
            text
        }
        Command::Version => "arborist v0.1.0".to_string(),
        Command::Run { script, args } => {
            let mut output = format!("Running script: {}\n", script);
            if !args.is_empty() {                           // +2 (nesting)
                if verbose {                                // +3 (nesting)
                    for (i, arg) in args.iter().enumerate() { // +4 (nesting)
                        output.push_str(&format!("  arg[{}] = {}\n", i, arg));
                    }
                } else {
                    output.push_str(&format!("  with {} args\n", args.len()));
                }
            }
            output
        }
        Command::Config { key, value } => {
            match value {                                   // +2 (nesting)
                Some(v) => format!("Set {} = {}", key, v),
                None => format!("Get {}", key),
            }
        }
        Command::Unknown(name) => {
            if verbose {                                    // +2 (nesting)
                format!("Unknown command '{}'. Try 'help'.", name)
            } else {
                format!("Unknown: {}", name)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Additional utility functions to reach 500+ lines
// ---------------------------------------------------------------------------

/// Runs a simple binary search.
fn binary_search(sorted: &[i32], target: i32) -> Option<usize> {
    let mut lo: isize = 0;
    let mut hi: isize = sorted.len() as isize - 1;

    while lo <= hi {                                // +1
        let mid = ((lo + hi) / 2) as usize;
        if sorted[mid] == target {                  // +2 (nesting)
            return Some(mid);
        } else if sorted[mid] < target {            // +2 (nesting)
            lo = mid as isize + 1;
        } else {
            hi = mid as isize - 1;
        }
    }

    None
}

/// Computes the longest common subsequence length.
fn lcs_length(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 1..=m {                                // +1
        for j in 1..=n {                            // +2 (nesting)
            if a_chars[i - 1] == b_chars[j - 1] {  // +3 (nesting)
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                if dp[i - 1][j] >= dp[i][j - 1] {  // +4 (nesting)
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i][j - 1];
                }
            }
        }
    }

    dp[m][n]
}

/// Encodes a string using run-length encoding.
fn run_length_encode(input: &str) -> String {
    if input.is_empty() {                           // +1
        return String::new();
    }

    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut count = 1;
    let mut current = chars[0];

    for i in 1..chars.len() {                       // +1
        if chars[i] == current {                    // +2 (nesting)
            count += 1;
        } else {
            if count > 1 {                          // +2 (nesting)
                result.push_str(&count.to_string());
            }
            result.push(current);
            current = chars[i];
            count = 1;
        }
    }

    if count > 1 {                                  // +1
        result.push_str(&count.to_string());
    }
    result.push(current);

    result
}

/// Validates an email address with basic rules.
fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();

    if parts.len() != 2 {                           // +1
        return false;
    }

    let local = parts[0];
    let domain = parts[1];

    if local.is_empty() || domain.is_empty() {      // +1 +1 (||)
        return false;
    }

    if local.len() > 64 || domain.len() > 255 {    // +1 +1 (||)
        return false;
    }

    if !domain.contains('.') {                      // +1
        return false;
    }

    let domain_parts: Vec<&str> = domain.split('.').collect();
    for part in &domain_parts {                     // +1
        if part.is_empty() {                        // +2 (nesting)
            return false;
        }
        for ch in part.chars() {                    // +2 (nesting)
            if !ch.is_alphanumeric() && ch != '-' { // +3 (nesting) +1 (&&)
                return false;
            }
        }
    }

    true
}

// Ensure the file compiles by having a main function.
fn main() {
    let _ = add(1, 2);
    let _ = greeting();
    let _ = wrap_option(42);
    let _ = square(3.14);
    let _ = identity("test");
    let _ = slice_len(&[1, 2, 3]);
    let _ = empty_map();
    let _ = absolute(-5);
    let _ = clamp(10, 0, 100);
    let _ = is_even(4);
    let _ = max_of_three(1, 2, 3);
    let _ = first_positive(&[-1, 0, 1]);
    let _ = temperature_category(22.0);
    let _ = count_vowels("hello");
    let _ = sum_positive_evens(&[1, 2, 3, 4]);
    let _ = fizzbuzz(15);
    let _ = fizzbuzz_range(1, 20);
    let _ = describe_status(200);
    let _ = filter_and_double(&[1, -2, 3]);
    let _ = flatten_matrix(&[vec![1, 2], vec![3, 4]]);
    let _ = process_user_records(
        &[("Alice".to_string(), 30, true)],
        18,
        true,
    );
    let _ = analyze_text("hello world\nfoo bar");
    let _ = validate_password("P@ssw0rd!");
    let mut data = vec![3, 1, 4, 1, 5];
    let _ = bubble_sort_counted(&mut data);
    let _ = eval_postfix(&["3", "4", "+", "2", "*"]);
    let _ = parse_csv_line("hello,\"world\",\"foo,bar\"");
    let _ = merge_sorted(&[1, 3, 5], &[2, 4, 6]);
    let _ = group_and_filter(&[("alpha", 10), ("beta", 5)], 6);
    let _ = transform_with(&[1, 2, 3, 4, 5], |x| {
        if x % 2 == 0 { Some(x * 10) } else { None }
    });
    let _ = apply_pipeline(&[1.0, 2.0, 3.0]);
    let _ = find_matching_items(&[], "", 0, false);
    let _ = find_regions(&[vec![1.0, 2.0], vec![3.0, 4.0]], 2.5, 1);
    let _ = parse_version("1.2.3-beta+build.42");
    let m = Matrix::zeros(2, 3);
    let _ = m.transpose();
    let _ = m.max_element();
    let tokens: Vec<String> = vec!["help".to_string()];
    let _ = parse_command(&tokens);
    let _ = execute_command(&Command::Help, false);
    let _ = binary_search(&[1, 2, 3, 4, 5], 3);
    let _ = lcs_length("abcde", "ace");
    let _ = run_length_encode("aaabbc");
    let _ = is_valid_email("user@example.com");
}
