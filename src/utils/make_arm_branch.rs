// Thanks to Luma - https://github.com/LumaTeam/Luma3DS/blob/3afecb064c03c26776e21aa54e30ec13e6674787/sysmodules/rosalina/include/utils.h#L44-L50
pub fn make_arm_branch(src: u32, dst: u32) -> u32 {
    let offset = dst - (src + 8);
    0xeb000000 | (offset >> 2) & 0xffffff
}
