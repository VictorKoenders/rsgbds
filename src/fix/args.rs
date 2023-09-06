use clap::Parser;
use std::str::FromStr;

pub fn parse() -> Args {
    let args = Args::parse();
    // TODO: validate these arguments and make adjustements as needed
    // See the main function at <https://github.com/gbdev/rgbds/blob/master/src/fix/main.c#L1220> for validation rules
    args
}

#[derive(Parser, Debug)]
pub struct Args {
    /// Set the Game Boy Color–only flag (0x143) to 0xC0.
    /// This overrides `-c` if it was set prior.
    #[arg(short = 'C', long)]
    pub color_only: bool,

    /// Set the Game Boy Color–compatible flag: (0x143) to 0x80.
    /// This overrides `-c` if it was set prior.
    #[arg(short = 'c', long)]
    pub color_compatible: bool,

    /// Fix certain header values that the Game Boy checks for correctness.
    /// Alternatively, intentionally trash these values by writing their binary inverse instead.
    /// fix_spec is a string containing any combination of the following characters:
    ///
    /// `l`: Fix the Nintendo logo (0x104–0x133).
    ///
    /// `L`: Trash the Nintendo logo.
    ///
    /// `h`: Fix the header checksum (0x14D).
    ///
    /// `H`: Trash the header checksum.
    ///
    /// `g`: Fix the global checksum (0x14E–0x14F).
    ///
    /// `G`: Trash the global checksum.
    #[arg(short, long, value_parser = FixSpec::from_str)]
    pub fix_spec: Option<FixSpec>,

    /// Set the game ID string (0x13F–0x142) to a given string.
    /// If it's longer than 4 chars, it will be truncated, and a warning emitted.
    #[arg(short = 'i', long)]
    pub game_id: Option<String>,

    /// Set the non-Japanese region flag (0x14A) to 0x01.
    #[arg(short = 'j', long)]
    pub non_japanese: bool,

    /// Set the new licensee string (0x144–0x145) to a given string.
    /// If it's longer than 2 chars, it will be truncated, and a warning emitted.
    #[arg(short = 'k', long)]
    pub new_licensee: Option<String>,

    /// Set the old licensee code (0x14B) to a given value from 0 to 0xFF.
    /// This value is deprecated and should be set to 0x33 in all new software.
    #[arg(short = 'l', long, value_parser = parse_u8)]
    pub old_licensee: Option<u8>,

    /// Set the MBC type (0x147) to a given value from 0 to 0xFF.
    ///
    /// This value may also be an MBC name.
    /// The list of accepted names can be obtained by passing ‘help’ as the argument.
    /// Any amount of whitespace (space and tabs) is allowed around plus signs, and the order of "components" is free, as long as the MBC name is first.
    /// There are special considerations to take for the TPP1 mapper; see the TPP1 section below.
    #[arg(short = 'm', long, value_parser = MBC::from_str)]
    pub mbc_type: Option<MBC>,

    /// Set the ROM version (0x14C) to a given value from 0 to 0xFF.
    #[arg(short = 'n', long, value_parser = parse_u8)]
    pub rom_version: Option<u8>,

    /// Allow overwriting different non-zero bytes in the header without a warning being emitted.
    #[arg(short = 'O', long)]
    pub overwrite: bool,

    /// Pad the ROM image to a valid size with a given pad value from 0 to 255 (0xFF).
    /// rgbfix will automatically pick a size from 32 KiB, 64 KiB, 128 KiB, ..., 8192 KiB.
    /// The cartridge size byte (0x148) will be changed to reflect this new size.
    /// The recommended padding value is 0xFF, to speed up writing the ROM to flash chips, and to avoid "nop slides" into VRAM.
    #[arg(short, long, value_parser = parse_u8)]
    pub pad_value: Option<u8>,

    /// Set the RAM size (0x149) to a given value from 0 to 0xFF.
    #[arg(short, long, value_parser = parse_u8)]
    pub ram_size: Option<u8>,

    /// Set the SGB flag (0x146) to 0x03.
    /// This flag will be ignored by the SGB unless the old licensee code is 0x33!
    /// If this is given as well as -l, but is not set to 0x33, a warning will be printed.
    #[arg(short, long)]
    pub sgb_compatible: bool,

    /// Set the title string (0x134–0x143) to a given string.
    /// If the title is longer than the max length, it will be truncated, and a warning emitted.
    /// The max length is 11 characters if the game ID (-i) is specified, 15 characters if the CGB flag (-c or -C) is specified but the game ID is not, and 16 characters otherwise.
    #[arg(short, long)]
    pub title: Option<String>,

    /// Print the version of the program and exit.
    #[arg(short = 'V', long)]
    pub version: bool,

    /// Equivalent to `-f/--fix-spec lhg`.
    /// Will fix the Nintendo logo, fix the header checksum and fix the global checksum.
    #[arg(short, long)]
    pub validate: bool,

    /// The file to be parsed. Set this to `-` to parse from STDIN and output to STDOUT.
    #[arg()]
    pub filename: String,
}

