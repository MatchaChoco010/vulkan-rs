use std::path::Path;
use std::process::Command;

use serde_json::Value;

pub const CAPTURE_LAYER: &str = "VK_LAYER_LUNARG_gfxreconstruct";

fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn is_pointer_value(key: &str) -> bool {
    key.starts_with("pfn") || key == "pUserData" || key == "ppData"
}

const MAX_ALIGNMENT_OFFSET: u64 = 256;

const ADDRESS_QUERIES: &[&str] = &[
    "vkGetBufferDeviceAddress",
    "vkGetBufferDeviceAddressKHR",
    "vkGetBufferDeviceAddressEXT",
    "vkGetAccelerationStructureDeviceAddressKHR",
];

fn as_address(value: &Value) -> Option<u64> {
    match value {
        Value::Number(number) => number.as_u64(),
        Value::String(text) => u64::from_str_radix(text.strip_prefix("0x")?, 16).ok(),
        _ => None,
    }
}

fn queried_address(entry: &Value) -> Option<u64> {
    let function = entry.get("function")?;
    let name = function.get("name")?.as_str()?;
    let address = as_address(function.get("return")?)?;
    (ADDRESS_QUERIES.contains(&name) && address != 0).then_some(address)
}

#[derive(Default)]
struct Addresses {
    queries: usize,
    latest: Vec<(u64, usize)>,
}

impl Addresses {
    fn query(&mut self, address: u64) {
        self.latest.retain(|&(known, _)| known != address);
        self.latest.push((address, self.queries));
        self.queries += 1;
    }

    fn query_of(&self, value: u64) -> Option<usize> {
        self.latest
            .iter()
            .find(|&&(known, _)| known == value)
            .map(|&(_, query)| query)
    }

    fn query_and_offset_of(&self, value: u64) -> Option<(usize, u64)> {
        self.latest
            .iter()
            .filter(|&&(known, _)| known <= value && value - known < MAX_ALIGNMENT_OFFSET)
            .max_by_key(|&&(known, _)| known)
            .map(|&(known, query)| (query, value - known))
    }

    fn remove_from(&self, bytes: &mut [u8]) {
        for word in bytes.chunks_exact_mut(8) {
            let value = u64::from_le_bytes((&*word).try_into().unwrap());
            if let Some(query) = self.query_of(value) {
                word.copy_from_slice(&(0xdead_0000_0000_0000_u64 | query as u64).to_le_bytes());
            }
        }
    }
}

fn normalize(value: Value, key: Option<&str>, binaries: &Path, addresses: &Addresses) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(key, value)| {
                    let value = normalize(value, Some(&key), binaries, addresses);
                    (key, value)
                })
                .collect(),
        ),
        Value::Array(values) => Value::Array(
            values
                .into_iter()
                .map(|value| normalize(value, key, binaries, addresses))
                .collect(),
        ),
        Value::String(_) if key.is_some_and(is_pointer_value) => {
            Value::String("<pointer>".to_string())
        }
        Value::String(text) if text.ends_with(".bin") && binaries.join(&text).is_file() => {
            let mut bytes = std::fs::read(binaries.join(&text)).expect("binary of the trace");
            addresses.remove_from(&mut bytes);
            Value::String(format!(
                "<{} bytes, fnv1a {:016x}>",
                bytes.len(),
                fnv1a(&bytes)
            ))
        }
        value => match as_address(&value).and_then(|found| addresses.query_and_offset_of(found)) {
            Some((query, 0)) => Value::String(format!("<device address of query {query}>")),
            Some((query, offset)) => {
                Value::String(format!("<device address of query {query} + {offset}>"))
            }
            None => value,
        },
    }
}

pub fn convert_available() -> Result<(), String> {
    match Command::new("gfxrecon-convert").arg("--version").output() {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => Err(format!(
            "gfxrecon-convert --version failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )),
        Err(error) => Err(format!("cannot run gfxrecon-convert: {error}")),
    }
}

pub fn convert(capture: &Path, directory: &Path) -> Vec<String> {
    let json = directory.join("trace.jsonl");
    let output = Command::new("gfxrecon-convert")
        .arg("--format")
        .arg("jsonl")
        .arg("--include-binaries")
        .arg("--output")
        .arg(&json)
        .arg(capture)
        .output()
        .expect("gfxrecon-convert runs");
    assert!(
        output.status.success(),
        "gfxrecon-convert failed on {}:\n{}",
        capture.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    let text = std::fs::read_to_string(&json)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", json.display()));
    let mut entries = Vec::new();
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let mut value: Value = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("{}: not JSON ({e}): {line}", json.display()));
        let object = value.as_object_mut().expect("a JSON object per line");
        if object.contains_key("header") || object.contains_key("annotation") {
            continue;
        }
        if object
            .get("meta")
            .and_then(|meta| meta.get("name"))
            .is_some_and(|name| name == "ExeFileInfo")
        {
            continue;
        }
        object.remove("index");
        entries.push(value);
    }
    let mut addresses = Addresses::default();
    entries
        .into_iter()
        .map(|entry| {
            if let Some(address) = queried_address(&entry) {
                addresses.query(address);
            }
            normalize(entry, None, directory, &addresses).to_string()
        })
        .collect()
}

pub fn calls(trace: &[String]) -> Vec<String> {
    trace
        .iter()
        .filter_map(|line| {
            let value: Value = serde_json::from_str(line).ok()?;
            Some(value.get("function")?.get("name")?.as_str()?.to_string())
        })
        .collect()
}

pub fn first_difference(c: &[String], rust: &[String]) -> Option<String> {
    let index = c
        .iter()
        .zip(rust)
        .position(|(a, b)| a != b)
        .or_else(|| (c.len() != rust.len()).then(|| c.len().min(rust.len())))?;
    let show = |lines: &[String]| {
        lines
            .iter()
            .enumerate()
            .skip(index.saturating_sub(2))
            .take(5)
            .map(|(at, line)| format!("{at:5} {line}"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    Some(format!(
        "the traces differ at entry {index} (C has {} entries, Rust {}):\n--- C\n{}\n--- Rust\n{}",
        c.len(),
        rust.len(),
        show(c),
        show(rust)
    ))
}
