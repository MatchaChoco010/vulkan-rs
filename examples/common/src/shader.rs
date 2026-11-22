use anyhow::{Result, bail};
use std::collections::BTreeSet;
use vulkan::{Device, vk};

/// SPIR-V as the 32-bit words Vulkan takes. `include_bytes!` gives no alignment
/// guarantee, so the bytes are copied rather than reinterpreted.
pub fn spirv_words(bytes: &[u8]) -> Result<Vec<u32>> {
    if !bytes.len().is_multiple_of(4) {
        bail!("SPIR-V size is not a multiple of four bytes");
    }
    Ok(bytes
        .chunks_exact(4)
        .map(|word| u32::from_le_bytes([word[0], word[1], word[2], word[3]]))
        .collect())
}

/// The SPIR-V capability `Shader`, which every Vulkan device supports.
pub const CAPABILITY_SHADER: u32 = 1;
/// The SPIR-V capability `DrawParameters`, which needs the device feature
/// `shaderDrawParameters`.
pub const CAPABILITY_DRAW_PARAMETERS: u32 = 4427;

const SPIRV_MAGIC: u32 = 0x0723_0203;
const SPIRV_HEADER_WORDS: usize = 5;
const OP_CAPABILITY: u32 = 17;

/// The capabilities that a SPIR-V module declares with `OpCapability`.
pub fn spirv_capabilities(bytes: &[u8]) -> Result<BTreeSet<u32>> {
    let words = spirv_words(bytes)?;
    if words.len() < SPIRV_HEADER_WORDS || words[0] != SPIRV_MAGIC {
        bail!("not a little-endian SPIR-V module");
    }
    let mut capabilities = BTreeSet::new();
    let mut rest = &words[SPIRV_HEADER_WORDS..];
    while let Some(&first) = rest.first() {
        let word_count = (first >> 16) as usize;
        if word_count == 0 || word_count > rest.len() {
            bail!(
                "malformed SPIR-V instruction at word {}",
                words.len() - rest.len()
            );
        }
        if first & 0xffff == OP_CAPABILITY {
            if word_count != 2 {
                bail!("OpCapability with {word_count} words");
            }
            capabilities.insert(rest[1]);
        }
        rest = &rest[word_count..];
    }
    Ok(capabilities)
}

pub unsafe fn create_shader_module(device: &Device, spirv: &[u8]) -> Result<vk::ShaderModule> {
    let code = spirv_words(spirv)?;
    let info = vk::ShaderModuleCreateInfo::default().code(&code);
    Ok(unsafe { device.create_shader_module(&info, None) }?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const OP_NOP: u32 = 0;
    const OP_SOURCE_EXTENSION: u32 = 4;

    fn instruction(opcode: u32, operands: &[u32]) -> Vec<u32> {
        let mut words = vec![((operands.len() as u32 + 1) << 16) | opcode];
        words.extend_from_slice(operands);
        words
    }

    fn module(instructions: &[Vec<u32>]) -> Vec<u8> {
        [SPIRV_MAGIC, 0x0001_0300, 0, 16, 0]
            .into_iter()
            .chain(instructions.iter().flatten().copied())
            .flat_map(u32::to_le_bytes)
            .collect()
    }

    /// A deterministic sequence of pseudo-random numbers, so that a failure repeats.
    fn numbers(seed: u64) -> impl Iterator<Item = u32> {
        let mut state = seed;
        std::iter::repeat_with(move || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 33) as u32
        })
    }

    #[test]
    fn finds_every_declared_capability_among_other_instructions() {
        for seed in 0..200 {
            let mut random = numbers(seed);
            let mut next = |bound: u32| random.next().unwrap() % bound;
            let mut declared = BTreeSet::new();
            let mut instructions = Vec::new();
            for _ in 0..next(12) {
                match next(3) {
                    0 => {
                        let capability = next(6000);
                        declared.insert(capability);
                        instructions.push(instruction(OP_CAPABILITY, &[capability]));
                    }
                    1 => {
                        let operands = (0..next(5)).map(|_| OP_CAPABILITY).collect::<Vec<_>>();
                        instructions.push(instruction(OP_SOURCE_EXTENSION, &operands));
                    }
                    _ => instructions.push(instruction(OP_NOP, &[])),
                }
            }
            assert_eq!(
                spirv_capabilities(&module(&instructions)).unwrap(),
                declared,
                "seed {seed}"
            );
        }
    }

    #[test]
    fn rejects_what_is_not_a_well_formed_module() {
        let capability = instruction(OP_CAPABILITY, &[CAPABILITY_SHADER]);
        let well_formed = module(std::slice::from_ref(&capability));
        assert!(spirv_capabilities(&well_formed).is_ok());

        let mut big_endian = well_formed.clone();
        big_endian[..4].copy_from_slice(&SPIRV_MAGIC.to_be_bytes());
        assert!(
            spirv_capabilities(&big_endian).is_err(),
            "byte-swapped magic"
        );
        assert!(
            spirv_capabilities(&well_formed[..16]).is_err(),
            "short header"
        );
        assert!(
            spirv_capabilities(&well_formed[..well_formed.len() - 1]).is_err(),
            "size not a multiple of four"
        );
        let truncated = module(&[vec![(3 << 16) | OP_CAPABILITY, CAPABILITY_SHADER]]);
        assert!(
            spirv_capabilities(&truncated).is_err(),
            "instruction past the end"
        );
        let empty = module(&[vec![OP_CAPABILITY]]);
        assert!(spirv_capabilities(&empty).is_err(), "zero word count");
        let long = module(&[instruction(OP_CAPABILITY, &[CAPABILITY_SHADER, 0])]);
        assert!(
            spirv_capabilities(&long).is_err(),
            "OpCapability with an extra operand"
        );
    }
}