bitflags::bitflags! {
    #[derive(Debug, Clone)]
    pub struct FixSpec : u8 {
        const FIX_LOGO = 0x80;
        const TRASH_LOGO = 0x40;
        const FIX_HEADER_SUM = 0x20;
        const TRASH_HEADER_SUM = 0x10;
        const FIX_GLOBAL_SUM = 0x08;
        const TRASH_GLOBAL_SUM = 0x04;
    }
}

impl FromStr for FixSpec {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = FixSpec::empty();
        for c in s.chars() {
            macro_rules! impl_fixspec_char {
                ($result:expr => [$([$good_char:tt => $good_variant:ident, $bad_char:tt => $bad_variant:ident]),* $(,)?]) => {
                    match c {
                        $(
                            $good_char => {
                                if result.contains(Self::$bad_variant) {
                                    eprintln!("Warning: {} ('{}') overwrites {} ('{}')", stringify!($good_variant), $good_char, stringify!($bad_variant), $bad_char);
                                    result.remove(Self::$bad_variant);
                                }
                                result |= Self::$good_variant;
                            }
                            $bad_char => {
                                if result.contains(Self::$good_variant) {
                                    eprintln!("Warning: {} ('{}') overwrites {} ('{}')", stringify!($bad_variant), $bad_char, stringify!($good_variant), $good_char);
                                    result.remove(Self::$good_variant);
                                }
                                result |= Self::$bad_variant;
                            }
                        )*
                        _ => {
                            eprintln!("Warning: unknown fixspec char '{c}'");
                        }
                    }

                }
            }
            impl_fixspec_char! {
                result => [
                    ['l' => FIX_LOGO, 'L' => TRASH_LOGO],
                    ['h' => FIX_HEADER_SUM, 'H' => TRASH_HEADER_SUM],
                    ['g' => FIX_GLOBAL_SUM, 'G' => TRASH_GLOBAL_SUM],
                ]
            }
        }
        Ok(result)
    }
}

#[derive(Debug, Clone)]
struct AsciiChar<const N: usize>([u8; N]);

impl<const N: usize> FromStr for AsciiChar<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_ascii() && s.len() <= N {
            let mut bytes = [0u8; N];
            bytes[..s.len()].copy_from_slice(s.as_bytes());
            Ok(Self(bytes))
        } else {
            Err(format!(
                "Invalid string, can only be ascii and up to {N} characters"
            ))
        }
    }
}

fn parse_u8(input: &str) -> Result<u8, String> {
    let (str, radix) = if let Some(str) = input
        .strip_prefix("0x")
        .or_else(|| input.strip_prefix("0X"))
    {
        (str, 16)
    } else if let Some(str) = input
        .strip_prefix("0b")
        .or_else(|| input.strip_prefix("0B"))
    {
        (str, 2)
    } else {
        (input, 10)
    };

    u8::from_str_radix(str, radix).map_err(|e| e.to_string())
}

#[derive(Debug, Clone)]
pub struct MBC {
    pub ty: MBCType,
    pub extensions: MBCExtension,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types, unused)]
pub enum MBCType {
    ROM = 0x00,
    MBC1 = 0x01,
    MBC2 = 0x05,
    MMM01 = 0x0B,
    MBC3 = 0x11,
    MBC5 = 0x19,
    MBC6 = 0x20,
    MBC7 = 0x22,
    POCKET_CAMERA = 0xFC,
    BANDAI_TAMA5 = 0xFD,
    HUC1 = 0xFF,
    HUC3 = 0xFE,
    TPP1 = 0x100,
}

impl MBCType {
    fn can_have_extension(&self, extension: MBCExtension) -> bool {
        self.valid_extensions().contains(extension)
    }

    fn valid_extensions(&self) -> MBCExtension {
        match self {
            MBCType::ROM | MBCType::MBC1 | MBCType::MMM01 => {
                MBCExtension::RAM | MBCExtension::BATTERY
            }
            MBCType::MBC2 => MBCExtension::BATTERY,
            MBCType::MBC3 => MBCExtension::RAM | MBCExtension::BATTERY | MBCExtension::TIMER,
            MBCType::MBC5 => MBCExtension::RAM | MBCExtension::BATTERY | MBCExtension::RUMBLE,
            MBCType::MBC6 | MBCType::POCKET_CAMERA | MBCType::BANDAI_TAMA5 | MBCType::HUC3 => {
                MBCExtension::empty()
            }
            MBCType::MBC7 => {
                MBCExtension::RAM
                    | MBCExtension::BATTERY
                    | MBCExtension::SENSOR
                    | MBCExtension::RUMBLE
            }
            MBCType::HUC1 => MBCExtension::RAM | MBCExtension::BATTERY,
            MBCType::TPP1 => {
                MBCExtension::RAM
                    | MBCExtension::BATTERY
                    | MBCExtension::MULTIRUMBLE
                    | MBCExtension::RUMBLE
            }
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone)]
    pub struct MBCExtension: u8 {
        const RAM = 0x80;
        const BATTERY = 0x40;
        const TIMER = 0x20;
        const RUMBLE = 0x10;
        const SENSOR = 0x08;
        const MULTIRUMBLE = 0x04;
    }
}

