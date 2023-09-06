use clap::Parser;

fn main() {
    let args = Args::parse();
}

#[derive(Parser, Debug)]
struct Args {
    /// Set the Game Boy Color–only flag (0x143) to 0xC0. This overrides `-c` if it was set prior.
    #[arg(short = 'C', long)]
    color_only: bool,

    /// Set the Game Boy Color–compatible flag: (0x143) to 0x80. This overrides `-c` if it was set prior.
    #[arg(short = 'c', long)]
    color_compatible: bool,

    /// Fix certain header values that the Game Boy checks for correctness. Alternatively, intentionally trash these values by writing their binary inverse instead. fix_spec is a string containing any combination of the following characters:
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
    #[arg(short, long)]
    fix_spec: Option<String>,

    /// Set the game ID string (0x13F–0x142) to a given string. If it's longer than 4 chars, it will be truncated, and a warning emitted.
    #[arg(short = 'i', long)]
    game_id: Option<String>,

    /// Set the non-Japanese region flag (0x14A) to 0x01.
    #[arg(short = 'j', long)]
    non_japanese: bool,

    /// Set the new licensee string (0x144–0x145) to a given string. If it's longer than 2 chars, it will be truncated, and a warning emitted.
    #[arg(short = 'k', long)]
    new_licensee: Option<String>,

    /// Set the old licensee code (0x14B) to a given value from 0 to 0xFF. This value is deprecated and should be set to 0x33 in all new software.
    #[arg(short = 'l', long)]
    old_licensee: Option<String>,

    /// Set the MBC type (0x147) to a given value from 0 to 0xFF.
    ///
    /// This value may also be an MBC name. The list of accepted names can be obtained by passing ‘help’ as the argument. Any amount of whitespace (space and tabs) is allowed around plus signs, and the order of "components" is free, as long as the MBC name is first. There are special considerations to take for the TPP1 mapper; see the TPP1 section below.
    #[arg(short = 'm', long)]
    mbc_type: Option<String>,

    /// Set the ROM version (0x14C) to a given value from 0 to 0xFF.
    #[arg(short = 'n', long)]
    rom_version: Option<String>,

    /// Allow overwriting different non-zero bytes in the header without a warning being emitted.
    #[arg(short = 'O', long)]
    overwrite: bool,

    /// Pad the ROM image to a valid size with a given pad value from 0 to 255 (0xFF). rgbfix will automatically pick a size from 32 KiB, 64 KiB, 128 KiB, ..., 8192 KiB. The cartridge size byte (0x148) will be changed to reflect this new size. The recommended padding value is 0xFF, to speed up writing the ROM to flash chips, and to avoid "nop slides" into VRAM.
    #[arg(short, long)]
    pad_value: Option<u8>,

    /// Set the RAM size (0x149) to a given value from 0 to 0xFF.
    #[arg(short, long)]
    ram_size: Option<u8>,

    /// Set the SGB flag (0x146) to 0x03. This flag will be ignored by the SGB unless the old licensee code is 0x33! If this is given as well as -l, but is not set to 0x33, a warning will be printed.
    #[arg(short, long)]
    sgb_compatible: bool,

    /// Set the title string (0x134–0x143) to a given string. If the title is longer than the max length, it will be truncated, and a warning emitted. The max length is 11 characters if the game ID (-i) is specified, 15 characters if the CGB flag (-c or -C) is specified but the game ID is not, and 16 characters otherwise.
    #[arg(short, long)]
    title: String,

    /// Print the version of the program and exit.
    #[arg(short = 'V', long)]
    version: bool,

    /// Equivalent to -f lhg.
    #[arg(short, long)]
    validate: bool,
}