impl MBC {
    fn print_help() {
        eprintln!("Accepted MBC types:");
        eprintln!("\tROM ($00) [aka ROM_ONLY]");
        eprintln!("\tMBC1 ($01), MBC1+RAM ($02), MBC1+RAM+BATTERY ($03)");
        eprintln!("\tMBC2 ($05), MBC2+BATTERY ($06)");
        eprintln!("\tROM+RAM ($08) [deprecated], ROM+RAM+BATTERY ($09) [deprecated]");
        eprintln!("\tMMM01 ($0B), MMM01+RAM ($0C), MMM01+RAM+BATTERY ($0D)");
        eprintln!("\tMBC3+TIMER+BATTERY ($0F), MBC3+TIMER+RAM+BATTERY ($10)");
        eprintln!("\tMBC3 ($11), MBC3+RAM ($12), MBC3+RAM+BATTERY ($13)");
        eprintln!("\tMBC5 ($19), MBC5+RAM ($1A), MBC5+RAM+BATTERY ($1B)");
        eprintln!("\tMBC5+RUMBLE ($1C), MBC5+RUMBLE+RAM ($1D), MBC5+RUMBLE+RAM+BATTERY ($1E)");
        eprintln!("\tMBC6 ($20)");
        eprintln!("\tMBC7+SENSOR+RUMBLE+RAM+BATTERY ($22)");
        eprintln!("\tPOCKET_CAMERA ($FC)");
        eprintln!("\tBANDAI_TAMA5 ($FD)");
        eprintln!("\tHUC3 ($FE)");
        eprintln!("\tHUC1+RAM+BATTERY ($FF)");
        eprintln!();
        eprintln!("\tTPP1_1.0, TPP1_1.0+RUMBLE, TPP1_1.0+MULTIRUMBLE, TPP1_1.0+TIMER,");
        eprintln!("\tTPP1_1.0+TIMER+RUMBLE, TPP1_1.0+TIMER+MULTIRUMBLE, TPP1_1.0+BATTERY,");
        eprintln!("\tTPP1_1.0+BATTERY+RUMBLE, TPP1_1.0+BATTERY+MULTIRUMBLE,");
        eprintln!("\tTPP1_1.0+BATTERY+TIMER, TPP1_1.0+BATTERY+TIMER+RUMBLE,");
        eprintln!("\tTPP1_1.0+BATTERY+TIMER+MULTIRUMBLE");
    }
}

impl FromStr for MBC {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("help") {
            MBC::print_help();
            std::process::exit(0);
        }
        macro_rules! match_mbc_ty {
            ($n:ident { $([$($name:expr),* $(,)?] => $value:ident),* $(,)? }) => {
                $(
                    $(
                        if $n.get(..$name.len()).map_or(false, |n| n.eq_ignore_ascii_case($name)) {
                            (MBCType::$value, &$n[$name.len()..])
                        } else
                    )*
                )*
                {
                    return Err(String::from("Invalid MBC type"))
                }
            }
        }
        let (ty, remaining) = match_mbc_ty! {
            s {
                [ "ROM_ONLY", "ROM" ] => ROM,
                [ "MMM01" ] => MMM01,
                [ "MBC1" ] => MBC1,
                [ "MBC2" ] => MBC2,
                [ "MBC3" ] => MBC3,
                [ "MBC5" ] => MBC5,
                [ "MBC6" ] => MBC6,
                [ "POCKET_CAMERA" ] => POCKET_CAMERA,
                [ "BANDAI_TAMA5" ] => BANDAI_TAMA5,
                [ "HUC1" ] => HUC1,
                [ "HUC3" ] => HUC3,
                [ "TPP1" ] => TPP1,
            }
        };

        let mut extensions = MBCExtension::empty();
        for rem in remaining.split(['+', '_', ' ']) {
            let rem = rem.trim();
            if rem.is_empty() {
                continue;
            }

            macro_rules! match_ram {
                ($rem:expr, $ty:expr, $extensions:expr => [$(
                    $e:ident
                ),* $(,)?]) => {
                    if false {}
                    $(
                        else if $rem.eq_ignore_ascii_case(stringify!($e)) {
                            if !$ty.can_have_extension(MBCExtension::$e) {
                                eprintln!("{:?} cannot have {:?}", $ty, stringify!($e));
                                std::process::exit(0);
                            }
                            $extensions |= MBCExtension::$e
                        }
                    )*

                    else {
                        return Err(format!("Invalid extension {:?}", $rem));
                    }
                }
            }
            match_ram! {
                rem, ty, extensions => [
                    RAM,
                    BATTERY,
                    TIMER,
                    RUMBLE,
                    SENSOR,
                    MULTIRUMBLE,
                ]
            }
        }

        Ok(Self { ty, extensions })
    }
}
